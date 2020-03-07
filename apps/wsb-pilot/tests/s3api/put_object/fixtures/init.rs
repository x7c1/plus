use crate::s3api::put_object::aws_s3api;
use crate::s3api::put_object::fixtures::create_sample_pair;
use crate::s3api::TEST_BUCKET;
use wsb_pilot::PilotResult;

pub fn run() -> PilotResult<()> {
    delete_s3_mock_files()?;
    Ok(())
}

fn delete_s3_mock_files() -> PilotResult<()> {
    // rf. [delete-objects — AWS CLI 1.18.16 Command Reference](https://docs.aws.amazon.com/cli/latest/reference/s3api/delete-objects.html)
    let objects = create_sample_pair()
        .as_vec()
        .iter()
        .map(|x| x.format_to_delete())
        .collect::<Vec<String>>()
        .join(",");

    let _aws_output = aws_s3api()
        .arg("delete-objects")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--delete", &format!("Objects=[{}]", objects)])
        .output()?;

    Ok(())
}
