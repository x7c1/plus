use shellwork::core::command::RunnerOutput;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct ActionOutput<A> {
    raw: RunnerOutput,
    _phantom: PhantomData<A>,
}

impl<A> ActionOutput<A> {
    pub fn new(raw: RunnerOutput) -> ActionOutput<A> {
        ActionOutput {
            raw,
            _phantom: PhantomData,
        }
    }
    pub fn stdout(&self) -> Cow<'_, str> {
        self.raw.stdout()
    }
}
