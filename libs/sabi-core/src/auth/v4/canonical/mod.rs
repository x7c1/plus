/*
    see also:
    https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
*/

mod canonical_headers;
pub use canonical_headers::CanonicalHeaders;

mod request;
pub use request::CanonicalRequest;
pub use request::HeadersCapturer;

mod uri;
pub use uri::CanonicalUri;

mod hashed_payload;
pub use hashed_payload::HashedPayload;

mod query;
pub use query::CanonicalQueryString;

mod signed_headers;
pub use signed_headers::SignedHeaders;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::request::RichHeaderMap;
    use crate::SabiResult;
    use http::{HeaderMap, Method};
    use url::Url;

    #[test]
    fn it_works() -> SabiResult<()> {
        let url =
            Url::parse("https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08").unwrap();

        let capturer = HeadersCapturer {
            url: &url,
            method: &Method::GET,
            hashed_payload: &HashedPayload::empty(),
        };
        let request = capturer.capture(&to_headers(&url)?);

        assert_eq!(
            request.as_hash(),
            "f536975d06c0309214f805bb90ccff089219ecd68b2577efef23edd43b7e1a59"
        );
        Ok({})
    }

    fn to_headers(url: &Url) -> SabiResult<HeaderMap> {
        let headers = HeaderMap::new()
            .push(("host", url.host_str().unwrap()))?
            .push((
                "content-type",
                "application/x-www-form-urlencoded; charset=utf-8",
            ))?
            .push(("x-amz-date", "20150830T123600Z"))?;

        Ok(headers)
    }
}
