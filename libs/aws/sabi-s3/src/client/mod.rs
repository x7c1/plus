mod error;
pub use error::Error;
pub use error::Result;

use crate::actions;
use crate::core::verbs::HasBucketScope;
use crate::core::S3Bucket;
use sabi_core::auth::Credentials;
use sabi_core::index::RegionCode;
use Error::{CredentialsError, RegionCodeError};

#[derive(Debug)]
pub struct S3Client {
    pub credentials: Credentials,
    pub default_region: Option<RegionCode>,
    pub bucket: S3Bucket,
}

impl HasBucketScope for &S3Client {
    fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    fn bucket(&self) -> &S3Bucket {
        &self.bucket
    }

    fn default_region(&self) -> &Option<RegionCode> {
        &self.default_region
    }
}

impl S3Client {
    pub fn from_env(bucket: S3Bucket) -> Result<S3Client> {
        Ok(S3Client {
            credentials: Credentials::from_env().map_err(|e| CredentialsError(e))?,
            default_region: RegionCode::find_from_env().map_err(|e| RegionCodeError(e))?,
            bucket,
        })
    }

    pub async fn put_object<A>(&self, request: A) -> actions::Result<actions::put_object::Response>
    where
        A: actions::put_object::Request,
    {
        actions::put_object::Requester::put_object(self, request)
    }

    pub async fn get_object<A>(&self, request: A) -> actions::Result<actions::get_object::Response>
    where
        A: actions::get_object::Request,
    {
        actions::get_object::Requester::get_object(self, request)
    }
}