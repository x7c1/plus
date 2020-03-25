use crate::core::targets::{RequireCC, TargetArch};
use crate::TaskResult;
use shellwork::core::command::{Runner, Unprepared};

pub trait InsertCC {
    fn insert_cc(self, target: &TargetArch) -> Self;
}

impl InsertCC for Runner<Unprepared> {
    fn insert_cc(self, target: &TargetArch) -> Self {
        let maybe = match target {
            TargetArch::LinuxX86 => None,
            TargetArch::LinuxArmV7 => Some("arm-linux-gnueabihf-gcc"),
            TargetArch::MacX86 => Some("x86_64-apple-darwin19-clang"),
        };
        if let Some(cc) = maybe {
            self.env("CC", cc)
        } else {
            self
        }
    }
}

// todo: deprecate
pub trait CanInsertCC {
    fn with_cc<F>(&self, f: F) -> TaskResult<Runner<Unprepared>>
    where
        Self: Sized,
        F: Fn(&Self) -> Runner<Unprepared>;
}

impl<A> CanInsertCC for A
where
    A: RequireCC,
{
    fn with_cc<F>(&self, f: F) -> TaskResult<Runner<Unprepared>>
    where
        F: Fn(&Self) -> Runner<Unprepared>,
    {
        let runner = f(self).env("CC", A::CC);
        Ok(runner)
    }
}
