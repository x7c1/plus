use reqwest::blocking::Client;

use crate::S3Result;
use crate::core::S3Bucket;
use std::fmt::Debug;

pub struct InternalClient<'a> {
    bucket: &'a S3Bucket,
}

impl InternalClient<'_> {
    pub fn new(bucket: &S3Bucket) -> InternalClient {
        InternalClient {
            bucket,
        }
    }

    pub fn post<A: Debug>(&self, request: A) -> S3Result<String> {
//        let full = format!(
//            "https://{}.s3.amazonaws.com/{}",
//            self.bucket,
//            request.object_key(),
//        );
//        Url::parse(&full)?

        let client = Client::new();
        let response = client
            .post("https://example.com/")
            .body("sample body")
            .send()?;

        println!("put_object request..: {:?}", request);
        println!("put_object response: {:?}", response);
        Ok("dummy result".to_string())
    }
}
