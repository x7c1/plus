mod outfile;
pub use outfile::Error as OutfileError;
pub use outfile::Outfile;

use crate::core::verbs::HasObjectKey;
use crate::core::S3Result;
use crate::internal;
use crate::internal::{RequestResource, ResourceLoader};
use crate::operations::get_object;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::now;
use sabi_core::io::BodyReceiver;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(Debug)]
pub struct FileRequest {
    object_key: String,
    outfile: Outfile,
}

impl FileRequest {
    pub fn create(object_key: String, file_path: PathBuf) -> S3Result<Self> {
        Ok(FileRequest {
            object_key,
            outfile: Outfile::create(file_path).map_err(|e| get_object::Error::from(e))?,
        })
    }
}

impl HasObjectKey for FileRequest {
    fn get_object_key(&self) -> &str {
        &self.object_key
    }
}

impl ResourceLoader for FileRequest {
    fn load(&self) -> internal::Result<RequestResource> {
        let resource = RequestResource {
            body: None,
            hash: HashedPayload::empty(),
            region: None,
            content_type: None,
            requested_at: now(),
        };
        Ok(resource)
    }
}

impl BodyReceiver for FileRequest {
    type Err = crate::Error;

    fn receive_body_from<A: Read>(&mut self, mut body: A) -> S3Result<u64> {
        let dir = self.outfile.directory();
        let mut tmp = NamedTempFile::new_in(dir)?;
        let size = io::copy(&mut body, &mut tmp)?;
        tmp.persist(&self.outfile).map_err(|e| io::Error::from(e))?;
        Ok(size)
    }
}

impl super::Request for FileRequest {}
