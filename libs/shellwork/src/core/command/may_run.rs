use crate::core::command::CommandProvider;

pub trait MayRun: CommandProvider {
    fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err>
    where
        <Self as CommandProvider>::Err: From<crate::Error>,
    {
        if let Some(report) = self.unsupported() {
            eprintln!("unsupported command > {:#?}", report);
            return Ok(());
        }
        let runner = self.prepare(params)?;
        let _status = runner.spawn()?;
        Ok(())
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
