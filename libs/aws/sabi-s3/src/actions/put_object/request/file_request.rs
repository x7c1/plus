use crate::actions::put_object;
use crate::core;
use crate::core::request::{RequestResource, ResourceLoader};
use crate::core::verbs::HasObjectKey;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::now;
use sabi_core::http::header::ContentType;
use sabi_core::index::RegionCode;
use sabi_core::io::stream::bytes_stream;
use std::error::Error;
use std::io::ErrorKind::NotFound;
use tokio::fs::File;

#[derive(Debug)]
pub struct FileRequest {
    pub file_path: String,
    pub object_key: String,
    pub content_type: Option<ContentType>,
    pub region_code: Option<RegionCode>,
}

impl FileRequest {
    async fn open_file(&self) -> core::Result<File> {
        File::open(&self.file_path).await.map_err(|e| match e {
            _ if e.kind() == NotFound => core::Error::FileNotFound {
                path: self.file_path.to_string(),
                description: e.description().to_string(),
            },
            _ => core::Error::StdIoError(e),
        })
    }

    async fn to_stream_body(&self) -> core::Result<reqwest::Body> {
        let file: File = self.open_file().await?;
        let stream = bytes_stream::from_file(file);
        let body = reqwest::Body::wrap_stream(stream);
        Ok(body)
    }
}

impl HasObjectKey for FileRequest {
    fn get_object_key(&self) -> &str {
        &self.object_key
    }
}

#[async_trait]
impl ResourceLoader for FileRequest {
    async fn load<'a>(&'a self) -> core::Result<RequestResource<'a>> {
        let file: File = self.open_file().await?;
        let content_length = file.metadata().await?.len();
        let hash = HashedPayload::from_file(file).await?;

        let resource = RequestResource {
            body: Some(self.to_stream_body().await?),
            hash,
            region: self.region_code.as_ref(),
            content_type: self.content_type.as_ref(),
            content_length,
            requested_at: now(),
        };
        Ok(resource)
    }
}

impl put_object::Request for FileRequest {}
