use reqwest::header::ToStrError;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "header not found. target: {}", 0)]
    HeaderNotFound(String),

    #[fail(
        display = "invalid character(s) found. name: {}, cause > {}",
        name, cause
    )]
    InvalidCharacters { name: String, cause: ToStrError },
}
