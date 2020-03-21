use crate::core::targets::{BuildTarget, RequireCC};

pub struct Params<T: BuildTarget> {
    pub target: T,
    output_format: OutputFormat,
}

pub enum OutputFormat {
    Default,
    FileName,
}

impl<T: BuildTarget> Params<T> {
    pub fn builder(_format: OutputFormat) -> ParamsBuilder<T> {
        ParamsBuilder {
            target: None,
            format: OutputFormat::Default,
        }
    }
}

impl<T> RequireCC for Params<T>
where
    T: RequireCC + BuildTarget,
{
    const CC: &'static str = T::CC;
}

pub struct ParamsBuilder<T: BuildTarget> {
    target: Option<T>,
    format: OutputFormat,
}

impl<T> ParamsBuilder<T>
where
    T: BuildTarget,
{
    pub fn target(mut self, target: T) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params<T> {
        Params {
            target: self.target.expect("target is required"),
            output_format: self.format,
        }
    }
}
