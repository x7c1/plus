use reqwest::Method;
use std::marker::PhantomData;

pub trait HasMethod<MARKER> {
    const METHOD: Method;
}

pub struct GetImpl<A>(PhantomData<A>);

pub trait IsGet<MARKER> {}

impl<A, B: IsGet<A>> HasMethod<GetImpl<A>> for B {
    const METHOD: Method = Method::GET;
}

pub struct PutImpl<A>(PhantomData<A>);

pub trait IsPut<MARKER> {}

impl<A, B: IsPut<A>> HasMethod<PutImpl<A>> for B {
    const METHOD: Method = Method::PUT;
}
