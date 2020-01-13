use super::{CommandResult, ResponseSummary};
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_task::Definition;

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/put-object.html

const COMMAND_NAME: &str = "put-object";

pub fn create<'a, 'b>() -> Definition<'a, 'b, CommandResult> {
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
        .arg(
            Arg::with_name("body")
                .long("body")
                .required(false)
                .takes_value(true)
                .help("Object data."),
        )
}

fn run(matches: &ArgMatches) -> CommandResult {
    println!("running {}!", COMMAND_NAME);
    println!("matches: {:#?}", matches);
    Ok(ResponseSummary::empty())
}
