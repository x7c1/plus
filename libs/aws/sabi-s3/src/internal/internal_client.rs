use reqwest::blocking::Client;

use crate::core::S3Bucket;
use crate::internal::InternalRequest;
use crate::S3Result;
use std::fmt::Debug;

#[derive(Debug)]
pub struct InternalClient<'a> {
    bucket: &'a S3Bucket,
}

impl InternalClient<'_> {
    pub fn new(bucket: &S3Bucket) -> InternalClient {
        InternalClient { bucket }
    }

    pub fn put<A>(&self, request_like: A) -> S3Result<String>
    where
        A: Into<S3Result<InternalRequest>>,
    {
        let request = request_like.into()?;
        println!("put > request > {:#?}", request);

        let builder = Client::new().put(request.url).headers(request.headers);
        let builder = match request.body {
            Some(body) => builder.body(body),
            _ => builder,
        };
        let response = builder.send()?;

        println!("put > response > {:#?}", response);
        Ok("dummy result".to_string())
    }
}
