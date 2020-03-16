mod cargo;
pub use cargo::build_pilot;
pub use cargo::cargo_build;
use std::marker::PhantomData;

pub mod support;

pub struct Action<A, X>(A, PhantomData<X>);

impl<A, X> Action<A, X> {
    pub fn new(a: A) -> Action<A, X> {
        Action(a, PhantomData)
    }
    pub fn value(&self) -> &A {
        &self.0
    }
}
