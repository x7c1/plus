use crate::{CommandResult, ResponseSummary};
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_task::Definition;

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/get-object.html

const COMMAND_NAME: &str = "get-object";

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
                .help("The bucket name containing the object."),
        )
        .arg(
            Arg::with_name("key")
                .long("key")
                .required(true)
                .takes_value(true)
                .help("Key of the object to get."),
        )
        .arg(
            Arg::with_name("outfile")
                .required(true)
                .takes_value(true)
                .help("Filename where the content will be saved."),
        )
}

fn run(matches: &ArgMatches) -> CommandResult {
    println!("running {}!", COMMAND_NAME);
    println!("matches: {:#?}", matches);
    Ok(ResponseSummary::empty())
}
