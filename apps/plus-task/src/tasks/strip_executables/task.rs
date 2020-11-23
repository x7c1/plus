use crate::core::build_mode::AsBuildMode;
use crate::core::env::executable_names;
use crate::core::support::{get_artifacts_dir, program_exists};
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};
use std::path::{Path, PathBuf};

pub struct Task;

impl Task {
    pub fn start<P>(&self, params: &P) -> TaskResult<()>
    where
        P: AsBuildTarget,
        P: AsBuildMode,
    {
        let file_names = executable_names();
        let runners = file_names
            .iter()
            .map(|file_name| to_path(params, file_name))
            .map(|path| to_runner(params, path));

        for runner in runners {
            let prepared = runner.prepare(program_exists);
            prepared?.spawn()?;
        }
        Ok(())
    }
}

fn to_path<P>(params: &P, file_name: &str) -> PathBuf
where
    P: AsBuildTarget,
    P: AsBuildMode,
{
    let target = params.as_build_target();
    let mode = params.as_build_mode();
    get_artifacts_dir(*target, *mode).join(file_name)
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
