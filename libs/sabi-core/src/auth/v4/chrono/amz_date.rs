/// ## Example
///  * 20150830T123600Z
///
/// ## See also
///  * [Handling Dates in Signature Version 4 - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/sigv4-date-handling.html)
#[derive(Debug)]
pub struct AmzDate(String);

impl AmzDate {
    pub fn new<A: Into<String>>(value: A) -> AmzDate {
        AmzDate(value.into())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
