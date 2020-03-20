use crate::commands::build_pilot::{DefaultFormat, ShowFileName};
use crate::commands::cargo::build_pilot;
use crate::commands::support::{mac, should, CanInsertCC, Definable};
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

fn base_runner<T, O>(params: &build_pilot::Params<T, O>) -> Runner<Unprepared>
where
    T: BuildTarget,
{
    // todo: move opt-level to params
    // todo: enable to add mode (--release)
    command::program("cargo")
        .arg("test")
        // .arg("--verbose")
        .args(&["--target", &params.target.as_triple()])
        .args(&["--package", "wsb-pilot"])
        .arg("--no-run")
        .env("RUSTFLAGS", "-C opt-level=0")
}

mod linux_x86 {
    use super::*;
    mod default {
        use super::*;
        impl Definable for build_pilot::Params<LinuxX86, DefaultFormat> {
            fn define(&self) -> TaskResult<Runner<Unprepared>> {
                Ok(base_runner(self))
            }
        }
        impl should::Run for build_pilot::Params<LinuxX86, DefaultFormat> {}
    }
    mod for_file_name {
        use super::*;
        impl Definable for build_pilot::Params<LinuxX86, ShowFileName> {
            fn define(&self) -> TaskResult<Runner<Unprepared>> {
                Ok(base_runner(self))
            }
        }
        impl should::Run for build_pilot::Params<LinuxX86, ShowFileName> {}
    }
}

mod linux_arm_v7 {
    use super::*;
    mod default {
        use super::*;
        impl Definable for build_pilot::Params<LinuxArmV7, DefaultFormat> {
            fn define(&self) -> TaskResult<Runner<Unprepared>> {
                self.with_cc(base_runner)
            }
        }
        impl should::Run for build_pilot::Params<LinuxArmV7, DefaultFormat> {}
    }
    mod for_file_name {
        use super::*;
        impl Definable for build_pilot::Params<LinuxArmV7, ShowFileName> {
            fn define(&self) -> TaskResult<Runner<Unprepared>> {
                self.with_cc(base_runner)
            }
        }
        impl should::Run for build_pilot::Params<LinuxArmV7, ShowFileName> {}
    }
}

mod mac_x86 {
    use super::*;
    mod default {
        use super::*;
        impl Definable for build_pilot::Params<MacX86, DefaultFormat> {
            fn define(&self) -> TaskResult<Runner<Unprepared>> {
                self.with_cc(base_runner)
            }
        }
        impl mac::RunMaybe for build_pilot::Params<MacX86, DefaultFormat> {}
    }
    mod for_file_name {
        use super::*;
        impl Definable for build_pilot::Params<MacX86, ShowFileName> {
            fn define(&self) -> TaskResult<Runner<Unprepared>> {
                self.with_cc(base_runner)
            }
        }
        impl mac::RunMaybe for build_pilot::Params<MacX86, ShowFileName> {}
    }
}
