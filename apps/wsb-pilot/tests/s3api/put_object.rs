use crate::s3api::*;
use std::env::set_current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use wsb_pilot::PilotResult;

fn get_sample1() -> Sample {
    Sample {
        object_key: "s3api/put-object/foo/bar/sample.txt".to_string(),
        upload_src: "./sample.txt".into(),
        download_dst: "./downloaded.tmp".into(),
    }
}

fn to_workspace() -> PilotResult<()> {
    let workspace = Path::new(&*TEST_WORKSPACE_DIR).join("s3api/put-object");
    println!("workspace: {:?}", workspace);
    set_current_dir(workspace)?;
    Ok({})
}

#[test]
fn return_zero_on_succeeded() -> PilotResult<()> {
    to_workspace()?;

    let sample = get_sample1();
    let output = s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &sample.object_key])
        .args(&["--body", &sample.upload_src.to_string_lossy()])
        .output()?;

    dump(&output);

    assert_eq!(
        Some(0),
        output.status.code(),
        "return non-zero if it failed."
    );
    let output = aws()
        .arg("s3api")
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &sample.object_key])
        .arg(&sample.download_dst)
        .output()?;

    dump_if_failed(&output);

    let expected = fs::read_to_string(&sample.upload_src)?;
    let actual = fs::read_to_string(&sample.download_dst)?;
    assert_eq!(expected, actual, "correctly uploaded.");
    Ok({})
}

#[test]
fn output_etag_is_same_as_one_by_aws_cli() -> PilotResult<()> {
    to_workspace()?;

    let sample = get_sample1();
    let original = aws()
        .arg("s3api")
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &sample.object_key])
        .args(&["--body", &sample.upload_src.to_string_lossy()])
        .output()?;

    dump(&original);

    let wsb = s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &sample.object_key])
        .args(&["--body", &sample.upload_src.to_string_lossy()])
        .output()?;

    dump(&wsb);

    Ok({})
}

#[test]
fn return_non_zero_on_failed() -> PilotResult<()> {
    let output = s3api().arg("unknown-subcommand").output()?;
    dump_if_failed(&output);

    assert_eq!(
        Some(1),
        output.status.code(),
        "return zero if it succeeded."
    );
    Ok({})
}

struct Sample {
    object_key: String,
    upload_src: PathBuf,
    download_dst: PathBuf,
}
