use bytes::Bytes;
use futures_util::stream::Stream;
use futures_util::TryStreamExt;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

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
        &self.0.parent().expect("must have parent")
    }

    pub async fn write<S, E>(&self, stream: S) -> Result<usize, E>
    where
        S: Stream<Item = Result<Bytes, E>>,
        E: From<std::io::Error>,
    {
        let file = {
            let dir = self.directory();
            NamedTempFile::new_in(dir)?
        };
        let write = stream.try_fold((0, file), |(acc, mut tmp), item: Bytes| {
            async move {
                let size = tmp.write(&item)?;
                Ok((acc + size, tmp))
            }
        });
        let (sum, file) = write.await?;
        file.persist(&self.0).map_err(|e| std::io::Error::from(e))?;
        Ok(sum)
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
