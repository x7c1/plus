use crate::s3api::put_object::{aws_s3api, wsb_s3api, SampleParameters};
use crate::s3api::TEST_BUCKET;
use serde_json::Value;
use std::io;
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::PilotResult;

#[test]
fn e_tag_is_correct() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb["ETag"], OUTPUT.aws["ETag"]);
    Ok(())
}

lazy_static! {
    static ref OUTPUT: Fixture = upload_sample().unwrap();
}

fn upload_sample() -> PilotResult<Fixture> {
    let sample = SampleParameters {
        object_key: "s3api/put-object/foo/bar/sample2.txt".to_string(),
        upload_src: "./sample.txt".into(),
        download_dst: "./downloaded2.tmp".into(),
    };
    let by_wsb = upload(wsb_s3api(), &sample)?;
    let by_aws = upload(aws_s3api(), &sample)?;
    Ok(Fixture {
        wsb: by_wsb.stdout_to_json()?,
        aws: by_aws.stdout_to_json()?,
    })
}

fn upload(runner: CommandRunner, params: &SampleParameters) -> io::Result<CommandOutput> {
    runner
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &params.object_key])
        .args(&["--body", &params.upload_src.to_string_lossy()])
        .output()
}

struct Fixture {
    wsb: Value,
    aws: Value,
}
