use crate::s3api::put_object::fixtures::OUTPUT;
use plus_pilot::PilotResult;

#[test]
fn is_uploaded_correctly() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb.download_text()?, OUTPUT.wsb.uploaded_text()?);
    Ok(())
}
