use crate::core::command::CanDefine;

pub trait ShouldRun: CanDefine {
    fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>,
    {
        let runner = self.prepare(params)?;
        let _status = runner.spawn()?;
        Ok(())
    }
}
