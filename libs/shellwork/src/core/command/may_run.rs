use crate::core::command::{CanDefine, Prepared, Runner, RunnerOutput};

pub trait MayRun: CanDefine {
    fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>,
    {
        if let Some(runner) = self.prepare_runner(params)? {
            runner.spawn()?;
        }
        Ok(())
    }

    fn capture(&self, params: &Self::Params) -> Result<Option<RunnerOutput>, Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>,
    {
        let maybe = if let Some(runner) = self.prepare_runner(params)? {
            Some(runner.capture()?)
        } else {
            None
        };
        Ok(maybe)
    }

    fn prepare_runner(&self, params: &Self::Params) -> Result<Option<Runner<Prepared>>, Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>,
    {
        let maybe = if let Some(report) = self.unsupported() {
            eprintln!("unsupported command > {:#?}", report);
            None
        } else {
            Some(self.prepare(params)?)
        };
        Ok(maybe)
    }

    fn unsupported(&self) -> Option<UnsupportedReport>;
}

#[derive(Debug)]
pub struct UnsupportedReport(String);

impl UnsupportedReport {
    pub fn new<A: Into<String>>(a: A) -> UnsupportedReport {
        UnsupportedReport(a.into())
    }
}
