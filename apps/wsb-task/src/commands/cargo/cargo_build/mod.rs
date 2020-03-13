mod runner;
pub use runner::{BuildRunner, UnsupportedReason};

mod params;
pub use params::Params;

use crate::core::targets::BuildTarget;
use crate::TaskResult;

pub fn spawn<A>(params: &Params<A>) -> TaskResult<()>
where
    A: BuildTarget + BuildRunner,
{
    if let Some(reason) = A::unsupported() {
        eprintln!(
            "unsupported target: {} > {:#?}",
            params.target.as_triple(),
            reason
        );
        return Ok(());
    }
    let _status = A::prepare(params)?.spawn()?;
    Ok(())
}
