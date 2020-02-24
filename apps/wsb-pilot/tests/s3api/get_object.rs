use crate::s3api::{TEST_BUCKET, TEST_WORKSPACE_DIR};
use std::path::PathBuf;
use wsb_pilot::cmd::CommandRunner;
use wsb_pilot::{MutableSelf, PilotResult};

#[test]
fn return_zero_on_succeeded() -> PilotResult<()> {
    setup()?;

    let run = |runner: CommandRunner, sample: Sample| {
        runner
            .arg("get-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &sample.object_key])
            .arg(&sample.outfile_dst)
            .output()
    };
    let aws_output = {
        let sample = get_sample().mutate(|mut x| x.outfile_dst = "./sample1.aws.tmp".into());
        run(aws_s3api(), sample)?
    };

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

    let wsb_output = {
        let sample = get_sample().mutate(|mut x| x.outfile_dst = "./sample1.wsb.tmp".into());
        run(wsb_s3api(), sample)?
    };
    let aws_json = aws_output.stdout_to_json()?;
    let wsb_json = wsb_output.stdout_to_json()?;
    assert_eq!(wsb_json["ETag"], aws_json["ETag"]);

    Ok({})
}

#[test]
fn sample1() {
    // setup();
}

#[test]
fn sample2() {
    // setup();
}

#[test]
fn sample3() {
    // setup();
}

// workaround to emulate singleton initializer
lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("get-object");
    static ref SETUP_RESULT: PilotResult<()> = init();
}

fn setup() -> PilotResult<()> {
    if let Err(e) = &*SETUP_RESULT {
        assert!(false, "setup failed: {:?}", e);
    }
    Ok({})
}

fn init() -> PilotResult<()> {
    println!("[get-object] init");

    // /*
    for mock in get_mock_files() {
        let _aws_output = aws_s3api()
            .arg("put-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &mock.object_key])
            .args(&["--body", &mock.file_path.to_string_lossy()])
            .output()?;
    }
    // */
    Ok({})
}

fn get_mock_files() -> Vec<MockFile> {
    vec![
        MockFile {
            object_key: "s3api/get-object/foo/bar/sample1.txt.tmp".to_string(),
            file_path: "./sample1.txt".into(),
        },
        MockFile {
            object_key: "s3api/get-object/foo/bar/sample2.txt.tmp".to_string(),
            file_path: "./sample2.txt".into(),
        },
        MockFile {
            object_key: "s3api/get-object/foo/bar/sample3.txt.tmp".to_string(),
            file_path: "./sample3.txt".into(),
        },
    ]
}

fn get_sample() -> Sample {
    Sample {
        object_key: "s3api/get-object/foo/bar/sample1.txt.tmp".to_string(),
        outfile_dst: "./sample1.tmp".into(),
    }
}

fn aws_s3api() -> CommandRunner {
    super::aws_s3api().current_dir(&*WORKSPACE)
}

fn wsb_s3api() -> CommandRunner {
    super::wsb_s3api().current_dir(&*WORKSPACE)
}

struct Sample {
    object_key: String,
    outfile_dst: PathBuf,
}

struct MockFile {
    object_key: String,
    file_path: PathBuf,
}
