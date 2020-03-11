use crate::core::targets::BuildTarget;

pub struct LinuxX86;

impl BuildTarget for LinuxX86 {
    fn as_triple(&self) -> &str {
        "x86_64-unknown-linux-musl"
    }
}
