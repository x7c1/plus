use crate::s3api::{TEST_BUCKET, TEST_WORKSPACE_DIR};
use serde_json::Value;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use wsb_pilot::cmd::CommandRunner;
use wsb_pilot::PilotResult;

#[test]
fn return_zero_on_succeeded() -> PilotResult<()> {
    let sample = get_sample1();

    let wsb_output = wsb_s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &sample.object_key])
        .args(&["--body", &sample.upload_src.to_string_lossy()])
        .output()?;

    wsb_output.dump();
    assert_eq!(0, wsb_output.status_code(), "return non-zero if it failed.");
    let aws_output = aws_s3api()
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &sample.object_key])
        .arg(&sample.download_dst)
        .output()?;

    aws_output.dump_if_failed();

    let expected = read_to_string(&sample.upload_src)?;
    let actual = read_to_string(&sample.download_dst)?;
    assert_eq!(actual, expected, "correctly uploaded.");
    Ok({})
}

fn read_to_string(path: &Path) -> io::Result<String> {
    let path_str: &str = &path.to_string_lossy();
    let output = Command::new("cat")
        .current_dir(&*WORKSPACE)
        .arg(path_str)
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(output_str)
}

#[test]
fn output_e_tag_is_correct() -> PilotResult<()> {
    let sample = get_sample1();
    let run = |runner: CommandRunner| {
        runner
            .arg("put-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &sample.object_key])
            .args(&["--body", &sample.upload_src.to_string_lossy()])
            .output()
    };
    let aws_output = run(aws_s3api())?;
    aws_output.dump_if_failed();

    let wsb_output = run(wsb_s3api())?;
    wsb_output.dump();

    let aws_json: Value = serde_json::from_slice(aws_output.stdout())?;
    let wsb_json: Value = serde_json::from_slice(wsb_output.stdout())?;
    assert_eq!(wsb_json["ETag"], aws_json["ETag"]);

    Ok({})
}

#[test]
fn return_non_zero_on_failed() -> PilotResult<()> {
    let output = wsb_s3api().arg("unknown-subcommand").output()?;
    output.dump_if_failed();

    assert_eq!(1, output.status_code(), "return zero if it succeeded.");
    Ok({})
}

lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("put-object");
}

fn get_sample1() -> Sample {
    Sample {
        object_key: "s3api/put-object/foo/bar/sample.txt".to_string(),
        upload_src: "./sample.txt".into(),
        download_dst: "./downloaded.tmp".into(),
    }
}

fn aws_s3api() -> CommandRunner {
    super::aws_s3api().current_dir(&*WORKSPACE)
}

fn wsb_s3api() -> CommandRunner {
    super::wsb_s3api().current_dir(&*WORKSPACE)
}

struct Sample {
    object_key: String,
    upload_src: PathBuf,
    download_dst: PathBuf,
}
