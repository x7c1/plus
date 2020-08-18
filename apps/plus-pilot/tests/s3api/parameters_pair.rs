pub struct ParametersPair<A> {
    pub plus: A,
    pub aws: A,
}

impl<A> ParametersPair<A> {
    pub fn as_vec(&self) -> Vec<&A> {
        vec![&self.plus, &self.aws]
    }
}
