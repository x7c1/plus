use failure::_core::marker::PhantomData;
use reqwest::Method;

pub trait HasMethod<MARKER> {
    const METHOD: Method;
}

pub trait GetRequest<A> {}

pub struct GetMethod<A>(PhantomData<A>);

impl<A, B: GetRequest<A>> HasMethod<GetMethod<A>> for B {
    const METHOD: Method = Method::GET;
}

pub trait PostRequest<A> {}

pub struct PostMethod<A>(PhantomData<A>);

impl<A, B: PostRequest<A>> HasMethod<PostMethod<A>> for B {
    const METHOD: Method = Method::POST;
}
