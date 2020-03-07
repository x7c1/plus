use crate::s3api::put_object::fixtures::OUTPUT;
use crate::s3api::put_object::wsb_s3api;
use wsb_pilot::PilotResult;

#[test]
fn is_zero_on_succeeded() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb.status_code, 0);
    Ok(())
}

#[test]
fn is_non_zero_on_failed() -> PilotResult<()> {
    let output = wsb_s3api().arg("unknown-subcommand").output()?;
    assert_ne!(0, output.status_code(), "return zero if it succeeded.");
    Ok(())
}
