use crate::core::targets::BuildTarget;

#[derive(Clone, Debug)]
pub struct LinuxX86;

impl BuildTarget for LinuxX86 {
    fn as_triple(&self) -> &str {
        "x86_64-unknown-linux-musl"
    }
    fn as_abbr(&self) -> &str {
        "linux_x86"
    }
}
