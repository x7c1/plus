use url::Url;

#[derive(Debug)]
pub struct CanonicalQueryString(String);

impl CanonicalQueryString {
    pub fn new<A: Into<String>>(value: A) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&Url> for CanonicalQueryString {
    fn from(url: &Url) -> Self {
        /*
            rf.
            https://github.com/durch/rust-s3/blob/a2b6879f3be920d61394d2c5ebecdfc779d20713/src/signing.rs#L43-L50
        */
        let mut keyvalues = url
            .query_pairs()
            .map(|(key, value)| uri_encode(&key, true) + "=" + &uri_encode(&value, true))
            .collect::<Vec<String>>();

        keyvalues.sort();
        CanonicalQueryString::new(keyvalues.join("&"))
    }
}

/// Encode a URI following the specific requirements of the AWS service.
fn uri_encode(string: &str, encode_slash: bool) -> String {
    /*
        rf.
        https://github.com/durch/rust-s3/blob/a2b6879f3be920d61394d2c5ebecdfc779d20713/src/signing.rs#L18-L36
    */
    let mut result = String::with_capacity(string.len() * 2);
    for c in string.chars() {
        match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' | '-' | '~' | '.' => result.push(c),
            '/' if encode_slash => result.push_str("%2F"),
            '/' if !encode_slash => result.push('/'),
            _ => {
                result.push('%');
                result.push_str(
                    &format!("{}", c)
                        .bytes()
                        .map(|b| format!("{:02X}", b))
                        .collect::<String>(),
                );
            }
        }
    }
    result
}
