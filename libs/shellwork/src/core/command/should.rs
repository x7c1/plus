use crate::core::command::{CanDefine, RunnerOutput};

pub fn spawn<A>(this: &A, params: &A::Params) -> Result<(), A::Err>
where
    A: CanDefine,
    A::Err: From<crate::Error>,
{
    let runner = this.prepare(params)?;
    runner.spawn()?;
    Ok(())
}

pub fn capture<A>(this: &A, params: &A::Params) -> Result<Option<RunnerOutput>, A::Err>
where
    A: CanDefine,
    A::Err: From<crate::Error>,
{
    let runner = this.prepare(params)?;
    let output = runner.capture()?;
    Ok(Some(output))
}
