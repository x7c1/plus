use shellwork::core::command;
use shellwork::core::command::{CanDefine, Runner, Unprepared, UnsupportedReport};

pub struct RunMaybe<'a, A>(&'a A);

impl<A> RunMaybe<'_, A> {
    pub fn new(a: &A) -> RunMaybe<A> {
        RunMaybe(a)
    }
}

impl<A: CanDefine> CanDefine for RunMaybe<'_, A> {
    type Params = A::Params;
    type Err = A::Err;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        self.0.define(params)
    }
}

impl<A: CanDefine> command::MayRun for RunMaybe<'_, A> {
    fn unsupported(&self) -> Option<UnsupportedReport> {
        // todo: check if sdk exists
        None
    }
}
