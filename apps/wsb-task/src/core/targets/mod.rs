mod linux_x86;
pub use linux_x86::LinuxX86;

mod linux_arm_v7;
pub use linux_arm_v7::LinuxArmV7;

mod mac_x86;
pub use mac_x86::MacX86;

mod require_cc;
pub use require_cc::{InsertCC, RequireCC};

pub trait BuildTarget {
    fn as_triple(&self) -> &str;
}

pub fn all() -> Vec<Box<dyn BuildTarget>> {
    vec![Box::new(LinuxX86), Box::new(LinuxArmV7), Box::new(MacX86)]
}

macro_rules! try_foreach_targets {
    (|$arg:ident| $x:expr) => {
        #[allow(clippy::redundant_closure_call)]
        {
            (|$arg: crate::core::targets::LinuxX86| $x)(crate::core::targets::LinuxX86)?;
            (|$arg: crate::core::targets::LinuxArmV7| $x)(crate::core::targets::LinuxArmV7)?;
            (|$arg: crate::core::targets::MacX86| $x)(crate::core::targets::MacX86)?;
        }
    };
}
