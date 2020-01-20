mod file_body;
pub use file_body::FileBody;

use crate::S3Client;
use std::fmt::Debug;

pub trait Request: Debug {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> String
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A: Request>(&self, request: A) -> String {
        println!("put_object request: {:?}", request);
        "dummy result".to_string()
    }
}
