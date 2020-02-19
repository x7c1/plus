use crate::{CommandOutput, CommandResult, S3ApiResult};
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_extractor::Matcher;
use clap_task::ClapTask;
use futures::executor;
use sabi_s3::core::{S3Bucket, S3Client};
use sabi_s3::operations::put_object;
use sabi_s3::operations::put_object::FileRequest;
use serde::ser::Serialize;

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/put-object.html

pub fn define() -> Box<dyn ClapTask<CommandResult>> {
    Box::new(Task)
}

struct Task;

impl ClapTask<CommandResult> for Task {
    fn name(&self) -> &str {
        "put-object"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
            .about("Adds an object to a bucket.")
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
            .arg(
                Arg::with_name("content-type")
                    .long("content-type")
                    .required(false)
                    .takes_value(true)
                    .help("A standard MIME type describing the format of the contents."),
            )
            .arg(
                Arg::with_name("region")
                    .long("region")
                    .required(false)
                    .takes_value(true),
            )
    }

    fn run(&self, matches: &ArgMatches) -> CommandResult {
        let client = S3Client::from_env(S3Bucket::from_string(
            matches.single("bucket").as_required()?,
        ))?;
        let request = FileRequest {
            file_path: matches.single("body").as_required()?,
            object_key: matches.single("key").as_required()?,
            content_type: matches.single("content_type").as_optional()?,
            region_code: matches.single("region").as_optional()?,
        };
        let response: put_object::Response = {
            let future = client.put_object(request);
            executor::block_on(future)?
        };
        let content = Content {
            e_tag: response.e_tag.as_str().to_string(),
        };
        Ok(CommandOutput::new(content.to_json()?))
    }
}

#[derive(Serialize, Deserialize)]
struct Content {
    #[serde(rename = "ETag")]
    e_tag: String,
}

impl Content {
    fn to_json(&self) -> S3ApiResult<String> {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut serializer = serde_json::Serializer::with_formatter(buf, formatter);
        self.serialize(&mut serializer)?;
        let json = String::from_utf8(serializer.into_inner()).unwrap();

        //        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }
}
