use crate::s3api::*;
use std::env::set_current_dir;
use std::fs;
use std::process::Command;
use wsb_pilot::PilotResult;

#[test]
fn return_zero_on_succeeded() -> PilotResult<()> {
    let object_key = "s3api/put-object/foo/bar/sample.txt";
    let uploadee = "./sample.txt";
    let downloaded = "./downloaded.tmp";

    let workspace = format!("{}/s3api/put-object", *TEST_WORKSPACE_DIR);
    println!("workspace: {}", workspace);
    set_current_dir(&workspace)?;

    let output = s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", object_key])
        .args(&["--body", uploadee])
        .output()?;

    dump_if_failed(&output);

    assert_eq!(
        Some(0),
        output.status.code(),
        "return non-zero if it failed."
    );
    let output = Command::new("aws")
        .arg("s3api")
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", object_key])
        .arg(downloaded)
        .output()?;

    dump_if_failed(&output);

    let expected = fs::read_to_string(uploadee)?;
    let actual = fs::read_to_string(downloaded)?;
    assert_eq!(expected, actual);

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
