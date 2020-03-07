pub struct ParametersPair<A> {
    pub wsb: A,
    pub aws: A,
}

impl<A> ParametersPair<A> {
    pub fn as_vec(&self) -> Vec<&A> {
        vec![&self.wsb, &self.aws]
    }
}
