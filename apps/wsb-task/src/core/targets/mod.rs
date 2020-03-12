mod linux_x86;
pub use linux_x86::LinuxX86;

mod linux_arm_v7;
pub use linux_arm_v7::LinuxArmV7;

mod mac_x86;
pub use mac_x86::MacX86;

pub trait BuildTarget {
    fn as_triple(&self) -> &str;
}

impl dyn BuildTarget {
    pub fn all() -> Vec<Box<dyn BuildTarget>> {
        vec![Box::new(LinuxX86), Box::new(LinuxArmV7), Box::new(MacX86)]
    }
}

#[macro_export]
macro_rules! try_foreach_targets {
    (|$arg:ident| $x:expr) => {
        (|$arg| $x)(crate::core::targets::LinuxX86)?;
        (|$arg| $x)(crate::core::targets::LinuxArmV7)?;
        (|$arg| $x)(crate::core::targets::MacX86)?;
    };
}
