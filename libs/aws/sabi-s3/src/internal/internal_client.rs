use reqwest::blocking::Client;

use crate::core::S3Bucket;
use crate::verbs::ToEndpoint;
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

    pub fn put<A: ToEndpoint>(&self, request: A) -> S3Result<String> {
        let url = request.to_endpoint()?;
        println!("url: {}", url);

        let response = Client::new().put(url).body("sample body").send()?;

        println!("put_object request..: {:#?}", request);
        println!("put_object response: {:#?}", response);
        Ok("dummy result".to_string())
    }
}
