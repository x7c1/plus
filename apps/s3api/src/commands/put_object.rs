use crate::{CommandResult, ResponseSummary};
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_extractor::Matcher;
use clap_task::ClapTask;
use sabi_s3::operations::put_object::FileBody;
use sabi_s3::S3Client;

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/put-object.html

const COMMAND_NAME: &str = "put-object";

pub fn define() -> Box<dyn ClapTask<CommandResult>> {
    Box::new(Task)
}

struct Task;

impl ClapTask<CommandResult> for Task {
    fn design(&self) -> App {
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

    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn run(&self, matches: &ArgMatches) -> CommandResult {
        println!("running {}!", COMMAND_NAME);
        println!("matches: {:#?}", matches);

        let client = S3Client {};
        let request = FileBody {
            file_path: matches.single("body").as_required()?,
        };
        client.put_object(request);

        Ok(ResponseSummary::empty())
    }
}
