pub mod aws {
    use env_extractor::SingleValue;

    pub fn default_region() -> SingleValue {
        SingleValue::new("AWS_DEFAULT_REGION")
    }
    pub fn access_key() -> SingleValue {
        SingleValue::new("AWS_ACCESS_KEY_ID")
    }
    pub fn secret_key() -> SingleValue {
        SingleValue::new("AWS_SECRET_ACCESS_KEY")
    }
}
