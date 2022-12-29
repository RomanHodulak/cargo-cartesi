pub trait CartesiMachine {
    fn build(&self, target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>);
    fn run_one_shot(&self, target_binary: impl AsRef<str>, dapp_fs: impl AsRef<str>);
}
