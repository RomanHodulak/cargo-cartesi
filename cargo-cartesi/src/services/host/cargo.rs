use crate::services::Cargo;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

pub struct HostCargo;

impl Cargo for HostCargo {
    fn create_new_binary_source(&self, target_bin: impl AsRef<str>) {
        Self::cargo()
            .arg("new")
            .arg("--color")
            .arg("always")
            .arg(target_bin.as_ref())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to execute process");
    }

    fn build_binary(&self) {
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

    fn package_name(&self) -> Result<String, Box<dyn Error>> {
        let mut buf = String::new();
        let mut file = File::open("Cargo.toml").unwrap();
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

    fn target_dir(&self) -> Result<String, Box<dyn Error>> {
        Ok(PathBuf::new()
            .join(env::current_dir()?)
            .join("../target")
            .join(Self::target_name())
            .join("release")
            .canonicalize()?
            .to_str()
            .unwrap()
            .to_owned())
    }
}

impl HostCargo {
    fn cargo() -> Command {
        let cargo_path = env::var("CARGO").expect("The `CARGO` environment variable was not set. This is unexpected: it should always be provided by `cargo` when invoking a custom sub-command, allowing `cargo-cartesi` to correctly detect which toolchain should be used. Please file a bug.");

        Command::new(cargo_path)
    }

    fn target_name() -> &'static str {
        "riscv64ima-cartesi-linux-gnu"
    }
}
