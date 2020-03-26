mod artifacts;
pub use artifacts::copy_as_artifact;

mod cargo;
pub use cargo::build_apps;
pub use cargo::build_pilot;

pub mod support;

use crate::commands::support::{mac, Definable};
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::TaskResult;
use shellwork::core::command::{may_run, should, RunnerOutput};
use std::marker::PhantomData;

pub struct Action<PARAMS>(PhantomData<PARAMS>);

impl<P> Default for Action<P> {
    fn default() -> Self {
        Action(PhantomData)
    }
}

impl<P> Action<P>
where
    P: Definable,
    P: AsBuildTarget,
{
    pub fn new() -> Action<P> {
        Action(PhantomData)
    }

    pub fn spawn(&self, params: &P) -> TaskResult<()> {
        match params.as_build_target() {
            BuildTarget::LinuxX86 => should::spawn(self, params)?,
            BuildTarget::LinuxArmV7 => should::spawn(self, params)?,
            BuildTarget::MacX86 => may_run::spawn(&mac::RunMaybe::new(self), params)?,
        };
        Ok(())
    }

    pub fn capture(&self, params: &P) -> TaskResult<Option<RunnerOutput>> {
        let maybe = match params.as_build_target() {
            BuildTarget::LinuxX86 => should::capture(self, params)?,
            BuildTarget::LinuxArmV7 => should::capture(self, params)?,
            BuildTarget::MacX86 => may_run::capture(&mac::RunMaybe::new(self), params)?,
        };
        Ok(maybe)
    }
}
