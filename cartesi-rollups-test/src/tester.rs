use std::env;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq)]
pub struct Notice {
    epoch_index: usize,
    input_index: usize,
    notice_index: usize,
    payload: String,
}

impl Notice {
    pub fn new(payload: impl Into<String>) -> Self {
        Self {
            epoch_index: 0,
            input_index: 0,
            notice_index: 0,
            payload: payload.into(),
        }
    }

    pub fn with_epoch_index(mut self, epoch_index: usize) -> Self {
        self.epoch_index = epoch_index;
        self
    }

    pub fn with_input_index(mut self, input_index: usize) -> Self {
        self.input_index = input_index;
        self
    }

    pub fn with_notice_index(mut self, notice_index: usize) -> Self {
        self.notice_index = notice_index;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    report_index: usize,
    payload: String,
}

impl Report {
    pub fn new(payload: impl Into<String>) -> Self {
        Self {
            report_index: 0,
            payload: payload.into(),
        }
    }

    pub fn with_report_index(mut self, report_index: usize) -> Self {
        self.report_index = report_index;
        self
    }
}

#[derive(Debug)]
pub struct TestMachineIo {
    /// One-based index of an input of an epoch.
    input_index: usize,
    /// Zero-based index of an epoch.
    epoch_index: usize,
    /// Port of the remote cartesi machine.
    port: u16,
    /// Name of the dapp binary to run on the cartesi machine.
    bin_name: String,
    /// Directory where cartesi dependencies and build output are placed.
    target_dir: String,
}

impl Default for TestMachineIo {
    fn default() -> Self {
        Self {
            port: 8080,
            input_index: 0,
            epoch_index: 0,
            bin_name: env::var("CARGO_PKG_NAME").expect("Cannot read bin_name from CARGO_PKG_NAME"),
            target_dir: format!("{}/cartesi", Self::target_dir().unwrap()),
        }
    }
}

impl TestMachineIo {
    pub fn write_input(mut self, payload: impl AsRef<str>) -> Self {
        self.input_index += 1;
        RequestWriter::write_input_metadata("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef", 0, self.input_index, 0, 0);
        RequestWriter::write_input_payload(payload, 0, self.input_index);
        self
    }

    pub fn write_query(self, payload: impl AsRef<str>) -> Self {
        RequestWriter::write_query_payload(payload);
        self
    }

    pub fn process(self) -> (Vec<Notice>, Vec<Report>) {
        Command::new("/opt/cartesi/bin/remote-cartesi-machine")
            .arg(format!("--server-address=localhost:{}", self.port))
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();

        Command::new("/opt/cartesi/bin/cartesi-machine")
            .arg(format!("--remote-address=localhost:{}", self.port))
            .arg("--checkin-address=localhost:8081")
            .arg("--remote-shutdown")
            .arg("--rollup")
            .arg(format!(
                "--rollup-advance-state=epoch_index:0,input_index_begin:1,input_index_end:{}",
                self.input_index + 1
            ))
            .arg("--rollup-inspect-state=query:query.bin")
            .arg("--ram-length=128Mi")
            .arg(format!(
                "--flash-drive=label:dapp,filename:{}/dapp.ext2",
                self.target_dir
            ))
            .arg(format!(
                "--flash-drive=label:root,filename:{}/rootfs.ext2",
                self.target_dir
            ))
            .arg(format!("--ram-image={}/linux-5.5.19-ctsi-6.bin", self.target_dir))
            .arg(format!("--rom-image={}/rom.bin", self.target_dir))
            .arg("--")
            .arg(format!("/mnt/dapp/{}", self.bin_name))
            .output()
            .unwrap();

        let r#type = "report";
        let reports = (0..usize::MAX)
            .map_while(|report_index| {
                let path = format!("query-{}-{}.bin", r#type, report_index);
                Self::decode_from_file(path, r#type).map(|payload| Report { report_index, payload })
            })
            .collect();

        let r#type = "notice";
        let values = (0..=self.epoch_index)
            .map(|epoch_index| {
                (1..=self.input_index)
                    .map(move |input_index| {
                        (0..usize::MAX).map_while(move |notice_index| {
                            let path = format!(
                                "epoch-{}-input-{}-{}-{}.bin",
                                epoch_index, input_index, r#type, notice_index
                            );
                            Self::decode_from_file(path, r#type).map(|payload| Notice {
                                epoch_index,
                                input_index,
                                notice_index,
                                payload,
                            })
                        })
                    })
                    .flatten()
            })
            .flatten()
            .collect();

        (values, reports)
    }

    fn decode_from_file(path: impl AsRef<Path>, r#type: impl AsRef<str>) -> Option<String> {
        File::open(path)
            .map(|file| {
                let output = Command::new("/opt/cartesi/bin/rollup-memory-range")
                    .arg("decode")
                    .arg(r#type.as_ref())
                    .stdin(Stdio::from(file))
                    .output()
                    .unwrap();

                let value = serde_json::from_slice::<serde_json::Value>(output.stdout.as_slice()).unwrap();
                value.get("payload").unwrap().as_str().unwrap().to_owned()
            })
            .ok()
    }

    fn target_name() -> &'static str {
        "riscv64ima-cartesi-linux-gnu"
    }

    fn target_dir() -> Result<String, Box<dyn Error>> {
        let mut path = PathBuf::new().join(env::current_dir()?);

        while path.exists() {
            let target = path.join("target");

            if target.exists() {
                path = target;
                break;
            }

            path = path.join("..");
        }
        if !path.exists() {
            return Err(Box::try_from("Target not found.").unwrap());
        }

        Ok(path
            .join(Self::target_name())
            .join("release")
            .canonicalize()?
            .to_str()
            .unwrap()
            .to_owned())
    }
}

impl Drop for TestMachineIo {
    fn drop(&mut self) {
        for epoch_index in 0..=self.epoch_index {
            for input_index in 1..=self.input_index {
                for r#type in ["notice", "voucher"] {
                    for response_index in 0..usize::MAX {
                        if remove_file(format!(
                            "epoch-{}-input-{}-{}-{}.bin",
                            epoch_index, input_index, r#type, response_index
                        ))
                        .is_err()
                        {
                            break;
                        }
                    }
                    let _ = remove_file(format!(
                        "epoch-{}-input-{}-{}-hashes.bin",
                        epoch_index, input_index, r#type
                    ));
                }
                let _ = remove_file(format!("epoch-{}-input-metadata-{}.bin", epoch_index, input_index));
                let _ = remove_file(format!("epoch-{}-input-{}.bin", epoch_index, input_index));
            }
        }
        for response_index in 0..usize::MAX {
            if remove_file(format!("query-report-{}.bin", response_index)).is_err() {
                break;
            }
        }
        let _ = remove_file("query.bin");
    }
}

#[derive(Debug)]
pub struct RequestWriter;

impl RequestWriter {
    pub fn write_input_metadata(
        msg_sender: impl AsRef<str>,
        epoch_index: usize,
        input_index: usize,
        block_number: usize,
        time_stamp: u64,
    ) {
        let input = Command::new("echo")
            .arg(format!(
                "{{
                \"msg_sender\": \"{}\",
                \"epoch_index\": \"{}\",
                \"input_index\": \"{}\"
                \"block_number\": \"{}\"
                \"time_stamp\": \"{}\"
            }}",
                msg_sender.as_ref(),
                epoch_index,
                input_index,
                block_number,
                time_stamp
            ))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let encoder = Command::new("/opt/cartesi/bin/rollup-memory-range")
            .arg("encode")
            .arg("input-metadata")
            .stdin(Stdio::from(input.stdout.unwrap()))
            .output()
            .unwrap();
        let path = format!("epoch-{}-input-metadata-{}.bin", epoch_index, input_index);
        let mut file = File::create(path).unwrap();
        file.write_all(&encoder.stdout).unwrap();
    }

    pub fn write_input_payload(payload: impl AsRef<str>, epoch_index: usize, input_index: usize) {
        let r#type = "input";
        let path = format!("epoch-{}-{}-{}.bin", epoch_index, r#type, input_index);
        Self::write_payload(payload, r#type, path);
    }

    pub fn write_query_payload(payload: impl AsRef<str>) {
        let r#type = "query";
        let path = format!("{}.bin", r#type);
        Self::write_payload(payload, r#type, path);
    }

    fn write_payload(payload: impl AsRef<str>, r#type: impl AsRef<str>, path: impl AsRef<Path>) {
        let input = Command::new("echo")
            .arg(format!("{{\"payload\": \"{}\"}}", payload.as_ref()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let encoder = Command::new("/opt/cartesi/bin/rollup-memory-range")
            .arg("encode")
            .arg(r#type.as_ref())
            .stdin(Stdio::from(input.stdout.unwrap()))
            .output()
            .unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&encoder.stdout).unwrap();
    }
}
