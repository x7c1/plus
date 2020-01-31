use crate::auth::v4::canonical::{
    CanonicalHeaders, CanonicalQueryString, CanonicalUri, HashedPayload, SignedHeaders,
};
use hex::ToHex;
use http::Method;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct CanonicalRequest {
    pub method: Method,
    pub uri: CanonicalUri,
    pub query_string: CanonicalQueryString,
    pub signed_headers: SignedHeaders,
    pub canonical_headers: CanonicalHeaders,
    pub hashed_payload: HashedPayload,
}

impl CanonicalRequest {
    pub fn to_hash(&self) -> String {
        let mut hasher = Sha256::default();
        hasher.input(self.combine().as_bytes());
        hasher.result().as_slice().encode_hex()
    }

    fn combine(&self) -> String {
        format!(
            "{method}\n{uri}\n{query_string}\n{canonical_headers}\n\n{signed_headers}\n{hash}",
            method = self.method.as_str(),
            uri = self.uri.as_str(),
            query_string = self.query_string.as_str(),
            canonical_headers = self.canonical_headers.as_str(),
            signed_headers = self.signed_headers.as_str(),
            hash = self.hashed_payload.as_str(),
        )
    }
}
