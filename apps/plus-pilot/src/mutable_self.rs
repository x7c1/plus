pub trait MutableSelf: Sized {
    fn mutate<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Self),
    {
        f(&mut self);
        self
    }
}

impl<A: Sized> MutableSelf for A {}
