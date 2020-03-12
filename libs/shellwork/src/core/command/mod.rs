use crate::core::ExitedProcess;
use std::ffi::OsStr;
use std::process::Command;

pub struct Sender {
    program: String,
    args: Vec<String>,
}

impl Sender {
    pub fn program<A: Into<String>>(program: A) -> Self {
        Self {
            program: program.into(),
            args: vec![],
        }
    }

    pub fn arg<A: AsRef<OsStr>>(mut self, arg: A) -> Self {
        self.args.push(arg.as_ref().to_string_lossy().to_string());
        self
    }

    pub fn args<I, S>(self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut xs = self;
        for arg in args {
            xs = xs.arg(arg);
        }
        xs
    }

    pub fn spawn(&self) -> crate::Result<ExitedProcess> {
        let raw = Command::new(&self.program).args(&self.args).spawn()?;
        let child = ExitedProcess::wait(raw)?;
        Ok(child)
    }
}
