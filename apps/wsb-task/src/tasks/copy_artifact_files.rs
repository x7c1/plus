use crate::commands::{app_names, copy_as_artifact};
use crate::core::targets::BuildTarget;
use crate::core::Action;
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;
use std::path::Path;

pub fn define() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

struct Task;

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "copy-artifact-files"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Copy files to artifact directory.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        BuildTarget::all()
            .iter()
            .map(|target| TaskCommands { target, matches })
            .try_for_each(|commands| commands.run())?;

        Ok(TaskOutput::empty())
    }
}

struct TaskCommands<'a> {
    target: &'a BuildTarget,
    #[allow(dead_code)]
    matches: &'a ArgMatches<'a>,
}

impl TaskCommands<'_> {
    fn run(&self) -> TaskResult<()> {
        self.copy_workspace()?;
        self.copy_test_runner()?;
        self.copy_release_apps()?;
        Ok(())
    }

    fn copy_workspace(&self) -> TaskResult<()> {
        let params = copy_as_artifact::Params::builder(self.target)
            .src(Path::new("dist.bundle/wsb-pilot-workspace"))
            .dst(Path::new("wsb-pilot-workspace"))
            .build();

        Action::new().spawn(&params)
    }

    fn copy_test_runner(&self) -> TaskResult<()> {
        let params = copy_as_artifact::Params::builder(self.target)
            .src(Path::new("dist.bundle/run_pilot_tests.sh.template"))
            .dst(Path::new("run_pilot_tests.sh"))
            .build();

        Action::new().spawn(&params)
    }

    fn copy_release_apps(&self) -> TaskResult<()> {
        for name in app_names().iter() {
            let params = self.to_app_params(name);
            Action::new().spawn(&params)?;
        }
        Ok(())
    }

    fn to_app_params(&self, name: &str) -> copy_as_artifact::Params {
        copy_as_artifact::Params::builder(self.target)
            .src(
                &Path::new("target")
                    .join(self.target.as_triple())
                    .join("release")
                    .join(name),
            )
            .dst(Path::new(name))
            .build()
    }
}
