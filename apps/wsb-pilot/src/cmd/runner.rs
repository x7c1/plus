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
        let reified = self.execute()?;
        reified.dump();
        Ok(reified)
    }

    pub fn execute(&self) -> io::Result<CommandOutput> {
        let output = Command::new(&self.program)
            .current_dir(&self.current_dir)
            .args(&self.args)
            .output()?;

        let reified = CommandOutput::new(output);
        let status_code = reified.status_code();
        if 0 != status_code {
            assert!(false, "exit({}) {:#?}", status_code, &self.args);
        }
        Ok(reified)
    }
}
