use crate::core::S3Bucket;
use sabi_core::auth::Credentials;
use sabi_core::index::RegionCode;

pub trait HasBucketScope {
    fn credentials(&self) -> &Credentials;
    fn bucket(&self) -> &S3Bucket;
    fn default_region(&self) -> &Option<RegionCode>;
}
