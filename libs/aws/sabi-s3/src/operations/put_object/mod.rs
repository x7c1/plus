mod put_file;
pub use put_file::PutFile;

use crate::S3Client;
use std::fmt::Debug;

pub trait Request: Debug {}

//todo: use type like ClientResult<Response<PutObjectResult>>
type PseudoResponse = String;

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
