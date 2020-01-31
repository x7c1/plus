/*
    see also:
    https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
*/

mod canonical_headers;

mod uri;
pub use uri::CanonicalUri;

mod hashed_payload;
pub use hashed_payload::HashedPayload;

mod query;
pub use query::CanonicalQueryString;
