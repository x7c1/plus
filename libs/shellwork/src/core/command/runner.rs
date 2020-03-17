use crate::core::ExitedProcess;
use crate::error::Error::CommandFailed;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Runner<T> {
    program: String,
    args: Vec<String>,
    env_vars: HashMap<String, String>,
    _prepared_state: PhantomData<T>,
}

impl<T> Runner<T> {
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

    pub fn create_summary(&self) -> RunnerSummary {
        RunnerSummary {
            command: format!("{} {}", &self.program, &self.args.join(" ")),
            env: self.env_vars.clone(),
        }
    }
}

pub fn program<A: Into<String>>(program: A) -> Runner<Unprepared> {
    Runner {
        program: program.into(),
        args: vec![],
        env_vars: HashMap::default(),
        _prepared_state: PhantomData,
    }
}

pub struct Prepared;

impl Runner<Prepared> {
    pub fn spawn(&self) -> crate::Result<ExitedProcess> {
        // rf. [rust - How would you stream output from a Process?](https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process)
        let raw = Command::new(&self.program)
            .args(&self.args)
            .envs(&self.env_vars)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        // todo: use logger
        println!("{:#?}", self.create_summary());

        let child = ExitedProcess::wait(raw)?;
        if child.success() {
            Ok(child)
        } else {
            Err(CommandFailed {
                code: child.status_code(),
                runner: self.create_summary(),
            })
        }
    }
}

pub struct Unprepared;

impl Runner<Unprepared> {
    pub fn prepare<F, E>(self, f: F) -> Result<Runner<Prepared>, E>
    where
        F: Fn(&Self) -> Result<(), E>,
    {
        f(&self)?;
        let runner = Runner {
            program: self.program,
            args: self.args,
            env_vars: self.env_vars,
            _prepared_state: PhantomData,
        };
        Ok(runner)
    }
}

#[derive(Debug)]
pub struct RunnerSummary {
    command: String,
    env: HashMap<String, String>,
}
