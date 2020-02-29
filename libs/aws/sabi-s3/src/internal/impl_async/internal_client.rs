use super::{RequestProvider, ResourceLoader};
use crate::core::verbs::HasObjectKey;
use crate::internal;
use crate::internal::error::Error::S3Error;
use crate::internal::impl_async::S3ErrorResponse;
use reqwest::{Client, Response, StatusCode};
use std::fmt::Debug;
use std::time::Duration;

#[derive(Debug)]
pub struct InternalClient {}

impl InternalClient {
    pub fn new() -> InternalClient {
        InternalClient {}
    }

    pub async fn request_by<'a, A>(
        &'a self,
        provider: RequestProvider<'a, A>,
    ) -> internal::Result<Response>
    where
        A: ResourceLoader,
        A: HasObjectKey,
    {
        let request = provider.provide().await?;
        eprintln!("request > {:#?}", request);

        let client: Client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| internal::Error::ReqwestError(e))?;

        let builder = client
            .request(request.method, request.url)
            .headers(request.headers);

        let builder = match request.body {
            Some(body) => builder.body(body),
            _ => builder,
        };
        let response: reqwest::Response = builder
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| internal::Error::ReqwestError(e))?;

        let status: StatusCode = response.status();
        if status.is_success() {
            eprintln!("response > {:#?}", response);
            Ok(response)
        } else {
            Err(S3Error(S3ErrorResponse::dump(response).await?))
        }
    }
}
