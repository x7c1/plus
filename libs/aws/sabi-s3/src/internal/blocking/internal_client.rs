use crate::core::verbs::HasObjectKey;
use crate::internal;
use crate::internal::blocking::{RequestProvider, ResourceLoader};
use reqwest::blocking::{Client, Response};
use std::fmt::Debug;
use std::time::Duration;

#[derive(Debug)]
pub struct InternalClient {}

impl InternalClient {
    pub fn new() -> InternalClient {
        InternalClient {}
    }

    pub fn request_by<A>(&self, provider: RequestProvider<A>) -> internal::Result<Response>
    where
        A: ResourceLoader,
        A: HasObjectKey,
    {
        let request = provider.provide()?;
        eprintln!("request > {:#?}", request);

        let client: Client = Client::builder().timeout(Duration::from_secs(5)).build()?;
        let builder = client
            .request(request.method, request.url)
            .headers(request.headers);

        let builder = match request.body {
            Some(body) => builder.body(body),
            _ => builder,
        };
        let response: Response = builder.timeout(Duration::from_secs(5)).send()?;

        eprintln!("response > {:#?}", response);
        Ok(response)
    }
}
