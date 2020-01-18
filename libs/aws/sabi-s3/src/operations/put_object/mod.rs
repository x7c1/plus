mod put_file;
pub use put_file::PutFile;

use crate::S3Client;
use std::fmt::Debug;

pub trait Request: Debug {}

//todo: use type like ClientResult<Response<PutObjectResult>>
type PseudoResponse = String;

pub trait Requester {
    fn put_object<A: Request>(&self, request: A) -> PseudoResponse;
}

impl Requester for S3Client {
    fn put_object<A: Request>(&self, request: A) -> String {
        println!("put_object request: {:?}", request);
        "dummy result".to_string()
    }
}

//type Hoge = crate__operations__put_object__Requester;

macro_rules! define__put_object__requester__methods {
    () => {
        pub fn put_object<A: operations::put_object::Request>(&self, request: A) -> String {
            operations::put_object::Requester::put_object(self, request)
        }
    }
}
