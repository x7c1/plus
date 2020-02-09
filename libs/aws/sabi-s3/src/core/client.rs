use crate::core::{S3Bucket, S3Result};
use crate::operations;
use sabi_core::auth::Credentials;
use sabi_core::index::RegionCode;

#[derive(Debug)]
pub struct S3Client {
    pub credentials: Credentials,
    pub default_region: Option<RegionCode>,
    pub bucket: S3Bucket,
}

impl S3Client {
    pub fn from_env(bucket: S3Bucket) -> S3Result<S3Client> {
        Ok(S3Client {
            credentials: Credentials::from_env()?,
            default_region: RegionCode::from_env()?,
            bucket,
        })
    }

    pub async fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: operations::put_object::Request,
    {
        operations::put_object::Requester::put_object(self, request)
    }
}
