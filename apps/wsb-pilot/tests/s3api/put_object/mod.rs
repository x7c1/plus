mod status;

use crate::s3api::{TEST_BUCKET, TEST_WORKSPACE_DIR};
use std::io;
use std::path::PathBuf;
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::PilotResult;

mod output {
    use super::*;
    use serde_json::Value;

    lazy_static! {
        static ref OUTPUT: Fixture = upload_sample().unwrap();
    }

    #[test]
    fn e_tag_is_correct() -> PilotResult<()> {
        assert_eq!(OUTPUT.wsb["ETag"], OUTPUT.aws["ETag"]);
        Ok({})
    }

    fn upload_sample() -> PilotResult<Fixture> {
        let sample = Sample {
            object_key: "s3api/put-object/foo/bar/sample2.txt".to_string(),
            upload_src: "./sample.txt".into(),
            download_dst: "./downloaded2.tmp".into(),
        };
        let by_wsb = upload(wsb_s3api(), &sample)?;
        let by_aws = upload(aws_s3api(), &sample)?;
        Ok(Fixture {
            wsb: by_wsb.stdout_to_json()?,
            aws: by_aws.stdout_to_json()?,
        })
    }

    fn upload(runner: CommandRunner, target: &Sample) -> io::Result<CommandOutput> {
        runner
            .arg("put-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &target.object_key])
            .args(&["--body", &target.upload_src.to_string_lossy()])
            .output()
    }

    struct Fixture {
        wsb: Value,
        aws: Value,
    }
}

lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("put-object");
}

fn aws_s3api() -> CommandRunner {
    super::aws_s3api().current_dir(&*WORKSPACE)
}

fn wsb_s3api() -> CommandRunner {
    super::wsb_s3api().current_dir(&*WORKSPACE)
}

fn cat() -> CommandRunner {
    super::cat().current_dir(&*WORKSPACE)
}

struct Sample {
    object_key: String,
    upload_src: PathBuf,
    download_dst: PathBuf,
}
