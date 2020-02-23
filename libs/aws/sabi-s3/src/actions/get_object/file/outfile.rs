use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Outfile(PathBuf);

impl Outfile {
    pub fn create(path: PathBuf) -> Result<Outfile, Error> {
        match path.parent() {
            None => Err(Error::ParentDirectoryNotFound(
                path.to_string_lossy().to_string(),
            )),
            Some(_) => Ok(Outfile(path)),
        }
    }

    pub fn directory(&self) -> &Path {
        &self.0.parent().unwrap()
    }
}

impl AsRef<Path> for Outfile {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "ParentDirectoryNotFound > {}", 0)]
    ParentDirectoryNotFound(String),
}
