use crate::core::command::{Prepared, Runner, RunnerSummary};
use crate::error::Error::CommandFailed;
use std::process::{Child, Stdio};

#[derive(Debug)]
pub struct InheritedRunner<'a> {
    pub previous: Child,
    pub previous_summary: RunnerSummary,
    pub runner: &'a Runner<Prepared>,
}

impl InheritedRunner<'_> {
    /// Call `wait` and `spawn` recursively to the end of next_runner.
    pub fn spawn_recursively<T: Into<Stdio>>(&mut self, out: T) -> crate::Result<Child> {
        let inherited = self;
        while let Some(next_runner) = &*(inherited.runner).next_runner {
            let (previous, previous_summary) = inherited.spawn_to_pipe()?;
            *inherited = InheritedRunner {
                previous,
                previous_summary,
                runner: next_runner,
            };
        }
        inherited.spawn_lastly(out)
    }

    fn wait_for_previous(&mut self) -> crate::Result<()> {
        let status = self.previous.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(CommandFailed {
                code: status.code(),
                summary: self.runner.create_summary(),
            })
        }
    }

    fn spawn_to_pipe(&mut self) -> crate::Result<(Child, RunnerSummary)> {
        if let Some(previous_output) = self.previous.stdout.take() {
            let current = self
                .runner
                .start_spawning(previous_output, Stdio::piped())?;

            self.wait_for_previous()?;
            Ok((current, self.runner.create_summary()))
        } else {
            unimplemented!()
            /*
            // fixme: what is correct procedure when stdout has not been captured?
            // should it be handled as error?
            Err(...)

            // or ignore absent stdout and keep on running command?
            self.wait_for_previous()?;
            self.runner.spawn_to_pipe()
            */
        }
    }

    fn spawn_lastly<T: Into<Stdio>>(&mut self, output: T) -> crate::Result<Child> {
        if let Some(previous_output) = self.previous.stdout.take() {
            let current = self.runner.start_spawning(previous_output, output)?;

            self.wait_for_previous()?;
            Ok(current)
        } else {
            unimplemented!()
            /*
            // fixme: see above todo in spawn_to_pipe.
            self.wait_for_previous()?;
            self.runner.spawn_lastly(output)
            */
        }
    }
}
