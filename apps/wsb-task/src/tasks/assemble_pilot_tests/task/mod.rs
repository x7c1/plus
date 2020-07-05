mod build_pilot;
use build_pilot::OutputKind;

use crate::core::build_mode::AsBuildMode;
use crate::core::support::CCRequired;
use crate::core::targets::AsBuildTarget;
use crate::core::ActionOutput;
use crate::tasks::shared::commands::copy_as_artifact;
use crate::TaskResult;
use shellwork::core::command::{no_op, Prepared, Runner, Unprepared};
use std::path::Path;

pub struct Task;

impl Task {
    pub fn start<P>(&self, params: &P) -> TaskResult<()>
    where
        P: AsBuildTarget,
        P: AsBuildMode,
    {
        BuildPilot::new(params, OutputKind::Default).spawn()?;
        let output = BuildPilot::new(params, OutputKind::FileName).capture()?;
        CopyFile::new(params, output).spawn()?;
        Ok(())
    }
}

struct BuildPilot {
    runner: Runner<Unprepared>,
    params: build_pilot::Params,
}

impl BuildPilot {
    pub fn new<P>(params: &P, kind: OutputKind) -> Self
    where
        P: AsBuildMode,
        P: AsBuildTarget,
    {
        let params = build_pilot::Params::builder(kind)
            .target(*params.as_build_target())
            .build_mode(*params.as_build_mode())
            .build();

        let runner = build_pilot::create_runner(&params);
        Self { runner, params }
    }

    pub fn spawn(self) -> TaskResult<()> {
        self.prepare()?.spawn()?;
        Ok(())
    }

    pub fn capture(self) -> TaskResult<ActionOutput<build_pilot::Params>> {
        let output = self.prepare()?.capture()?;
        Ok(ActionOutput::new(output))
    }

    pub fn prepare(self) -> TaskResult<Runner<Prepared>> {
        let params = &self.params;
        self.runner.prepare(|_| confirm_cc(params))
    }
}

fn confirm_cc<P: CCRequired>(params: &P) -> TaskResult<()> {
    // todo:
    println!("params:{:#?}", params.cc());
    Ok(())
}

struct CopyFile {
    runner: Runner<Unprepared>,
}

impl CopyFile {
    pub fn new<P>(params: &P, output: ActionOutput<build_pilot::Params>) -> Self
    where
        P: AsBuildTarget,
    {
        let params = copy_as_artifact::Params::builder(*params.as_build_target())
            .src(&output.pilot_file_path())
            .dst(Path::new("wsb_pilot_tests"))
            .build();

        let runner = copy_as_artifact::create_runner(&params);
        Self { runner }
    }

    pub fn spawn(self) -> TaskResult<()> {
        self.prepare()?.spawn()?;
        Ok(())
    }

    pub fn prepare(self) -> TaskResult<Runner<Prepared>> {
        self.runner.prepare(no_op)
    }
}
