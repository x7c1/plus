use crate::core::env::{artifacts_dir, executable_names};
use crate::core::support::confirm_program;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};
use std::path::{Path, PathBuf};

pub struct Task;

impl Task {
    pub fn start<P: AsBuildTarget>(&self, params: &P) -> TaskResult<()> {
        let file_names = executable_names();
        let runners = file_names
            .iter()
            .map(|file_name| to_path(params, file_name))
            .map(|path| to_runner(params, path));

        for runner in runners {
            let prepared = runner.prepare(confirm_program);
            prepared?.spawn()?;
        }
        Ok(())
    }
}

fn to_path<P: AsBuildTarget>(params: &P, file_name: &str) -> PathBuf {
    let target = params.as_build_target();
    artifacts_dir().join(target.as_abbr()).join(file_name)
}

fn to_runner<A, B>(params: &A, path: B) -> Runner<Unprepared>
where
    A: AsBuildTarget,
    B: AsRef<Path>,
{
    let program = match params.as_build_target() {
        BuildTarget::LinuxX86 => "strip",
        BuildTarget::LinuxArmV7 => "arm-linux-gnueabihf-strip",
        BuildTarget::MacX86 => "x86_64-apple-darwin19-strip",
    };
    let file_path = path.as_ref().to_string_lossy().to_string();
    command::program(program).arg(&file_path)
}
