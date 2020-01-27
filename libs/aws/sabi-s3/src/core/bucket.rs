#[derive(Debug)]
pub struct S3Bucket(String);

impl S3Bucket {
    pub fn new<A: Into<String>>(name: A) -> S3Bucket {
        S3Bucket(name.into())
    }
    pub fn from_string(name: String) -> S3Bucket {
        S3Bucket(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
