use std::error::Error;
use std::path::PathBuf;

pub trait CartesiMachine {
    fn build(&self, target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>);
    fn run_one_shot(&self, target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>);
}

pub trait Cargo {
    fn create_new_binary_source(&self, target_bin: impl AsRef<str>);
    fn build_binary(&self);
    fn package_name(&self) -> Result<String, Box<dyn Error>>;
    fn target_dir(&self) -> Result<String, Box<dyn Error>>;
}

pub trait FileSystem {
    /// Creates a file-system suitable to mount in Cartesi machine.
    ///
    /// Takes the `files`, creates a file-system image of `size` blocks and stores it in `output`.
    fn create(
        &self,
        files: impl IntoIterator<Item = impl Into<PathBuf>>,
        size: Option<usize>,
        output: impl Into<PathBuf>,
    ) -> Result<(), Box<dyn Error>>;
}

pub trait DependenciesDownloader {
    fn download_if_not_present_and_verify(&self, target_dir: impl AsRef<str>) -> Result<(), Box<dyn Error>>;
}

pub trait ResourceCreator {
    fn create(&self, target_dir: impl AsRef<str>) -> Result<(), Box<dyn Error>>;
}
