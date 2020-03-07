use crate::s3api::get_object::aws_s3api;
use crate::s3api::TEST_BUCKET;
use std::path::PathBuf;
use wsb_pilot::PilotResult;

pub fn run() -> PilotResult<()> {
    upload_mock_files()?;
    Ok({})
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
    Ok({})
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
