use crate::core::targets::BuildTarget;
use shellwork::core::command::{Runner, Unprepared};

pub trait InsertCC {
    fn insert_cc(self, target: &BuildTarget) -> Self;
}

impl InsertCC for Runner<Unprepared> {
    fn insert_cc(self, target: &BuildTarget) -> Self {
        let maybe = match target {
            BuildTarget::LinuxX86 => None,
            BuildTarget::LinuxArmV7 => Some("arm-linux-gnueabihf-gcc"),
            BuildTarget::MacX86 => Some("x86_64-apple-darwin19-clang"),
        };
        if let Some(cc) = maybe {
            self.env("CC", cc)
        } else {
            self
        }
    }
}
