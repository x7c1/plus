use crate::core::command::runner::can_pipe::CanPipe;
use crate::core::command::{Prepared, Runner};
use crate::error::Error::CommandFailed;
use std::process::{Child, Stdio};

#[derive(Debug)]
pub struct InheritedRunner<'a> {
    pub previous: Child,
    pub runner: &'a Runner<Prepared>,
}

impl InheritedRunner<'_> {
    /// Call `wait` and `spawn` recursively to the end of next_runner.
    pub fn spawn_recursively<T: Into<Stdio>>(self, out: T) -> crate::Result<Child> {
        let mut inherited = self;
        while let Some(next_runner) = &*(inherited.runner).next_runner {
            inherited = InheritedRunner {
                runner: next_runner,
                previous: inherited.spawn_to_pipe()?,
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
}

impl CanPipe for InheritedRunner<'_> {
    fn spawn_to_pipe(mut self) -> crate::Result<Child> {
        if let Some(previous_output) = self.previous.stdout.take() {
            let current = self
                .runner
                .start_spawning(previous_output, Stdio::piped())?;

            self.wait_for_previous()?;
            Ok(current)
        } else {
            self.wait_for_previous()?;
            self.runner.spawn_to_pipe()
        }
    }

    fn spawn_lastly<T: Into<Stdio>>(mut self, output: T) -> crate::Result<Child> {
        if let Some(previous_output) = self.previous.stdout.take() {
            let current = self.runner.start_spawning(previous_output, output)?;

            self.wait_for_previous()?;
            Ok(current)
        } else {
            self.wait_for_previous()?;
            self.runner.spawn_lastly(output)
        }
    }
}
