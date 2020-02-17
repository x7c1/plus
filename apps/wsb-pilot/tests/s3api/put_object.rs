use wsb_pilot::PilotResult;

use crate::s3api::*;

#[test]
fn return_zero_on_succeeded() -> PilotResult<()> {
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
        "return non-zero if it failed."
    );
    Ok({})
}

#[test]
fn return_non_zero_on_failed() -> PilotResult<()> {
    let output = s3api().arg("unknown-subcommand").output()?;

    dump(&output);
    assert_eq!(
        Some(1),
        output.status.code(),
        "return zero if it succeeded."
    );
    Ok({})
}
