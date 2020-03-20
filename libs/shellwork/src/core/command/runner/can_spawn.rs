use crate::core::command::{Prepared, Runner};
use std::process::{Child, Stdio};

pub trait CanSpawn {
    fn spawn_to_pipe(self) -> crate::Result<Child>;
    fn spawn_lastly(self) -> crate::Result<Child>;
}

impl CanSpawn for &Runner<Prepared> {
    fn spawn_to_pipe(self) -> crate::Result<Child> {
        self.start_spawning(Stdio::inherit(), Stdio::piped())
    }

    fn spawn_lastly(self) -> crate::Result<Child> {
        self.start_spawning(Stdio::inherit(), Stdio::inherit())
    }
}

impl CanSpawn for (&Runner<Prepared>, Child) {
    fn spawn_to_pipe(mut self) -> crate::Result<Child> {
        if let Some(previous_output) = self.1.stdout.take() {
            let current = self.0.start_spawning(previous_output, Stdio::piped())?;

            // todo: reject non-zero status
            let _status = self.1.wait()?;
            Ok(current)
        } else {
            // todo: reject non-zero status?
            let _status = self.1.wait()?;
            self.0.spawn_to_pipe()
        }
    }

    fn spawn_lastly(mut self) -> crate::Result<Child> {
        if let Some(previous_output) = self.1.stdout.take() {
            let current = self.0.start_spawning(previous_output, Stdio::inherit())?;

            // todo: reject non-zero status?
            let _status = self.1.wait()?;
            Ok(current)
        } else {
            // todo: reject non-zero status?
            let _status = self.1.wait()?;
            self.0.spawn_lastly()
        }
    }
}
