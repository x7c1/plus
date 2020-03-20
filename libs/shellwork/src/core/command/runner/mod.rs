mod can_pipe;
use can_pipe::CanPipe;

mod inherited;
use inherited::InheritedRunner;

mod output;
pub use output::RunnerOutput;

use crate::error::Error::CommandFailed;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::process::{Child, Command, ExitStatus, Stdio};

#[derive(Debug)]
pub struct Runner<T> {
    program: String,
    args: Vec<String>,
    env_vars: HashMap<String, String>,
    next_runner: Box<Option<Runner<T>>>,
    _prepared_state: PhantomData<T>,
}

impl<T> Runner<T>
where
    T: Debug,
{
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

    pub fn pipe(mut self, next: Runner<T>) -> Self {
        self.append_runner(next);
        self
    }

    fn append_runner(&mut self, runner: Runner<T>) {
        if let Some(ref mut next) = *self.next_runner {
            next.append_runner(runner);
        } else {
            self.next_runner = Box::new(Some(runner));
        }
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
        next_runner: Box::new(None),
        _prepared_state: PhantomData,
    }
}

#[derive(Debug)]
pub struct Prepared;

impl Runner<Prepared> {
    pub fn spawn(&self) -> crate::Result<()> {
        let mut child = self.spawn_to(Stdio::inherit())?;
        self.validate_status(&child.wait()?)?;
        Ok(())
    }

    pub fn capture(&self) -> crate::Result<RunnerOutput> {
        let child = self.spawn_to(Stdio::piped())?;
        let output = child.wait_with_output()?;
        self.validate_status(&output.status)?;
        Ok(RunnerOutput::new(output))
    }

    fn validate_status(&self, status: &ExitStatus) -> crate::Result<()> {
        if status.success() {
            Ok(())
        } else {
            Err(CommandFailed {
                code: status.code(),
                summary: self.create_summary(),
            })
        }
    }

    fn spawn_to<T: Into<Stdio>>(&self, output: T) -> crate::Result<Child> {
        let child = if let Some(next) = self.spawn_to_inherit()? {
            next.spawn_recursively(output)?
        } else {
            // todo: use logger
            println!("{:#?}", self.create_summary());
            self.spawn_lastly(output)?
        };
        Ok(child)
    }

    fn spawn_to_inherit(&self) -> crate::Result<Option<InheritedRunner>> {
        let spawn = |runner| {
            self.spawn_to_pipe().map(|child| InheritedRunner {
                runner,
                previous: child,
            })
        };
        (*self.next_runner).as_ref().map(spawn).transpose()
    }

    fn start_spawning<T1, T2>(&self, stdin: T1, stdout: T2) -> crate::Result<Child>
    where
        T1: Into<Stdio>,
        T2: Into<Stdio>,
    {
        let child = Command::new(&self.program)
            .args(&self.args)
            .envs(&self.env_vars)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()?;

        Ok(child)
    }
}

#[derive(Debug)]
pub struct Unprepared;

impl Runner<Unprepared> {
    pub fn prepare<F, E>(self, f: F) -> Result<Runner<Prepared>, E>
    where
        F: Fn(&Self) -> Result<(), E>,
    {
        f(&self)?;
        let runner = self.into_prepared();
        Ok(runner)
    }

    fn into_prepared(self) -> Runner<Prepared> {
        let next_runner = {
            let next = self.next_runner.map(|x| x.into_prepared());
            Box::new(next)
        };
        Runner {
            program: self.program,
            args: self.args,
            env_vars: self.env_vars,
            next_runner,
            _prepared_state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RunnerSummary {
    command: String,
    env: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use crate::core::command::runner::program;

    #[test]
    fn it_can_pipe_once() -> crate::Result<()> {
        let piped_once = program("echo")
            .args(&["11\n22\n33\n44\n55"])
            .pipe(program("head").args(&["-n", "3"]))
            .into_prepared();

        let output = piped_once.capture()?;
        let expected = "11\n22\n33\n";
        assert_eq!(output.stdout(), expected);

        Ok(())
    }

    #[test]
    fn it_can_pipe_twice() -> crate::Result<()> {
        let piped_twice = program("echo")
            .args(&["11\n22\n33\n44\n55"])
            .pipe(program("sort").args(&["-r"]))
            .pipe(program("head").args(&["-n", "3"]))
            .into_prepared();

        let output = piped_twice.capture()?;
        let expected = "55\n44\n33\n";
        assert_eq!(output.stdout(), expected);

        Ok(())
    }
}
