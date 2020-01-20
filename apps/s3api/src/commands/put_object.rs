use crate::{CommandResult, ResponseSummary};
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_extractor::Extractor;
use clap_task::Definition;
use sabi_s3::operations::put_object::FileBody;
use sabi_s3::S3Client;

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/put-object.html

const COMMAND_NAME: &str = "put-object";

pub fn define<'a, 'b>() -> Definition<'a, 'b, CommandResult> {
    Definition {
        name: COMMAND_NAME.to_string(),
        create,
        run,
    }
}

fn create<'a, 'b>() -> App<'a, 'b> {
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
    let extractor = Extractor::new(matches);

    let client = S3Client {};
    let request = FileBody {
        file_path: extractor.single("body").as_required()?,
    };
    client.put_object(request);

    Ok(ResponseSummary::empty())
}
