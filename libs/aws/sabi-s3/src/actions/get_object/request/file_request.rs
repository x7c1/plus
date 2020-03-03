use crate::actions;
use crate::actions::get_object;
use crate::actions::get_object::Outfile;
use crate::core;
use crate::core::request::{RequestResource, ResourceLoader};
use crate::core::verbs::HasObjectKey;
use bytes::Bytes;
use futures_util::stream::Stream;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::now;
use sabi_core::io::stream::BodyReceiver;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileRequest {
    object_key: String,
    outfile: Outfile,
}

impl FileRequest {
    pub fn create(object_key: String, file_path: PathBuf) -> actions::Result<Self> {
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

#[async_trait]
impl ResourceLoader for FileRequest {
    async fn load<'a>(&'a self) -> core::Result<RequestResource<'a>> {
        let resource = RequestResource {
            body: None,
            hash: HashedPayload::empty(),
            region: None,
            content_type: None,
            content_length: 0,
            requested_at: now(),
        };
        Ok(resource)
    }
}

#[async_trait]
impl BodyReceiver for FileRequest {
    type Err = get_object::Error;

    async fn receive_body_from<S>(&mut self, body: S) -> Result<usize, Self::Err>
    where
        S: Stream<Item = Result<Bytes, Self::Err>>,
        S: Send,
    {
        let sum = self.outfile.write(body).await?;
        Ok(sum)
    }
}

impl get_object::Request for FileRequest {}
