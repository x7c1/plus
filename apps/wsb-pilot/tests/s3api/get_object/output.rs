use crate::s3api::get_object::fixture::OUTPUT;
use wsb_pilot::PilotResult;

/*
{
    "AcceptRanges": "bytes",
    "LastModified": "Thu, 20 Feb 2020 13:28:58 GMT",
    "ContentLength": 8,
    "ETag": "\"090a4e14a392f707cf164a20cee76c18\"",
    "ContentType": "binary/octet-stream",
    "Metadata": {}
}
*/

#[test]
fn e_tag_is_correct() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb["ETag"], OUTPUT.aws["ETag"]);
    Ok({})
}
