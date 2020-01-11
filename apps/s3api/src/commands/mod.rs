use crate::S3ApiResult;
use clap::{App, ArgMatches};

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/

pub mod put_object;

pub struct Definition<'a, 'b> {
    pub name: String,
    pub define: fn() -> App<'a, 'b>,
    pub run: fn(matches: &ArgMatches) -> S3ApiResult<()>,
}

pub struct Task<'a, 'b> {
    pub definition: &'a Definition<'a, 'b>,
    pub matches: &'a ArgMatches<'a>,
}

impl Task<'_, '_> {
    pub fn run(&self) -> S3ApiResult<()> {
        let run = self.definition.run;
        run(self.matches)
    }
}
