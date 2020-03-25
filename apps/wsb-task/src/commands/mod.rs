mod artifacts;
pub use artifacts::copy_as_artifact;

mod cargo;
pub use cargo::build_apps;
pub use cargo::build_pilot;

pub mod support;

use crate::commands::support::{mac, Definable2};
use crate::core::targets::{AsTargetArch, TargetArch};
use crate::TaskResult;
use clap::ArgMatches;
use shellwork::core::command::{may_run, should};
use std::marker::PhantomData;

pub struct Action<TARGET, PARAMS>(PhantomData<(TARGET, PARAMS)>);

impl<T, P> Action<T, P> {
    pub fn from<F>(target: T, matches: &ArgMatches, to_params: F) -> (Action<T, P>, P)
    where
        F: FnOnce(T, &ArgMatches) -> P,
    {
        let params = to_params(target, matches);
        (Action(PhantomData), params)
    }

    pub fn create(_target: &T, _params: &P) -> Action<T, P> {
        Action(PhantomData)
    }
}

pub struct Action2<PARAMS>(PhantomData<PARAMS>);

impl<P> Default for Action2<P> {
    fn default() -> Self {
        Action2(PhantomData)
    }
}

impl<P> Action2<P>
where
    P: Definable2,
    P: AsTargetArch,
{
    pub fn new() -> Action2<P> {
        Action2(PhantomData)
    }

    pub fn spawn(&self, params: &P) -> TaskResult<()> {
        match params.as_target_arch() {
            TargetArch::LinuxX86 => should::spawn(self, params)?,
            TargetArch::LinuxArmV7 => should::spawn(self, params)?,
            TargetArch::MacX86 => may_run::spawn(&mac::RunMaybe2::new(self), params)?,
        };
        Ok(())
    }
}
