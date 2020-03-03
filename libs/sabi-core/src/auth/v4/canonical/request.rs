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
    pub fn as_hash(&self) -> &str {
        &self.hash
    }
}

pub struct HeadersCapturer<'a> {
    pub url: &'a Url,
    pub method: &'a Method,
    pub hashed_payload: &'a HashedPayload,
}

impl HeadersCapturer<'_> {
    pub fn capture(&self, headers: &HeaderMap) -> CanonicalRequest {
        let signed_headers = SignedHeaders::from(headers);
        let combined = format!(
            "{method}\n{uri}\n{query_string}\n{canonical_headers}\n\n{signed_headers}\n{hash}",
            method = self.method.as_str(),
            uri = CanonicalUri::from(self.url).as_str(),
            query_string = CanonicalQueryString::from(self.url).as_str(),
            canonical_headers = CanonicalHeaders::from(headers).as_str(),
            signed_headers = signed_headers.as_str(),
            hash = self.hashed_payload.as_str(),
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
}
