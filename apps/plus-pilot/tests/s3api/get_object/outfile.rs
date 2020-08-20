use crate::s3api::get_object::fixtures::OUTPUT;
use plus_pilot::PilotResult;

#[test]
fn is_written_correctly() -> PilotResult<()> {
    assert_eq!(OUTPUT.plus.outfile_text()?, OUTPUT.aws.outfile_text()?);
    Ok(())
}
