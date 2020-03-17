use crate::error::Error::CommandFailed;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::process::{Child, Command, Stdio};

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

    fn append_runner(&mut self, runner: Runner<T>)  {
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
        let result = if let Some(next_runner) = &*self.next_runner {
            let child = Command::new(&self.program)
                .args(&self.args)
                .envs(&self.env_vars)
                .stdout(Stdio::piped())
                .spawn()?;

            next_runner.receive(child)
        } else {
            // rf. [rust - How would you stream output from a Process?](https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process)
            let mut raw = Command::new(&self.program)
                .args(&self.args)
                .envs(&self.env_vars)
                .stdout(Stdio::inherit())
                .spawn()?;

            // todo: use logger
            println!("{:#?}", self.create_summary());

            let status = raw.wait()?;
            if status.success() {
                Ok(())
            } else {
                Err(CommandFailed {
                    code: status.code(),
                    runner: self.create_summary(),
                })
            }
        };
        result
    }

    fn receive(&self, mut previous: Child) -> crate::Result<()> {
        if let Some(output) = previous.stdout.take() {
            if let Some(next_runner) = &*self.next_runner {
                let current = Command::new(&self.program)
                    .args(&self.args)
                    .envs(&self.env_vars)
                    .stdin(output)
                    .stdout(Stdio::piped())
                    .spawn()?;

                let _status = previous.wait()?;

                next_runner.receive(current)?
            } else {
                let mut current = Command::new(&self.program)
                    .args(&self.args)
                    .envs(&self.env_vars)
                    .stdin(output)
                    .stdout(Stdio::inherit())
                    .spawn()?;

                let _status1 = current.wait()?;

                let _status2 = previous.wait()?;
            }
        } else {
            let _status = previous.wait()?;
        }
        Ok(())
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
    fn it_works() -> crate::Result<()> {
        // du -ah . | sort -hr | head -n 10

        program("du")
            .args(&["-ah", "."])
            .pipe(program("sort").args(&["-hr"]))
            .pipe(program("head").args(&["-n", "10"]))
            // .pipe(program("grep").args(&["foobarfoobar"]))
            .into_prepared()
            .spawn()?;

        /*
        let r1 = program("du").args(&["-ah", "."]);
        let r2 = program("sort").arg("-hr");
        let r3 = program("head").args(&["-n", "5"]);
        r1.pipe(r2).pipe(r3).into_prepared().spawn();
        */

        assert_eq!(2 + 2, 4);
        Ok(())
    }
}
