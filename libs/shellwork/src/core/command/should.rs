use crate::core::command::{CanDefine, RunnerOutput};

pub fn spawn<A: CanDefine>(this: &A, params: &A::Params) -> Result<(), A::Err>
where
    A::Err: From<crate::Error>,
{
    let runner = this.prepare(params)?;
    runner.spawn()?;
    Ok(())
}

pub fn capture<A: CanDefine>(this: &A, params: &A::Params) -> Result<Option<RunnerOutput>, A::Err>
where
    A::Err: From<crate::Error>,
{
    let runner = this.prepare(params)?;
    let output = runner.capture()?;
    Ok(Some(output))
}
