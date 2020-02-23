use std::fs::File;
use std::io::{Seek, SeekFrom};

pub trait RichFile {
    fn reset_cursor_after<F, A, E>(&mut self, f: F) -> Result<A, E>
    where
        F: Fn(&File) -> Result<A, E>,
        E: From<std::io::Error>;
}

impl RichFile for File {
    fn reset_cursor_after<F, A, E>(&mut self, f: F) -> Result<A, E>
    where
        F: Fn(&File) -> Result<A, E>,
        E: From<std::io::Error>,
    {
        let x = f(self)?;
        self.seek(SeekFrom::Start(0))?;
        Ok(x)
    }
}
