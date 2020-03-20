use crate::core::command::runner::can_spawn::CanSpawn;
use crate::core::command::{Prepared, Runner};
use std::process::{Child, Stdio};

pub struct InheritedRunner<'a> {
    pub previous: Child,
    pub runner: &'a Runner<Prepared>,
}

impl InheritedRunner<'_> {
    /// Call `wait` and `spawn` recursively to the end of next_runner.
    pub fn spawn_recursively(self) -> crate::Result<Child> {
        let mut inherited = self;
        while let Some(next_runner) = &*(inherited.runner).next_runner {
            inherited = InheritedRunner {
                runner: next_runner,
                previous: inherited.spawn_to_pipe()?,
            };
        }
        inherited.spawn_lastly()
    }
}

impl CanSpawn for InheritedRunner<'_> {
    fn spawn_to_pipe(mut self) -> crate::Result<Child> {
        if let Some(previous_output) = self.previous.stdout.take() {
            let current = self
                .runner
                .start_spawning(previous_output, Stdio::piped())?;

            // todo: reject non-zero status
            let _status = self.previous.wait()?;
            Ok(current)
        } else {
            // todo: reject non-zero status?
            let _status = self.previous.wait()?;
            self.runner.spawn_to_pipe()
        }
    }

    fn spawn_lastly(mut self) -> crate::Result<Child> {
        if let Some(previous_output) = self.previous.stdout.take() {
            let current = self
                .runner
                .start_spawning(previous_output, Stdio::inherit())?;

            // todo: reject non-zero status?
            let _status = self.previous.wait()?;
            Ok(current)
        } else {
            // todo: reject non-zero status?
            let _status = self.previous.wait()?;
            self.runner.spawn_lastly()
        }
    }
}
