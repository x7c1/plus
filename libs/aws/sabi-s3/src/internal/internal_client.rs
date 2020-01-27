use reqwest::blocking::Client;

use crate::core::S3Bucket;
use crate::verbs::HasObjectKey;
use crate::S3Result;
use reqwest::Url;
use std::fmt::Debug;

#[derive(Debug)]
pub struct InternalClient<'a> {
    bucket: &'a S3Bucket,
}

impl InternalClient<'_> {
    pub fn new(bucket: &S3Bucket) -> InternalClient {
        InternalClient { bucket }
    }

    pub fn post<A: HasObjectKey>(&self, request: A) -> S3Result<String> {
        let full = format!(
            "https://{}.s3.amazonaws.com/{}",
            self.bucket.as_str(),
            request.get_object_key(),
        );
        let url = Url::parse(&full)?;
        println!("url: {}", url);

        let response = Client::new().post(url).body("sample body").send()?;

        println!("put_object request..: {:?}", request);
        println!("put_object response: {:?}", response);
        Ok("dummy result".to_string())
    }
}
