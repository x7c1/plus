mod bucket;
pub use bucket::S3Bucket;

mod client;
pub use client::S3Client;

pub use super::error::Result as S3Result;

mod e_tag;
pub use e_tag::ETag;

pub mod headers;
pub use headers::S3HeaderMap;
