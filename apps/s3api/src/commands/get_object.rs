use crate::{CommandResult, ResponseSummary};
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_task::ClapTask;

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/get-object.html

pub fn define() -> Box<dyn ClapTask<CommandResult>> {
    Box::new(Task)
}

struct Task;

impl ClapTask<CommandResult> for Task {
    fn name(&self) -> &str {
        "get-object"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
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

    fn run(&self, matches: &ArgMatches) -> CommandResult {
        println!("running {}!", self.name());
        println!("matches: {:#?}", matches);
        Ok(ResponseSummary::empty())
    }
}
