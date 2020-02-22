use reqwest::header::ToStrError;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "ETag header not found.")]
    ETagHeaderNotFound,

    #[fail(display = "ETag header has invalid characters > {:?}", 0)]
    InvalidETagHeader(ToStrError),
}
