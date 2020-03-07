use crate::s3api::put_object::fixtures::OUTPUT;
use wsb_pilot::PilotResult;

#[test]
fn e_tag_is_correct() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb.json["ETag"], OUTPUT.aws.json["ETag"]);
    Ok(())
}
