/// > The date must be in the YYYYMMDD format.
/// > Note that the date does not include a time value.
///
/// ## Example
///  * 20150830
///
/// ## See also
///  * [Task 2: Create a String to Sign for Signature Version 4 - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html)
///
#[derive(Debug)]
pub struct DateStamp(String);

impl DateStamp {
    pub fn new<A: Into<String>>(key: A) -> DateStamp {
        DateStamp(key.into())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'a> Into<&'a [u8]> for &'a DateStamp {
    fn into(self) -> &'a [u8] {
        self.0.as_bytes()
    }
}
