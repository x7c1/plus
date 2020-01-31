/*
    see also:
    https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
*/

mod uri;
pub use uri::CanonicalUri;

mod hashed_payload;
pub use hashed_payload::HashedPayload;
