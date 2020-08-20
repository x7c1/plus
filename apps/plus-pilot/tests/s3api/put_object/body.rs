use crate::s3api::put_object::fixtures::OUTPUT;
use plus_pilot::PilotResult;

#[test]
fn is_uploaded_correctly() -> PilotResult<()> {
    assert_eq!(OUTPUT.plus.download_text()?, OUTPUT.plus.uploaded_text()?);
    Ok(())
}
