mod error;
pub use error::Error;
pub use error::Result;

use crate::actions;
use crate::actions::{get_object, put_object};
use crate::core::verbs::HasBucketScope;
use crate::core::S3Bucket;
use plus_aws::auth::Credentials;
use plus_aws::index::RegionCode;
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
            credentials: Credentials::from_env().map_err(CredentialsError)?,
            default_region: RegionCode::find_from_env().map_err(RegionCodeError)?,
            bucket,
        })
    }

    pub async fn put_object<A>(&self, request: A) -> actions::Result<put_object::Response>
    where
        A: put_object::Request,
    {
        put_object::Requester::put_object(self, request).await
    }

    pub async fn get_object<A>(&self, request: A) -> actions::Result<get_object::Response>
    where
        A: get_object::Request,
    {
        actions::get_object::Requester::get_object(self, request).await
    }
}
