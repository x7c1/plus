use crate::cmd::CommandOutput;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct CommandRunner {
    program: String,
    current_dir: PathBuf,
    args: Vec<String>,
}

impl CommandRunner {
    pub fn new<A: Into<String>>(program: A) -> Self {
        Self {
            program: program.into(),
            current_dir: PathBuf::new(),
            args: vec![],
        }
    }

    pub fn current_dir<A: AsRef<Path>>(mut self, dir: A) -> Self {
        self.current_dir = dir.as_ref().to_path_buf();
        self
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

    pub fn output(&self) -> io::Result<CommandOutput> {
        let reified = self.execute_silently()?;
        reified.dump();

        assert_eq!(reified.status_code(), 0);
        Ok(reified)
    }

    pub fn execute(&self) -> io::Result<CommandOutput> {
        let reified = self.execute_silently()?;
        reified.dump();
        Ok(reified)
    }

    pub fn execute_silently(&self) -> io::Result<CommandOutput> {
        let output = Command::new(&self.program)
            .current_dir(&self.current_dir)
            .args(&self.args)
            .output()?;

        let reified = CommandOutput::new(output);
        Ok(reified)
    }

    pub fn run<F, X1, Y>(self, f: F, x1: X1) -> Y
    where
        F: Fn(CommandRunner, X1) -> Y,
    {
        f(self, x1)
    }
}
