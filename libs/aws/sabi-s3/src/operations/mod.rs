mod client;
pub use client::Error as S3ClientError;
pub use client::S3Client;

pub mod get_object;
pub mod put_object;
