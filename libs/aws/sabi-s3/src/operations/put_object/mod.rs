mod file_body;

pub use file_body::FileBody;
use reqwest::blocking::Client;
use std::fmt::Debug;

use crate::{S3Client, S3Result};

pub trait Request: Debug {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A: Request>(&self, request: A) -> S3Result<String> {
        let client = Client::new();
        let response = client
            .post("http://example.com/post")
            .body("the exact body that is sent")
            .send()?;

        println!("put_object request..: {:?}", request);
        println!("put_object response: {:?}", response);
        Ok("dummy result".to_string())
    }
}
