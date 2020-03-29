mod output;
pub use output::ActionOutput;

use crate::commands::support::{mac, Definable};
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::TaskResult;
use shellwork::core::command::{should, MayRun};
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
            BuildTarget::MacX86 => mac::RunMaybe::new(self).spawn(params)?,
        };
        Ok(())
    }

    pub fn capture(&self, params: &P) -> TaskResult<ActionOutput<P>> {
        let output = match params.as_build_target() {
            BuildTarget::LinuxX86 => should::capture(self, params)?,
            BuildTarget::LinuxArmV7 => should::capture(self, params)?,
            BuildTarget::MacX86 => mac::RunMaybe::new(self).capture(params)?,
        };
        let reified = ActionOutput::new(output);
        Ok(reified)
    }
}
