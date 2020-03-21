use crate::core::command::{CanDefine, RunnerOutput};

pub trait MayRun: CanDefine {
    fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>,
    {
        if let Some(report) = self.unsupported() {
            eprintln!("unsupported command > {:#?}", report);
            return Ok(());
        }
        let runner = self.prepare(params)?;
        runner.spawn()?;
        Ok(())
    }

    fn capture(&self, params: &Self::Params) -> Result<Option<RunnerOutput>, Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>,
    {
        if let Some(report) = self.unsupported() {
            eprintln!("unsupported command > {:#?}", report);
            return Ok(None);
        }
        let runner = self.prepare(params)?;
        let output = runner.capture()?;
        Ok(Some(output))
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
