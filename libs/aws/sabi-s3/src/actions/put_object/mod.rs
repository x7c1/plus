mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;

mod rich_file;
pub use rich_file::RichFile;

use crate::actions::put_object;
use crate::client::S3Client;
use crate::core::verbs::HasObjectKey;
use crate::core::{ETag, S3HeaderMap};
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::{actions, core};
use reqwest::header::HeaderMap;
use reqwest::Method;

pub trait Request: HasObjectKey + ResourceLoader {}

#[derive(Debug)]
pub struct Response {
    pub headers: Headers,
}

#[derive(Debug)]
pub struct Headers {
    pub e_tag: ETag,
}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
    {
        let client = InternalClient::new();
        let provider = RequestProvider::new(
            Method::PUT,
            &self.credentials,
            &self.bucket,
            &request,
            &self.default_region,
        )
        .map_err(|e| put_object::Error::from(e))?;

        let response = client
            .request_by(provider)
            .map_err(|e| put_object::Error::from(e))?;

        let headers = to_headers(response.headers()).map_err(|e| put_object::Error::from(e))?;
        Ok(Response { headers })
    }
}

fn to_headers(map: &HeaderMap) -> core::Result<Headers> {
    Ok(Headers {
        e_tag: map.get_e_tag()?,
    })
}
