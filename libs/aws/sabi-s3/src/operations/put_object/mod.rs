mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;

mod rich_file;
pub use rich_file::RichFile;

use crate::core::{S3Client, S3Result};
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::verbs::HasObjectKey;
use reqwest::Method;

pub trait Request: HasObjectKey + ResourceLoader {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request,
    {
        let client = InternalClient::new();
        let provider = RequestProvider::new(
            Method::PUT,
            &self.credentials,
            &self.bucket,
            request,
            &self.default_region,
        )?;
        client.request_by(provider)
    }
}
