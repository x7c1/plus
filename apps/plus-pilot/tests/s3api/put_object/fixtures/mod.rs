mod init;

use crate::s3api::put_object::{workspace, SampleParameters};
use crate::s3api::{ParametersPair, TEST_BUCKET};
use plus_pilot::cmd::{CommandOutput, CommandRunner};
use plus_pilot::PilotResult;
use serde_json::Value;

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
    pub fn uploaded_text(&self) -> PilotResult<String> {
        let text = workspace().cat(&self.parameters.upload_src)?;
        Ok(text)
    }

    pub fn download_text(&self) -> PilotResult<String> {
        let _aws_output = download(&self.parameters)?;
        let text = workspace().cat(&self.parameters.download_dst)?;
        Ok(text)
    }
}

fn setup_fixture() -> PilotResult<Fixture> {
    init::run()?;

    let pair = create_sample_pair();
    let plus = {
        let output = workspace().plus_s3api().run(upload, &pair.plus)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: pair.plus,
        }
    };
    let aws = {
        let output = workspace().aws_s3api().run(upload, &pair.aws)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: pair.aws,
        }
    };
    Ok(Fixture { plus, aws })
}

fn create_sample_pair() -> ParametersPair<SampleParameters> {
    ParametersPair {
        plus: SampleParameters {
            object_key: "s3api/put-object/foo/bar/sample1.plus.tmp".to_string(),
            upload_src: "./sample.txt".into(),
            download_dst: "./downloaded.plus.tmp".into(),
        },
        aws: SampleParameters {
            object_key: "s3api/put-object/foo/bar/sample1.aws.tmp".to_string(),
            upload_src: "./sample.txt".into(),
            download_dst: "./downloaded.aws.tmp".into(),
        },
    }
}

fn upload(runner: CommandRunner, target: &SampleParameters) -> PilotResult<CommandOutput> {
    runner
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .args(&["--body", &target.upload_src.to_string_lossy()])
        .output()
}

fn download(target: &SampleParameters) -> PilotResult<CommandOutput> {
    workspace()
        .aws_s3api()
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .arg(&target.download_dst)
        .output()
}
