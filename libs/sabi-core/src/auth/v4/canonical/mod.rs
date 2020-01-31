/*
    see also:
    https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
*/

mod canonical_headers;
pub use canonical_headers::CanonicalHeaders;

mod request;
pub use request::CanonicalRequest;

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
    use crate::SabiResult;
    use http::{HeaderMap, Method};
    use url::Url;

    #[test]
    fn it_works() -> SabiResult<()> {
        let url =
            Url::parse("https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08").unwrap();

        let headers = to_headers(&url)?;
        let hash = HashedPayload::empty();
        let request = CanonicalRequest::from(&Method::GET, &url, &headers, hash);

        assert_eq!(
            request.to_hash(),
            "f536975d06c0309214f805bb90ccff089219ecd68b2577efef23edd43b7e1a59"
        );
        Ok({})
    }

    fn to_headers(url: &Url) -> SabiResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-type",
            "application/x-www-form-urlencoded; charset=utf-8"
                .parse()
                .unwrap(),
        );
        headers.insert("host", url.host_str().unwrap().parse().unwrap());
        headers.insert("x-amz-date", "20150830T123600Z".parse().unwrap());
        Ok(headers)
    }
}
