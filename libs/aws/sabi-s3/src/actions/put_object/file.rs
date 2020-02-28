use crate::core::verbs::HasObjectKey;
use crate::internal;
use crate::internal::impl_async;
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
    async fn open_file(&self) -> internal::Result<File> {
        File::open(&self.file_path).await.map_err(|e| match e {
            _ if e.kind() == NotFound => internal::Error::FileNotFound {
                path: self.file_path.to_string(),
                description: e.description().to_string(),
            },
            _ => internal::Error::StdIoError(e),
        })
    }

    async fn to_stream_body(&self) -> internal::Result<reqwest::Body> {
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
impl impl_async::ResourceLoader for FileRequest {
    async fn load<'a>(&'a self) -> internal::Result<impl_async::RequestResource<'a>> {
        let file: File = self.open_file().await?;
        let content_length = file.metadata().await?.len();
        let hash = HashedPayload::from_file(file).await?;

        let resource = impl_async::RequestResource {
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

impl super::Request for FileRequest {}
