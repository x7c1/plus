use crate::{CommandOutput, CommandResult};
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_extractor::Matcher;
use clap_task::ClapTask;
use futures::executor;
use sabi_s3::core::{S3Bucket, S3Client};
use sabi_s3::operations::get_object;

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
            .about("Retrieves objects from a bucket.")
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
        eprintln!("running {}!", self.name());
        eprintln!("matches: {:#?}", matches);

        let client = S3Client::from_env(S3Bucket::from_string(
            matches.single("bucket").as_required()?,
        ))?;
        let request = get_object::FileRequest::new(
            matches.single("key").as_required()?,
            matches.single("outfile").as_required()?,
        )?;
        let response: get_object::Response = {
            let future = client.get_object(request);
            executor::block_on(future)?
        };
        let content = Content {
            e_tag: response.headers.e_tag.into_string(),
        };
        Ok(CommandOutput::json(content)?)
    }
}

#[derive(Serialize, Deserialize)]
struct Content {
    #[serde(rename = "ETag")]
    e_tag: String,
}
