use crate::s3api::get_object::fixture::OUTPUT;
use wsb_pilot::PilotResult;

#[test]
fn is_written_correctly() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb.outfile_text()?, OUTPUT.aws.outfile_text()?);
    Ok(())
}
