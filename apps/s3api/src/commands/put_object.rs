use crate::commands::Definition;
use crate::S3ApiResult;
use clap::{App, Arg, ArgMatches, SubCommand};

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/put-object.html

const COMMAND_NAME: &str = "put-object";

pub fn create<'a, 'b>() -> Definition<'a, 'b, S3ApiResult<()>> {
    Definition {
        name: COMMAND_NAME.to_string(),
        define,
        run,
    }
}

fn define<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about("Adds an object to a bucket")
        .arg(
            Arg::with_name("bucket")
                .long("bucket")
                .required(true)
                .takes_value(true)
                .help("Bucket name to which the PUT operation was initiated."),
        )
        .arg(
            Arg::with_name("key")
                .long("key")
                .required(true)
                .takes_value(true)
                .help("Object key for which the PUT operation was initiated."),
        )
}

fn run(matches: &ArgMatches) -> S3ApiResult<()> {
    println!("running put-object!");
    println!("matches: {:#?}", matches);
    Ok({})
}
