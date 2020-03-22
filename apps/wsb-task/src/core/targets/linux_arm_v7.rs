use crate::core::targets::BuildTarget;

#[derive(Clone, Debug)]
pub struct LinuxArmV7;

impl BuildTarget for LinuxArmV7 {
    fn as_triple(&self) -> &str {
        "armv7-unknown-linux-musleabihf"
    }
    fn as_abbr(&self) -> &str {
        "linux_armv7"
    }
}
