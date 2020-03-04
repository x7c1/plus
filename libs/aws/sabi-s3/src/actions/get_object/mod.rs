mod error;
pub use error::Error;

mod request;
pub use request::FileRequest;
pub use request::{Outfile, OutfileError};

mod response;
pub use response::{Response, ResponseHeaders};

use crate::actions;
use crate::actions::get_object;
use crate::client::S3Client;
use crate::core::request::ResourceLoader;
use crate::core::verbs::{HasObjectKey, IsGet};
use crate::internal::impl_async::{InternalClient, RequestProvider};
use futures_util::TryStreamExt;
use sabi_core::io::stream::BodyReceiver;

/// rf.
/// [GetObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)
pub trait Request:
    HasObjectKey + ResourceLoader + BodyReceiver<Err = get_object::Error> + Send + Sync
{
}

impl<A: Request> IsGet<Response> for A {}

type Result<A> = std::result::Result<A, get_object::Error>;

#[async_trait]
pub trait Requester {
    async fn get_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send;
}

#[async_trait]
impl Requester for S3Client {
    async fn get_object<A>(&self, mut request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send,
    {
        let client = InternalClient::default();
        let headers: get_object::Result<ResponseHeaders> = async {
            let provider = RequestProvider::new(&self, &request)?;
            let response = client.request_by(provider).await?;
            let headers = ResponseHeaders::from(response.headers())?;
            let stream = response.bytes_stream().map_err(get_object::Error::from);

            request.receive_body_from(stream).await?;
            Ok(headers)
        }
        .await;
        Ok(Response { headers: headers? })
    }
}
