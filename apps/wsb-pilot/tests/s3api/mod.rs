use env_extractor::{FromSingle, InfallibleResult};
use std::process::{Command, Output};
use wsb_pilot::PilotResult;

lazy_static! {
    static ref TEST_BUCKET: String = load_test_bucket().unwrap();
    static ref TEST_APPS_DIR: String = load_apps_dir().unwrap();
}

#[test]
fn return_0_if_succeeded() -> PilotResult<()> {
    let output = s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", "foo/bar/README.md"])
        .args(&["--body", "./README.md"])
        .output()?;

    dump(&output);
    assert_eq!(
        Some(0),
        output.status.code(),
        "return non-zero status if it failed."
    );
    Ok({})
}

#[test]
fn s3api_works2() {
    assert_eq!(2 + 2, 4);
}

fn dump(output: &Output) {
    if !output.status.success() {
        let to_string = |vec| String::from_utf8_lossy(vec).to_string();
        println!("{}", to_string(&output.stdout));
        println!("stderr: {}", to_string(&output.stderr));
    }
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
