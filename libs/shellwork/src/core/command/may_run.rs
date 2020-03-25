use crate::core::command::{CanDefine, Prepared, Runner, RunnerOutput};

// todo: move to impl MayRun
pub fn spawn<A: MayRun>(this: &A, params: &A::Params) -> Result<(), A::Err>
where
    A::Err: From<crate::Error>,
{
    if let Some(runner) = prepare_runner(this, params)? {
        runner.spawn()?;
    }
    Ok(())
}

pub fn capture<A: MayRun>(this: &A, params: &A::Params) -> Result<Option<RunnerOutput>, A::Err>
where
    A::Err: From<crate::Error>,
{
    let maybe = if let Some(runner) = prepare_runner(this, params)? {
        Some(runner.capture()?)
    } else {
        None
    };
    Ok(maybe)
}

fn prepare_runner<A: MayRun>(
    this: &A,
    params: &A::Params,
) -> Result<Option<Runner<Prepared>>, A::Err>
where
    A::Err: From<crate::Error>,
{
    let maybe = if let Some(report) = this.unsupported() {
        eprintln!("unsupported command > {:#?}", report);
        None
    } else {
        Some(this.prepare(params)?)
    };
    Ok(maybe)
}

pub trait MayRun: CanDefine {
    fn unsupported(&self) -> Option<UnsupportedReport>;
}

#[derive(Debug)]
pub struct UnsupportedReport(String);

impl UnsupportedReport {
    pub fn new<A: Into<String>>(a: A) -> UnsupportedReport {
        UnsupportedReport(a.into())
    }
}
