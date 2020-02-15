use crate::core::S3Result;
use crate::internal::{RequestProvider, ResourceLoader};
use crate::verbs::HasObjectKey;
use reqwest::blocking::{Client, Response};
use std::fmt::Debug;
use std::time::Duration;

#[derive(Debug)]
pub struct InternalClient {}

impl InternalClient {
    pub fn new() -> InternalClient {
        InternalClient {}
    }

    pub fn request_by<A>(&self, provider: RequestProvider<A>) -> S3Result<String>
    where
        A: ResourceLoader,
        A: HasObjectKey,
    {
        let request = provider.provide()?;
        println!("request > {:#?}", request);

        let builder = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?
            .request(request.method, request.url)
            .headers(request.headers);

        let builder = match request.body {
            Some(body) => builder.body(body),
            _ => builder,
        };
        let response: Response = builder.send()?;

        println!("response > {:#?}", response);
        Ok("dummy result".to_string())
    }
}
