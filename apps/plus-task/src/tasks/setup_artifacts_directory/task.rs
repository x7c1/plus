use crate::core::build_mode::AsBuildMode;
use crate::core::env::artifacts_dir;
use crate::core::support::get_artifacts_dir;
use crate::core::targets::AsBuildTarget;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn start<P>(&self, params: &P) -> TaskResult<()>
    where
        P: AsBuildTarget,
        P: AsBuildMode,
    {
        self.runner(params)
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    fn runner<P>(&self, params: &P) -> Runner<Unprepared>
    where
        P: AsBuildTarget,
        P: AsBuildMode,
    {
        let dst = get_artifacts_dir(*params.as_build_target(), *params.as_build_mode());
        command::program("mkdir").arg("-p").arg(&dst)
    }
}
