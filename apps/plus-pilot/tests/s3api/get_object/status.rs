use crate::s3api::get_object::fixtures::OUTPUT;
use plus_pilot::PilotResult;

#[test]
fn is_zero_on_succeeded() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb.status_code, 0);
    Ok(())
}
