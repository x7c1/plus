mod bucket;
pub use bucket::S3Bucket;

mod error;
pub use error::Error;
pub use error::Result;

pub mod response;
pub mod verbs;
