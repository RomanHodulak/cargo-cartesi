use crate::services::ResourceCreator;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct HostResourceCreator;

impl ResourceCreator for HostResourceCreator {
    fn create(&self, target_dir: impl AsRef<str>) -> Result<(), Box<dyn Error>> {
        for (contents, name) in Self::RESOURCES.iter() {
            let path = format!("{}/{}", target_dir.as_ref(), name);
            let mut file = File::create(path)?;
            file.write_all(contents)?;
        }

        Ok(())
    }
}

impl HostResourceCreator {
    const RESOURCES: [(&'static [u8], &'static str); 1] = [(
        include_bytes!("../../../res/riscv64ima-cartesi-linux-gnu.json"),
        "riscv64ima-cartesi-linux-gnu.json",
    )];
}
