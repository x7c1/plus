use crate::core::S3Result;
use crate::error::Error::{FileNotFound, StdIoError};
use crate::internal::{RequestResource, ResourceLoader};
use crate::operations::put_object::RichFile;
use crate::operations::Kind;
use crate::verbs::HasObjectKey;
use reqwest::blocking::Body;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::now;
use sabi_core::http::header::ContentType;
use sabi_core::index::RegionCode;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind::NotFound;
use std::io::Write;

#[derive(Debug)]
pub struct FileRequest {
    object_key: String,
    // todo: use BufWriter
    outfile: File,
}

impl FileRequest {
    pub fn new(object_key: String, file_path: String) -> S3Result<Self> {
        Ok(FileRequest {
            object_key,
            outfile: File::create(file_path)?,
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

impl Write for FileRequest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.outfile.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.outfile.flush()
    }
}

impl super::Request for FileRequest {}
