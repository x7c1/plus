use crate::s3api::get_object::{aws_s3api, cat, wsb_s3api, Sample};
use crate::s3api::TEST_BUCKET;
use std::io;
use std::path::{Path, PathBuf};
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::{MutableSelf, PilotResult};

#[test]
fn is_zero_on_succeeded() -> PilotResult<()> {
    setup()?;

    let actual = {
        let sample = get_sample().mutate(|mut x| x.outfile_dst = "./sample1.wsb.tmp".into());
        let output = wsb_s3api().run(download, &sample)?;
        assert_eq!(output.status_code(), 0, "return 0 if succeeded.");

        read_to_string(&sample.outfile_dst)?;
    };
    let expected = {
        let sample = get_sample().mutate(|mut x| x.outfile_dst = "./sample1.aws.tmp".into());
        let output = aws_s3api().run(download, &sample)?;
        assert_eq!(output.status_code(), 0, "return 0 if succeeded.");

        read_to_string(&sample.outfile_dst)?;
    };
    assert_eq!(actual, expected, "correctly downloaded");
    Ok({})
}

lazy_static! {
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

fn download(runner: CommandRunner, target: &Sample) -> io::Result<CommandOutput> {
    runner
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .arg(&target.outfile_dst)
        .output()
}

fn get_sample() -> Sample {
    Sample {
        object_key: "s3api/get-object/foo/bar/sample1.txt.tmp".to_string(),
        outfile_dst: "./sample1.tmp".into(),
    }
}

fn read_to_string(path: &Path) -> io::Result<String> {
    let path_str: &str = &path.to_string_lossy();
    let output = cat().arg(path_str).output_silently()?;
    Ok(output.stdout_to_string())
}

struct MockFile {
    object_key: String,
    file_path: PathBuf,
}
