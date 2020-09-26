#[derive(Debug)]
pub enum PackageName {
    EnvExtractor,
    S3Api,
}

impl PackageName {
    pub fn as_str(&self) -> &str {
        match self {
            PackageName::EnvExtractor => "env-extractor",
            PackageName::S3Api => "s3api",
        }
    }
}
