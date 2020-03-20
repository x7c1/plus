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
