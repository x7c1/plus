use crate::core::S3Result;

use crate::internal::{RequestResource, ResourceLoader};
use crate::verbs::HasObjectKey;
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
    outfile: PathBuf,
}

impl FileRequest {
    pub fn new(object_key: String, file_path: PathBuf) -> S3Result<Self> {
        Ok(FileRequest {
            object_key,
            outfile: file_path,
        })
    }
}

impl HasObjectKey for FileRequest {
    fn get_object_key(&self) -> &str {
        &self.object_key
    }
}

impl ResourceLoader for FileRequest {
    fn load(&self) -> S3Result<RequestResource> {
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
        // todo: remove unwrap
        let dir = self.outfile.parent().unwrap();
        let mut tmp = NamedTempFile::new_in(dir)?;
        let size = io::copy(&mut body, &mut tmp)?;
        tmp.persist(&self.outfile).map_err(|e| io::Error::from(e))?;
        Ok(size)
    }
}

impl super::Request for FileRequest {}
