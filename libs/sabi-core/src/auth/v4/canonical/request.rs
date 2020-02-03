use crate::auth::v4::canonical::{
    CanonicalHeaders, CanonicalQueryString, CanonicalUri, HashedPayload, SignedHeaders,
};
use hex::ToHex;
use http::{HeaderMap, Method};
use sha2::{Digest, Sha256};
use url::Url;

#[derive(Debug)]
pub struct CanonicalRequest {
    pub signed_headers: SignedHeaders,
    hash: String,
}

impl CanonicalRequest {
    pub fn from(method: &Method, url: &Url, headers: &HeaderMap, hash: HashedPayload) -> Self {
        let signed_headers = SignedHeaders::from(headers);
        let combined = format!(
            "{method}\n{uri}\n{query_string}\n{canonical_headers}\n\n{signed_headers}\n{hash}",
            method = method.as_str(),
            uri = CanonicalUri::from(url).as_str(),
            query_string = CanonicalQueryString::from(url).as_str(),
            canonical_headers = CanonicalHeaders::from(headers).as_str(),
            signed_headers = signed_headers.as_str(),
            hash = hash.as_str(),
        );
        let hash = {
            let mut hasher = Sha256::default();
            hasher.input(combined.as_bytes());
            hasher.result().as_slice().encode_hex()
        };
        CanonicalRequest {
            signed_headers,
            hash,
        }
    }
    pub fn as_hash(&self) -> &str {
        &self.hash
    }
}
