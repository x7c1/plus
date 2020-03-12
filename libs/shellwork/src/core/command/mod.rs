use crate::core::ExitedProcess;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::process::{Command, Stdio};

pub struct Sender {
    program: String,
    args: Vec<String>,
    env_vars: HashMap<String, String>,
}

impl Sender {
    pub fn program<A: Into<String>>(program: A) -> Self {
        Self {
            program: program.into(),
            args: vec![],
            env_vars: HashMap::default(),
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

    pub fn env<K, V>(mut self, key: K, val: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.env_vars.insert(
            key.as_ref().to_string_lossy().to_string(),
            val.as_ref().to_string_lossy().to_string(),
        );
        self
    }

    pub fn spawn(&self) -> crate::Result<ExitedProcess> {
        // rf. [rust - How would you stream output from a Process?](https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process)
        let raw = Command::new(&self.program)
            .args(&self.args)
            .envs(&self.env_vars)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        let child = ExitedProcess::wait(raw)?;
        Ok(child)
    }
}
