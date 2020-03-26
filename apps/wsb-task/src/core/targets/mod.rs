mod linux_x86;
pub use linux_x86::LinuxX86;

mod linux_arm_v7;
pub use linux_arm_v7::LinuxArmV7;

mod mac_x86;
pub use mac_x86::MacX86;

mod require_cc;
pub use require_cc::RequireCC;

use std::fmt::Debug;

#[derive(Debug)]
pub enum TargetArch {
    LinuxX86,
    LinuxArmV7,
    MacX86,
}

impl TargetArch {
    pub fn all() -> Vec<TargetArch> {
        vec![
            TargetArch::LinuxX86,
            TargetArch::LinuxArmV7,
            TargetArch::MacX86,
        ]
    }

    pub fn as_triple(&self) -> &str {
        match self {
            TargetArch::LinuxX86 => "x86_64-unknown-linux-musl",
            TargetArch::LinuxArmV7 => "armv7-unknown-linux-musleabihf",
            TargetArch::MacX86 => "x86_64-apple-darwin",
        }
    }
    pub fn as_abbr(&self) -> &str {
        match self {
            TargetArch::LinuxX86 => "linux_x86",
            TargetArch::LinuxArmV7 => "linux_armv7",
            TargetArch::MacX86 => "macos_x86",
        }
    }
}

pub trait AsTargetArch {
    fn as_target_arch(&self) -> &TargetArch;
}

// todo: deprecate
pub trait BuildTarget: Debug {
    fn as_triple(&self) -> &str;

    fn as_abbr(&self) -> &str;
}

pub fn all() -> Vec<Box<dyn BuildTarget>> {
    vec![Box::new(LinuxX86), Box::new(LinuxArmV7), Box::new(MacX86)]
}
