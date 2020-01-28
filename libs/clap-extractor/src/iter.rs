pub trait FallibleResults<T, E> {
    fn complete_or_failed(self) -> Result<Vec<T>, E>;
}

impl<A, T, E> FallibleResults<T, E> for A
where
    A: Iterator<Item = Result<T, E>>,
{
    fn complete_or_failed(self) -> Result<Vec<T>, E> {
        let mut items = vec![];
        for item in self {
            match item {
                Ok(i) => items.push(i),
                Err(e) => return Err(e),
            }
        }
        Ok(items)
    }
}
