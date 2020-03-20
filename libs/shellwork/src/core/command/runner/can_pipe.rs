use crate::core::command::{Prepared, Runner};
use std::process::{Child, Stdio};

pub trait CanPipe {
    fn spawn_to_pipe(self) -> crate::Result<Child>;
    fn spawn_lastly<T: Into<Stdio>>(self, out: T) -> crate::Result<Child>;
}

impl CanPipe for &Runner<Prepared> {
    fn spawn_to_pipe(self) -> crate::Result<Child> {
        self.start_spawning(Stdio::inherit(), Stdio::piped())
    }

    fn spawn_lastly<T: Into<Stdio>>(self, out: T) -> crate::Result<Child> {
        self.start_spawning(Stdio::inherit(), out)
    }
}
