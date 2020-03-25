use crate::core::targets::{LinuxArmV7, MacX86};

// todo: deprecate
pub trait RequireCC {
    const CC: &'static str;
}

impl RequireCC for LinuxArmV7 {
    const CC: &'static str = "arm-linux-gnueabihf-gcc";
}

impl RequireCC for MacX86 {
    const CC: &'static str = "x86_64-apple-darwin19-clang";
}
