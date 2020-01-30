use crate::internal::RequestResource;
use crate::verbs::HasObjectKey;
use crate::S3Result;
use reqwest::blocking::Body;
use std::fs::File;

#[derive(Debug)]
pub struct FileRequest {
    pub file_path: String,
    pub object_key: String,
}

impl HasObjectKey for FileRequest {
    fn get_object_key(&self) -> &str {
        &self.object_key
    }
}

impl From<FileRequest> for S3Result<RequestResource> {
    fn from(request: FileRequest) -> Self {
        let file = File::open(request.file_path)?;
        let body = Body::from(file);
        let resource = RequestResource { body: Some(body) };
        Ok(resource)
    }
}

impl super::Request for FileRequest {}
