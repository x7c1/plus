use crate::core::targets::{AsBuildTarget, BuildTarget};
use shellwork::core::env::EnvEntry;

pub trait CCRequired: AsBuildTarget {
    fn cc(&self) -> Option<EnvEntry> {
        let maybe = match self.as_build_target() {
            BuildTarget::LinuxX86 => None,
            BuildTarget::LinuxArmV7 => Some("arm-linux-gnueabihf-gcc"),
            BuildTarget::MacX86 => Some("x86_64-apple-darwin19-clang"),
        };
        maybe.map(|target| EnvEntry {
            key: "CC".to_string(),
            value: target.to_string(),
        })
    }
}
