mod init;

use crate::s3api::put_object::{aws_s3api, cat, wsb_s3api, SampleParameters};
use crate::s3api::TEST_BUCKET;
use serde_json::Value;
use std::io;
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::PilotResult;

lazy_static! {
    pub static ref OUTPUT: Fixture = setup_fixture().unwrap();
}

pub struct Fixture {
    pub wsb: OutputFixture,
    pub aws: OutputFixture,
}

pub struct OutputFixture {
    pub status_code: i32,
    pub json: Value,
    pub parameters: SampleParameters,
}

impl OutputFixture {
    pub fn uploaded_text(&self) -> PilotResult<String> {
        let text = cat(&self.parameters.upload_src)?;
        Ok(text)
    }

    pub fn download_text(&self) -> PilotResult<String> {
        let _aws_output = download(&self.parameters)?;
        let text = cat(&self.parameters.download_dst)?;
        Ok(text)
    }
}

pub struct SamplePair {
    wsb: SampleParameters,
    aws: SampleParameters,
}

impl SamplePair {
    pub fn as_vec(&self) -> Vec<&SampleParameters> {
        vec![&self.wsb, &self.aws]
    }
}

fn setup_fixture() -> PilotResult<Fixture> {
    init::run()?;

    let pair = create_sample_pair();
    let wsb = {
        let output = wsb_s3api().run(upload, &pair.wsb)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: pair.wsb,
        }
    };
    let aws = {
        let output = aws_s3api().run(upload, &pair.aws)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: pair.aws,
        }
    };
    Ok(Fixture { wsb, aws })
}

fn create_sample_pair() -> SamplePair {
    SamplePair {
        wsb: SampleParameters {
            object_key: "s3api/put-object/foo/bar/sample1.wsb.tmp".to_string(),
            upload_src: "./sample.txt".into(),
            download_dst: "./downloaded.wsb.tmp".into(),
        },
        aws: SampleParameters {
            object_key: "s3api/put-object/foo/bar/sample1.aws.tmp".to_string(),
            upload_src: "./sample.txt".into(),
            download_dst: "./downloaded.aws.tmp".into(),
        },
    }
}

fn upload(runner: CommandRunner, target: &SampleParameters) -> io::Result<CommandOutput> {
    runner
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .args(&["--body", &target.upload_src.to_string_lossy()])
        .output()
}

fn download(target: &SampleParameters) -> io::Result<CommandOutput> {
    aws_s3api()
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .arg(&target.download_dst)
        .output()
}
