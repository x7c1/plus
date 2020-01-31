use crate::core::S3Result;
use crate::internal::InternalRequest;
use reqwest::blocking::Client;
use std::fmt::Debug;

#[derive(Debug)]
pub struct InternalClient {}

impl InternalClient {
    pub fn new() -> InternalClient {
        InternalClient {}
    }

    pub fn send<A>(&self, request_like: A) -> S3Result<String>
    where
        A: Into<S3Result<InternalRequest>>,
    {
        let request = request_like.into()?;
        println!("request > {:#?}", request);

        let builder = Client::new()
            .request(request.method, request.url)
            .headers(request.headers);

        let builder = match request.body {
            Some(body) => builder.body(body),
            _ => builder,
        };
        let response = builder.send()?;

        println!("response > {:#?}", response);
        Ok("dummy result".to_string())
    }
}
