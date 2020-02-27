use crate::actions::put_object::RichFile;
use crate::core::verbs::HasObjectKey;
use crate::internal;
use crate::internal::blocking::{RequestResource, ResourceLoader};
use crate::internal::impl_async;
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

use bytes::BytesMut;
// use futures_util::TryFutureExt;
use futures_util::TryStreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

#[async_trait]
impl impl_async::ResourceLoader for FileRequest {
    async fn load<'a>(&'a self) -> internal::Result<impl_async::RequestResource<'a>> {
        /*
        let file1: tokio::fs::File = tokio::fs::File::open("C:\\Source\\Backup_Ignore.txt").await?;
        let stream1: FramedRead<tokio::fs::File, BytesCodec> = FramedRead::new(file1, BytesCodec::new());
        let stream2 = TryStreamExt::map_ok(stream1, BytesMut::freeze);
        */

        /*
        let stream = tokio::fs::File::open("C:\\Source\\Backup_Ignore.txt")
            .map_ok(|file| FramedRead::new(file, BytesCodec::new()).map_ok(BytesMut::freeze))
            .try_flatten_stream();
        */

        // todo: use async i/f
        let mut file = self.open_file()?;
        let hash = file.reset_cursor_after(|file| HashedPayload::try_from(file))?;
        let content_length = file.metadata()?.len();

        let stream1: FramedRead<tokio::fs::File, BytesCodec> = {
            let file1: tokio::fs::File = tokio::fs::File::from_std(file);
            FramedRead::new(file1, BytesCodec::new())
        };
        let stream2 = TryStreamExt::map_ok(stream1, BytesMut::freeze);

        let resource = impl_async::RequestResource {
            // body: Some(reqwest::Body::from(file)),
            body: Some(reqwest::Body::wrap_stream(stream2)),
            // body: Some(reqwest::Body::wrap_stream(stream)),
            hash,
            region: self.region_code.as_ref(),
            content_type: self.content_type.as_ref(),
            content_length,
            requested_at: now(),
        };
        Ok(resource)
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
