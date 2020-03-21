pub mod artifacts;

mod cargo;
pub use cargo::build_apps;
pub use cargo::build_pilot;

pub mod support;

use clap::ArgMatches;
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
}
