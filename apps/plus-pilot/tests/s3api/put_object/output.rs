use crate::s3api::put_object::fixtures::OUTPUT;
use plus_pilot::PilotResult;

#[test]
fn e_tag_is_correct() -> PilotResult<()> {
    assert_eq!(OUTPUT.plus.json["ETag"], OUTPUT.aws.json["ETag"]);
    Ok(())
}
