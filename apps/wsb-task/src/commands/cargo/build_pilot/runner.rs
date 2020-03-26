use crate::commands::build_pilot;
use crate::commands::build_pilot::OutputKind;
use crate::commands::support::{Definable2, InsertCC};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{program, Runner, Unprepared};

impl Definable2 for build_pilot::Params<'_> {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        // todo: move opt-level to params
        // todo: enable to add mode (--release)
        let default = command::program("cargo")
            .arg("test")
            .args(&["--target", self.target.as_triple()])
            .args(&["--package", "wsb-pilot"])
            .arg("--no-run")
            .env("RUSTFLAGS", "-C opt-level=0")
            .insert_cc(&self.target);

        // call via OutputKind::Default in advance to see compilation errors,
        // since OutputKind::FileName hides them by the --message-format option.
        let runner = match self.output_kind {
            OutputKind::Default => default,
            OutputKind::FileName => pipe_to_get_file_name(default),
        };
        Ok(runner)
    }
}

/*
fn define<T>(params: &build_pilot::Params) -> Runner<Unprepared>
where
    T: BuildTarget,
{
    // todo: move opt-level to params
    // todo: enable to add mode (--release)
    let default = command::program("cargo")
        .arg("test")
        .args(&["--target", &params.target.as_triple()])
        .args(&["--package", "wsb-pilot"])
        .arg("--no-run")
        .env("RUSTFLAGS", "-C opt-level=0");

    // call via OutputKind::Default in advance to see compilation errors,
    // since OutputKind::FileName hides them by the --message-format option.
    match params.output_kind {
        OutputKind::Default => default,
        OutputKind::FileName => pipe_to_get_file_name(default),
    }
}
*/

fn pipe_to_get_file_name(base: Runner<Unprepared>) -> Runner<Unprepared> {
    let jq = program("jq").args(&["-r", "select(.profile.test == true) | .filenames[]"]);
    let grep = program("grep").arg("wsb_pilot_tests");
    let tr = program("tr").args(&["-d", "\n"]);
    base.args(&["--message-format", "json"])
        .pipe(jq)
        .pipe(grep)
        .pipe(tr)
}

/*
mod linux_x86 {
    use super::*;
    impl Definable for build_pilot::Params<LinuxX86> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            Ok(define(self))
        }
    }
    impl should::Run for build_pilot::Params<LinuxX86> {}
}

mod linux_arm_v7 {
    use super::*;
    impl Definable for build_pilot::Params<LinuxArmV7> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            self.with_cc(define)
        }
    }
    impl should::Run for build_pilot::Params<LinuxArmV7> {}
}

mod mac_x86 {
    use super::*;
    impl Definable for build_pilot::Params<MacX86> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            self.with_cc(define)
        }
    }
    impl mac::RunMaybe for build_pilot::Params<MacX86> {}
}
*/
