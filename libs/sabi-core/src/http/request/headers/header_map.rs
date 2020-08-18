use crate::auth::v4::request::AuthorizationFactory;
use crate::http::request::ToHeaderFragment;
use crate::Error::HostNotFound;
use crate::PlusResult;
use http::header::{AUTHORIZATION, HOST};
use http::HeaderMap;
use url::Url;

pub trait RichHeaderMap: Sized {
    fn push<A>(self, header: A) -> PlusResult<Self>
    where
        A: ToHeaderFragment;

    fn push_if_exists<A>(self, header: Option<A>) -> PlusResult<Self>
    where
        A: ToHeaderFragment;

    fn host(self, url: &Url) -> PlusResult<Self>;

    fn authorize_with(self, factory: AuthorizationFactory) -> PlusResult<Self>;
}

impl RichHeaderMap for HeaderMap {
    fn push<A>(mut self, header: A) -> PlusResult<Self>
    where
        A: ToHeaderFragment,
    {
        let fragment = header.into()?;
        self.insert(fragment.key, fragment.value);
        Ok(self)
    }

    fn push_if_exists<A>(self, header: Option<A>) -> PlusResult<Self>
    where
        A: ToHeaderFragment,
    {
        if let Some(header) = header {
            self.push(header)
        } else {
            Ok(self)
        }
    }

    fn host(mut self, url: &Url) -> PlusResult<Self> {
        let host = url
            .host_str()
            .ok_or_else(|| HostNotFound(url.clone()))?
            .parse()?;

        self.insert(HOST, host);
        Ok(self)
    }

    fn authorize_with(mut self, factory: AuthorizationFactory) -> PlusResult<Self> {
        self.insert(AUTHORIZATION, factory.create_from(&self).to_header_value()?);
        Ok(self)
    }
}
