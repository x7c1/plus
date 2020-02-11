mod error;
pub use error::Error;

pub mod aws {
    use env_extractor::FromSingle;

    pub fn default_region() -> FromSingle {
        FromSingle::new("AWS_DEFAULT_REGION")
    }
    pub fn access_key() -> FromSingle {
        FromSingle::new("AWS_ACCESS_KEY_ID")
    }
    pub fn secret_key() -> FromSingle {
        FromSingle::new("AWS_SECRET_ACCESS_KEY")
    }
}
