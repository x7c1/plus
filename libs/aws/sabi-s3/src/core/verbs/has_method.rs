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

pub struct PostImpl<A>(PhantomData<A>);

pub trait IsPost<MARKER> {}

impl<A, B: IsPost<A>> HasMethod<PostImpl<A>> for B {
    const METHOD: Method = Method::POST;
}
