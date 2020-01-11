use crate::S3ApiResult;
use clap::{App, ArgMatches};

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/

pub mod put_object;

pub struct Task<'a, 'b> {
    pub name: String,
    pub create: fn() -> App<'a, 'b>,
    pub run: fn(matches: &ArgMatches) -> S3ApiResult<()>,
}
