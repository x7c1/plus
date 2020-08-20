use crate::core::S3Bucket;
use plus_aws::auth::Credentials;
use plus_aws::index::RegionCode;

pub trait HasBucketScope {
    fn credentials(&self) -> &Credentials;
    fn bucket(&self) -> &S3Bucket;
    fn default_region(&self) -> &Option<RegionCode>;
}
