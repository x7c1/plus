use crate::s3api::get_object::fixtures::OUTPUT;
use wsb_pilot::PilotResult;

/* rf. output example by `aws s3api get-object`
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
    assert_eq!(OUTPUT.wsb.json["ETag"], OUTPUT.aws.json["ETag"]);
    Ok({})
}
