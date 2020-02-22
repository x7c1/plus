mod error;
pub use error::Error;

use crate::core::S3Bucket;
use crate::operations;
use crate::operations::client::error::Error::{CredentialsError, RegionCodeError};
use sabi_core::auth::Credentials;
use sabi_core::index::RegionCode;
use sabi_core::io::BodyReceiver;

#[derive(Debug)]
pub struct S3Client {
    pub credentials: Credentials,
    pub default_region: Option<RegionCode>,
    pub bucket: S3Bucket,
}

impl S3Client {
    pub fn from_env(bucket: S3Bucket) -> crate::Result<S3Client> {
        Ok(S3Client {
            credentials: Credentials::from_env().map_err(|e| CredentialsError(e))?,
            default_region: RegionCode::find_from_env().map_err(|e| RegionCodeError(e))?,
            bucket,
        })
    }

    pub async fn put_object<A>(&self, request: A) -> crate::Result<operations::put_object::Response>
    where
        A: operations::put_object::Request,
    {
        operations::put_object::Requester::put_object(self, request)
    }

    pub async fn get_object<A>(&self, request: A) -> crate::Result<operations::get_object::Response>
    where
        A: operations::get_object::Request,
        crate::Error: From<<A as BodyReceiver>::Err>,
    {
        operations::get_object::Requester::get_object(self, request)
    }
}
