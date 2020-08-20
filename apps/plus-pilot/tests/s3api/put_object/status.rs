use crate::s3api::put_object::fixtures::OUTPUT;
use crate::s3api::put_object::workspace;
use crate::s3api::TEST_BUCKET;
use plus_pilot::PilotResult;

#[test]
fn is_zero_on_succeeded() -> PilotResult<()> {
    assert_eq!(OUTPUT.plus.status_code, 0);
    Ok(())
}

#[test]
fn is_non_zero_on_invalid_subcommand() -> PilotResult<()> {
    let output = workspace()
        .plus_s3api()
        .arg("unknown-subcommand")
        .execute()?;

    assert_ne!(0, output.status_code(), "return zero if it succeeded.");
    Ok(())
}

#[test]
fn is_non_zero_if_body_not_found() -> PilotResult<()> {
    let body = "./invalid/file/path";
    let output = workspace()
        .plus_s3api()
        .arg("put-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", "sample-object-key"])
        .args(&["--body", body])
        .execute()?;

    assert_ne!(output.status_code(), 0);
    assert!(output.stderr_to_string().contains(body));
    Ok(())
}
