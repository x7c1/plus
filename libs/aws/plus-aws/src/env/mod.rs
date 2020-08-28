pub mod aws {
    use env_extractor::EnvVar;

    pub fn default_region() -> EnvVar {
        EnvVar::new("AWS_DEFAULT_REGION")
    }
    pub fn access_key() -> EnvVar {
        EnvVar::new("AWS_ACCESS_KEY_ID")
    }
    pub fn secret_key() -> EnvVar {
        EnvVar::new("AWS_SECRET_ACCESS_KEY")
    }
}
