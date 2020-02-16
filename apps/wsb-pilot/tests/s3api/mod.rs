use env_extractor::{FromSingle, InfallibleResult};
use std::process::Command;
use wsb_pilot::PilotResult;

lazy_static! {
    static ref TEST_BUCKET: String = load_test_bucket().unwrap();
    static ref TEST_APPS_DIR: String = load_apps_dir().unwrap();
}

#[test]
fn s3api_works1() -> PilotResult<()> {
    let output = s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", "foo/bar/README.md"])
        .args(&["--body", "./README.md"])
        .output()?;

    if !output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("stderr: {}", String::from_utf8(output.stderr).unwrap());
    }
    assert_eq!(Some(0), output.status.code(), "exit with non-zero status.");
    Ok({})
}

#[test]
fn s3api_works2() {
    assert_eq!(2 + 2, 4);
}

fn s3api() -> Command {
    Command::new(format!("{}/s3api", *TEST_APPS_DIR))
}

fn load_test_bucket() -> InfallibleResult<String> {
    FromSingle::new("WSB_TEST_BUCKET").as_required()
}

fn load_apps_dir() -> InfallibleResult<String> {
    FromSingle::new("WSB_APPS_DIR").as_required()
}
