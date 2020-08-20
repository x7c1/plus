mod init;

use crate::s3api::get_object::{workspace, SampleParameters};
use crate::s3api::{ParametersPair, TEST_BUCKET};
use plus_pilot::cmd::{CommandOutput, CommandRunner};
use plus_pilot::PilotResult;
use serde_json::Value;
use std::io;

lazy_static! {
    pub static ref OUTPUT: Fixture = setup_fixture().unwrap();
}

pub struct Fixture {
    pub plus: OutputFixture,
    pub aws: OutputFixture,
}

pub struct OutputFixture {
    pub status_code: i32,
    pub json: Value,
    pub parameters: SampleParameters,
}

impl OutputFixture {
    pub fn outfile_text(&self) -> PilotResult<String> {
        let text = workspace().cat(&self.parameters.outfile_dst)?;
        Ok(text)
    }
}

fn setup_fixture() -> PilotResult<Fixture> {
    init::run()?;

    let pair = create_sample_pair();
    let plus = {
        let output = workspace().plus_s3api().run(download, &pair.plus)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: pair.plus,
        }
    };
    let aws = {
        let output = workspace().aws_s3api().run(download, &pair.aws)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: pair.aws,
        }
    };
    Ok(Fixture { plus, aws })
}

fn create_sample_pair() -> ParametersPair<SampleParameters> {
    let params = init::create_mock_params();
    let object_key = &params[0].object_key;

    ParametersPair {
        plus: SampleParameters {
            object_key: object_key.to_owned(),
            outfile_dst: "./sample1.plus.tmp".into(),
        },
        aws: SampleParameters {
            object_key: object_key.to_owned(),
            outfile_dst: "./sample1.aws.tmp".into(),
        },
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
