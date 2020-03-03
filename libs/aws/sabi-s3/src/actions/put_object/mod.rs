mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;

mod response;
pub use response::{Response, ResponseHeaders};

use crate::actions;
use crate::actions::put_object;
use crate::client::S3Client;
use crate::core::request::ResourceLoader;
use crate::core::verbs::{HasObjectKey, IsPut};
use crate::internal::impl_async::{InternalClient, RequestProvider};

/// rf.
/// [PutObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)
pub trait Request: HasObjectKey + ResourceLoader + Send + Sync {}

impl<A: Request> IsPut<Response> for A {}

type Result<A> = std::result::Result<A, put_object::Error>;

#[async_trait]
pub trait Requester {
    async fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send,
        A: Sync;
}

#[async_trait]
impl Requester for S3Client {
    async fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send,
        A: Sync,
    {
        let client = InternalClient::new();
        let headers: put_object::Result<ResponseHeaders> = async {
            let provider = RequestProvider::new(&self, &request)?;
            let response = client.request_by(provider).await?;
            Ok(ResponseHeaders::from(response.headers())?)
        }
        .await;
        Ok(Response { headers: headers? })
    }
}
