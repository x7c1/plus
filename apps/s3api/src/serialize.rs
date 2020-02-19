use crate::S3ApiResult;
use serde::Serialize;

pub trait AwsJsonSerialize: Serialize {
    /// generate JSON String which has indents with 4 spaces
    /// according to the behavior of AWS CLI.
    fn to_aws_json(&self) -> S3ApiResult<String> {
        let mut serializer = {
            let buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            serde_json::Serializer::with_formatter(buf, formatter)
        };
        self.serialize(&mut serializer)?;

        let json = String::from_utf8(serializer.into_inner()).unwrap();
        Ok(json)
    }
}

impl<A> AwsJsonSerialize for A where A: Serialize {}
