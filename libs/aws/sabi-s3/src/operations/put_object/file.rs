use super::error::Error::FileNotFound;
use crate::error::Error::{PutObjectError, StdIoError};
use crate::internal::RequestResource;
use crate::verbs::HasObjectKey;
use crate::S3Result;
use reqwest::blocking::Body;
use std::error::Error;
use std::fs::File;
use std::io::ErrorKind::NotFound;

#[derive(Debug)]
pub struct FileRequest {
    pub file_path: String,
    pub object_key: String,
}

impl FileRequest {
    fn open_file(&self) -> S3Result<File> {
        File::open(&self.file_path).map_err(|e| match e {
            _ if e.kind() == NotFound => PutObjectError(FileNotFound {
                path: self.file_path.to_string(),
                description: e.description().to_string(),
            }),
            _ => StdIoError(e),
        })
    }
}

impl HasObjectKey for FileRequest {
    fn get_object_key(&self) -> &str {
        &self.object_key
    }
}

impl From<FileRequest> for S3Result<RequestResource> {
    fn from(request: FileRequest) -> Self {
        let file = request.open_file()?;
        let body = Body::from(file);
        let resource = RequestResource { body: Some(body) };
        Ok(resource)
    }
}

impl super::Request for FileRequest {}
