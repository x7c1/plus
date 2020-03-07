use crate::s3api::get_object::fixtures::create_sample_pair;
use crate::s3api::get_object::{aws_s3api, remove_if_exists};
use crate::s3api::TEST_BUCKET;
use std::path::PathBuf;
use wsb_pilot::PilotResult;

pub fn run() -> PilotResult<()> {
    delete_downloaded_files()?;
    upload_mock_files()?;
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

pub fn create_mock_params() -> Vec<MockParameters> {
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

pub struct MockParameters {
    pub object_key: String,
    file_path: PathBuf,
}
