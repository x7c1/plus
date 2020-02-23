use crate::actions::put_object::RichFile;
use crate::core::verbs::HasObjectKey;
use crate::internal;
use crate::internal::{RequestResource, ResourceLoader};
use reqwest::blocking::Body;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::now;
use sabi_core::http::header::ContentType;
use sabi_core::index::RegionCode;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::ErrorKind::NotFound;

#[derive(Debug)]
pub struct FileRequest {
    pub file_path: String,
    pub object_key: String,
    pub content_type: Option<ContentType>,
    pub region_code: Option<RegionCode>,
}

impl FileRequest {
    fn open_file(&self) -> internal::Result<File> {
        File::open(&self.file_path).map_err(|e| match e {
            _ if e.kind() == NotFound => internal::Error::FileNotFound {
                path: self.file_path.to_string(),
                description: e.description().to_string(),
            },
            _ => internal::Error::StdIoError(e),
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
        let mut file = self.open_file()?;
        let hash = file.reset_cursor_after(|file| HashedPayload::try_from(file))?;
        let resource = RequestResource {
            body: Some(Body::from(file)),
            hash,
            region: self.region_code.as_ref(),
            content_type: self.content_type.as_ref(),
            requested_at: now(),
        };
        Ok(resource)
    }
}

impl super::Request for FileRequest {}
