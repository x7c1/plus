use crate::s3api::get_object::fixtures::create_sample_pair;
use crate::s3api::get_object::{aws_s3api, remove_if_exists};
use crate::s3api::TEST_BUCKET;
use std::path::PathBuf;
use wsb_pilot::PilotResult;

pub fn run() -> PilotResult<()> {
    delete_downloaded_files()?;
    delete_s3_mock_files()?;
    upload_mock_files()?;
    Ok(())
}

fn delete_s3_mock_files() -> PilotResult<()> {
    // rf. [delete-objects — AWS CLI 1.18.16 Command Reference](https://docs.aws.amazon.com/cli/latest/reference/s3api/delete-objects.html)
    let objects = create_mock_params()
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

fn delete_downloaded_files() -> PilotResult<()> {
    create_sample_pair()
        .as_vec()
        .iter()
        .map(|params| &params.outfile_dst)
        .try_for_each(|path| remove_if_exists(path))?;

    Ok(())
}

fn upload_mock_files() -> PilotResult<()> {
    for params in create_mock_params() {
        let _aws_output = aws_s3api()
            .arg("put-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &params.object_key])
            .args(&["--body", &params.file_path.to_string_lossy()])
            .output()?;
    }
    Ok(())
}

fn create_mock_params() -> Vec<MockParameters> {
    vec![
        MockParameters {
            object_key: "s3api/get-object/foo/bar/sample1.txt.tmp".to_string(),
            file_path: "./sample1.txt".into(),
        },
        MockParameters {
            object_key: "s3api/get-object/foo/bar/sample2.txt.tmp".to_string(),
            file_path: "./sample2.txt".into(),
        },
    ]
}

struct MockParameters {
    object_key: String,
    file_path: PathBuf,
}

impl MockParameters {
    fn format_to_delete(&self) -> String {
        format!("{{ Key={} }}", self.object_key)
    }
}
