mod artifacts;
pub use artifacts::copy_as_artifact;

mod cargo;
pub use cargo::build_apps;
pub use cargo::build_pilot;

pub mod support;

use clap::ArgMatches;
use shellwork::core::command::{CanDefine, Runnable};
use std::marker::PhantomData;

pub struct Action<TARGET, PARAMS>(PhantomData<(TARGET, PARAMS)>);

impl<T, P> Action<T, P> {
    pub fn from<F>(target: T, matches: &ArgMatches, to_params: F) -> (Action<T, P>, P)
    // pub fn from<F>(
    //     target: T,
    //     matches: &ArgMatches,
    //     to_params: F,
    // ) -> (Action<T, P>, <Action<T, P> as CanDefine>::Params)
    where
        F: FnOnce(T, &ArgMatches) -> P,
        // F: FnOnce(T, &ArgMatches) -> <Action<T, P> as CanDefine>::Params,
        // Action<T, P>: Runnable,
        // P: Runnable::
    {
        let params = to_params(target, matches);
        (Action(PhantomData), params)
    }

    pub fn create(_target: &T, _params: &P) -> Action<T, P> {
        Action(PhantomData)
    }
}
