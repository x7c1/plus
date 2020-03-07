use crate::s3api::get_object::{aws_s3api, wsb_s3api, SampleParameters};
use crate::s3api::TEST_BUCKET;
use serde_json::Value;
use std::io;
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::{MutableSelf, PilotResult};

lazy_static! {
    pub static ref OUTPUT: Fixture = setup_fixture().unwrap();
}

pub struct Fixture {
    pub wsb: CliOutput,
    pub aws: CliOutput,
}

pub struct CliOutput {
    pub status_code: i32,
    pub json: Value,
}

fn setup_fixture() -> PilotResult<Fixture> {
    let by_wsb: CommandOutput = {
        let sample = create_sample().mutate(|mut x| x.outfile_dst = "./sample1.wsb.tmp".into());
        wsb_s3api().run(download, &sample)?
    };
    let by_aws = {
        let sample = create_sample().mutate(|mut x| x.outfile_dst = "./sample1.aws.tmp".into());
        aws_s3api().run(download, &sample)?
    };
    Ok(Fixture {
        wsb: CliOutput {
            status_code: by_wsb.status_code(),
            json: by_wsb.stdout_to_json()?,
        },
        aws: CliOutput {
            status_code: by_aws.status_code(),
            json: by_aws.stdout_to_json()?,
        },
    })
}

fn create_sample() -> SampleParameters {
    SampleParameters {
        object_key: "s3api/get-object/foo/bar/sample1.txt.tmp".to_string(),
        outfile_dst: "./sample1.tmp".into(),
    }
}

fn download(runner: CommandRunner, target: &SampleParameters) -> io::Result<CommandOutput> {
    runner
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .arg(&target.outfile_dst)
        .output()
}
