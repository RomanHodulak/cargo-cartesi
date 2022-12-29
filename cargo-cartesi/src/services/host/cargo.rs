use cargo_metadata::{Message, MetadataCommand};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::process::{self, Stdio};
use std::{env, io};

pub struct HostCargo;

impl HostCargo {
    pub fn create_new_binary_source() {
        let mut command = Self::cargo();
        command.arg("new");

        let output = command.output().expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    pub fn build_binary() {
        Self::cargo()
            .arg("build")
            .arg("-Z")
            .arg("build-std=std,core,alloc,panic_abort,proc_macro")
            .arg("--color")
            .arg("always")
            .arg("--target")
            .arg(format!("{}.json", Self::target_name()))
            .arg("--release")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process");
    }

    fn create_path(binary_name: &str) -> PathBuf {
        let target_name = Self::target_name();

        PathBuf::new()
            .join("target")
            .join(target_name)
            .join("release")
            .join(binary_name)
    }

    fn target_name() -> &'static str {
        "riscv64ima-cartesi-linux-gnu"
    }

    fn cargo() -> Command {
        Command::new(Self::cargo_path())
    }

    fn cargo_path() -> String {
        env::var("CARGO").expect("The `CARGO` environment variable was not set. This is unexpected: it should always be provided by `cargo` when invoking a custom sub-command, allowing `cargo-cartesi` to correctly detect which toolchain should be used. Please file a bug.")
    }

    pub fn package_name() -> Result<String, Box<dyn Error>> {
        let mut buf = String::new();
        let mut file = File::open(PathBuf::new().join(std::env::current_dir().unwrap()).join("Cargo.toml")).unwrap();
        file.read_to_string(&mut buf)?;
        let toml: toml::Value = toml::from_str(&buf)?;
        Ok(toml
            .get("package")
            .unwrap()
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned())
    }

    pub fn target_dir() -> Result<String, Box<dyn Error>> {
        let target_name = Self::target_name();
        let path = PathBuf::new()
            .join(env::current_dir()?)
            .join("../target")
            .join(target_name)
            .join("release")
            .canonicalize()?;

        Ok(path.to_str().unwrap().to_owned())
        // let metadata = MetadataCommand::new().exec()?;
        // let package = match metadata.root_package() {
        //     Some(p) => p,
        //     None => return Err("cargo out-dir must be run from within a crate".into()),
        // };
        // let mut command = Command::new("cargo")
        //     .args(&["check", "--message-format=json", "--quiet"])
        //     .stdout(Stdio::piped())
        //     .stderr(Stdio::null())
        //     .spawn()
        //     .unwrap();
        // let reader = BufReader::new(command.stdout.take().unwrap());
        // for message in Message::parse_stream(reader) {
        //     match message? {
        //         Message::BuildScriptExecuted(script) => {
        //             if script.package_id == package.id {
        //                 println!("{:?}", script.out_dir);
        //                 return Ok(script.out_dir.into_string());
        //             }
        //         },
        //         _ => ()
        //     }
        // }
        // Err(format!("crate {} did not run a build script", package.name).into())
    }
}
