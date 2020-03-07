use crate::s3api::put_object::fixtures::OUTPUT;

use crate::s3api::put_object::wsb_s3api;
use wsb_pilot::PilotResult;

#[test]
fn is_zero_on_succeeded() -> PilotResult<()> {
    assert_eq!(OUTPUT.wsb.status_code, 0);
    Ok(())
    /*
    let put_object = |target: &SampleParameters| {
        wsb_s3api()
            .arg("put-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &target.object_key])
            .args(&["--body", &target.upload_src.to_string_lossy()])
            .output()
    };
    let sample = SampleParameters {
        object_key: "s3api/put-object/foo/bar/sample1.txt".to_string(),
        upload_src: "./sample.txt".into(),
        download_dst: "./downloaded1.tmp".into(),
    };
    let expected = {
        assert_eq!(put_object(&sample)?.status_code(), 0);
        cat(&sample.upload_src)?
    };
    let actual = {
        assert_eq!(get_object(&sample)?.status_code(), 0);
        cat(&sample.download_dst)?
    };
    assert_eq!(actual, expected, "correctly uploaded.");
    Ok(())
    */
}

#[test]
fn is_non_zero_on_failed() -> PilotResult<()> {
    let output = wsb_s3api().arg("unknown-subcommand").output()?;
    assert_ne!(0, output.status_code(), "return zero if it succeeded.");
    Ok(())
}
