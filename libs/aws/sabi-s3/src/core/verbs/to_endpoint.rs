use crate::core;
use crate::core::verbs::HasObjectKey;
use crate::core::S3Bucket;
use reqwest::Url;
use std::fmt::Debug;

pub trait ToEndpoint: Debug {
    fn to_endpoint(&self) -> core::Result<Url>;
}

impl<A: HasObjectKey> ToEndpoint for (&S3Bucket, &A) {
    fn to_endpoint(&self) -> core::Result<Url> {
        let (bucket, request) = self;
        let full = format!(
            "https://{}.s3.amazonaws.com/{}",
            bucket.as_str(),
            request.get_object_key(),
        );
        Ok(Url::parse(&full)?)
    }
}
