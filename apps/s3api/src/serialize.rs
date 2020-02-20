use crate::S3ApiResult;
use serde::Serialize;

pub trait AwsJsonSerialize: Serialize {
    /// generate JSON String which has indents with 4 spaces
    /// according to the behavior of AWS CLI.
    fn to_aws_json(&self) -> S3ApiResult<String> {
        let mut serializer = {
            let inner = Vec::<u8>::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            serde_json::Serializer::with_formatter(inner, formatter)
        };
        let json = {
            self.serialize(&mut serializer)?;
            String::from_utf8(serializer.into_inner())?
        };
        Ok(json)
    }
}

impl<A> AwsJsonSerialize for A where A: Serialize {}
