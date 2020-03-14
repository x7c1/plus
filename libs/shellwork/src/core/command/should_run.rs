use crate::core::command::CommandProvider;

pub trait ShouldRun: CommandProvider {
    fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err>
    where
        <Self as CommandProvider>::Err: From<crate::Error>,
    {
        let runner = self.prepare(params)?;
        let _status = runner.spawn()?;
        Ok(())
    }
}
