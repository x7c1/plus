pub mod aws {
    use env_extractor::{env_var, EnvVar};

    pub fn default_region() -> EnvVar {
        env_var("AWS_DEFAULT_REGION")
    }
    pub fn access_key() -> EnvVar {
        env_var("AWS_ACCESS_KEY_ID")
    }
    pub fn secret_key() -> EnvVar {
        env_var("AWS_SECRET_ACCESS_KEY")
    }
}
