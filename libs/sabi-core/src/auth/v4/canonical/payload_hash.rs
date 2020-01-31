use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct PayloadHash(String);

impl PayloadHash {
    pub fn new<A: Into<String>>(value: A) -> PayloadHash {
        PayloadHash(value.into())
    }

    pub fn empty() -> PayloadHash {
        PayloadHash::new(
            /*
                rf.
                https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
            */
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        )
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&File> for PayloadHash {
    fn from(file: &File) -> Self {
        unimplemented!()
    }
}
