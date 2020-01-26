mod file_body;

use crate::{S3Client, S3Result};
pub use file_body::FileBody;

use hyper::body::{Bytes, HttpBody};
use hyper::{Body, Client, Method, StatusCode, Uri};
use hyper_rustls::HttpsConnector;
use std::fmt::Debug;
use tokio::runtime::Runtime;

pub trait Request: Debug {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A: Request>(&self, request: A) -> S3Result<String> {
        let request = hyper::Request::builder()
            .method(Method::POST)
            .uri("http://httpbin.org/post")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"library":"hyper"}"#))?;

        let mut runtime = Runtime::new()?;
        let client = Client::new();
        let response = runtime.block_on(client.request(request))?;
        println!("put_object response: {:?}", response.status());

        let bytes_future = hyper::body::to_bytes(response.into_body());
        let bytes: Bytes = runtime.block_on(bytes_future)?;
        let body = String::from_utf8(bytes.to_vec())?;
        println!("put_object body: {}", body);
        Ok(body)

        /*
        let mut runtime = Runtime::new()?;
        let url = ("https://hyper.rs").parse()?;
        let https = HttpsConnector::new();

        let client: Client<_, hyper::Body> = Client::builder().build(https);

        let mut response = runtime.block_on(client.get(url))?;
        println!("put_object request..: {:?}", request);
        println!("put_object response: {:?}", response.body());

        let bytes_future = hyper::body::to_bytes(response.into_body());
        let bytes: Bytes = runtime.block_on(bytes_future)?;
        let body = String::from_utf8(bytes.to_vec())?;
        println!("put_object body: {}", body);

        Ok("dummy result".to_string())
        */
    }
}
