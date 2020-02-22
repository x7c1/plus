mod bucket;
pub use bucket::S3Bucket;

mod error;
pub use error::Error;
pub use error::Result;

mod e_tag;
pub use e_tag::ETag;

pub mod headers;
pub use headers::S3HeaderMap;

pub mod verbs;
