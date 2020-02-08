use crate::core::S3Result;
use crate::internal::{RequestProvider, ResourceProvider};
use crate::verbs::HasObjectKey;
use reqwest::blocking::Client;
use std::fmt::Debug;

#[derive(Debug)]
pub struct InternalClient {}

impl InternalClient {
    pub fn new() -> InternalClient {
        InternalClient {}
    }

    pub fn request_by<A>(&self, provider: RequestProvider<A>) -> S3Result<String>
    where
        A: ResourceProvider,
        A: HasObjectKey,
    {
        let request = provider.provide()?;
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
