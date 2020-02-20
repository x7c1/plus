use crate::s3api::{aws, dump, s3api, TEST_BUCKET, TEST_WORKSPACE_DIR};
use std::env::set_current_dir;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Once;
use wsb_pilot::{MutableSelf, PilotResult};

fn go_to_workspace() -> PilotResult<()> {
    let workspace = Path::new(&*TEST_WORKSPACE_DIR).join("s3api/get-object");
    println!("workspace: {:?}", workspace);
    set_current_dir(workspace)?;
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

fn get_sample_for_aws() -> Sample {
    get_sample().mutate(|mut x| x.outfile_dst = "./sample1.aws.tmp".into())
}

fn get_sample_for_wsb() -> Sample {
    get_sample().mutate(|mut x| x.outfile_dst = "./sample1.wsb.tmp".into())
}

lazy_static! {
    static ref setup_result: () = init().unwrap();
}

fn setup() -> () {
    let _ = &*setup_result;

    /*
    let mut result: PilotResult<()> = Ok({});

    SETUP.call_once(move || {
        result = init();
        //        if let Err(e) = init() {
        //            eprintln!("setup failed: {:?}", e);
        //            exit(1)
        //        }
    });
    result;
    Ok({})
    */
}

fn init() -> PilotResult<()> {
    go_to_workspace()?;
    println!("[get-object] init");

    /*
    for mock in get_mock_files() {
        let aws_output = aws()
            .arg("s3api")
            .arg("put-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &mock.object_key])
            .args(&["--body", &mock.file_path.to_string_lossy()])
            .output()?;

        dump(&aws_output);
    }
    */
    Ok({})
}

#[test]
fn return_zero_on_succeeded() -> PilotResult<()> {
    setup();

    /*
    let aws_sample = get_sample_for_aws();
    let aws_output = aws()
        .arg("s3api")
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &aws_sample.object_key])
        .arg(&aws_sample.outfile_dst)
        .output()?;

    dump(&aws_output);
    */

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

    let wsb_sample = get_sample_for_wsb();
    let wsb_output = s3api()
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &wsb_sample.object_key])
        .arg(&wsb_sample.outfile_dst)
        .output()?;

    dump(&wsb_output);

    Ok({})
}

#[test]
fn sample1() {
    setup();
}

#[test]
fn sample2() {
    setup();
}

#[test]
fn sample3() {
    setup();
}

struct Sample {
    object_key: String,
    outfile_dst: PathBuf,
}

struct MockFile {
    object_key: String,
    file_path: PathBuf,
}
