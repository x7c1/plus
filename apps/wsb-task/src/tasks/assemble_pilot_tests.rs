use crate::commands::build_pilot::{OutputKind, Params};
use crate::commands::support::Definable;
use crate::commands::{build_pilot, copy_as_artifact, Action};
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;
use shellwork::core::command::{CanDefine, Runnable, RunnerOutput};
use std::path::Path;

pub fn define() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

struct Task;

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "assemble-pilot-tests"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Build E2E tests.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        // let s = Hoge0::build(LinuxX86, matches);

        try_foreach_targets!(|target| {
            // Hoge0::build(target.clone(), matches);

            // build and show compile error if exists.
            let to_params = to_params_for(OutputKind::Default);
            let (action, params) = Action::from(target.clone(), matches, to_params);
            action.spawn(&params)?;

            // Foooooo::build(target.clone(), matches)?;
            // let output = Foooooo::build2(target.clone(), matches)?;

            // build and get filename of pilot-test.
            let to_params = to_params_for(OutputKind::FileName);
            let (action, params) = Action::from(target.clone(), matches, to_params);
            let output = action.capture(&params)?;

            // copy pilot-test file to artifact directory.
            let params = params_to_copy_pilot(target.clone(), output);
            let action = Action::create(&target, &params);
            action.spawn(&params)?;

            TaskResult::Ok(())
        });
        Ok(TaskOutput::empty())
    }
}

impl Foooooo for LinuxX86 {}
impl Foooooo for LinuxArmV7 {}
impl Foooooo for MacX86 {}

trait Foooooo
where
    Self: BuildTarget,
    Self: Clone,
    Action<Self, build_pilot::Params<Self>>: Runnable,
    <Action<Self, build_pilot::Params<Self>> as CanDefine>::Err: From<shellwork::Error>,
    <Action<Self, build_pilot::Params<Self>> as CanDefine>::Params: From<build_pilot::Params<Self>>,
    crate::Error: From<<Action<Self, build_pilot::Params<Self>> as CanDefine>::Err>,
{
    /// build and show compile error if exists.
    fn build(target: Self, matches: &ArgMatches) -> TaskResult<()> {
        let params = create_params(target.clone(), matches, OutputKind::Default);
        let action = Action::create(&target, &params);
        action.spawn(&params.into())?;
        Ok(())
    }

    fn build2(target: Self, matches: &ArgMatches) -> TaskResult<Option<RunnerOutput>> {
        // build and get filename of pilot-test.
        // let to_params = to_params_for(OutputKind::FileName);
        // let (action, params) = Action::from(target.clone(), matches, to_params);
        let params = create_params(target.clone(), matches, OutputKind::FileName);
        let action = Action::create(&target, &params);
        let output = action.capture(&params.into())?;
        Ok(output)
    }
}

fn to_params_for<T>(kind: OutputKind) -> impl FnOnce(T, &ArgMatches) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    |target, matches| create_params(target, matches, kind)
}

fn create_params<T>(target: T, _matches: &ArgMatches, kind: OutputKind) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    build_pilot::Params::builder(kind).target(target).build()
}

fn params_to_copy_pilot<T>(target: T, output: Option<RunnerOutput>) -> copy_as_artifact::Params<T>
where
    T: BuildTarget,
{
    let output = output.expect("output required");
    let src = output.stdout();

    copy_as_artifact::Params::builder(target)
        .src(Path::new(src.as_ref()))
        .dst(Path::new("wsb_pilot_tests"))
        .build()
}
