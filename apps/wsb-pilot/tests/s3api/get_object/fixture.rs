use crate::s3api::get_object::{aws_s3api, cat, wsb_s3api, SampleParameters};
use crate::s3api::TEST_BUCKET;
use serde_json::Value;
use std::io;
use std::path::Path;
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::{MutableSelf, PilotResult};

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
    pub fn outfile_text(&self) -> PilotResult<String> {
        let text = read_to_string(&self.parameters.outfile_dst)?;
        Ok(text)
    }
}

fn setup_fixture() -> PilotResult<Fixture> {
    let wsb = {
        let params = create_sample().mutate(|mut x| x.outfile_dst = "./sample1.wsb.tmp".into());
        let output = wsb_s3api().run(download, &params)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: params,
        }
    };
    let aws = {
        let params = create_sample().mutate(|mut x| x.outfile_dst = "./sample1.aws.tmp".into());
        let output = aws_s3api().run(download, &params)?;
        OutputFixture {
            status_code: output.status_code(),
            json: output.stdout_to_json()?,
            parameters: params,
        }
    };
    Ok(Fixture { wsb, aws })
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

fn read_to_string(path: &Path) -> io::Result<String> {
    let path_str: &str = &path.to_string_lossy();
    let output = cat().arg(path_str).output_silently()?;
    Ok(output.stdout_to_string())
}
