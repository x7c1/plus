use http::header::IntoHeaderName;

pub struct HeaderFragment<K, V>
where
    K: IntoHeaderName,
{
    pub key: K,
    pub value: V,
}

impl<'a> Into<HeaderFragment<&'static str, &'a str>> for (&'static str, &'a str) {
    fn into(self) -> HeaderFragment<&'static str, &'a str> {
        let (key, value) = self;
        HeaderFragment { key, value }
    }
}
