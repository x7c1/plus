use crate::s3api::{TEST_BUCKET, TEST_WORKSPACE_DIR};
use std::io;
use std::path::{Path, PathBuf};
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::PilotResult;

mod status {
    use super::*;

    #[test]
    fn is_zero_on_succeeded() -> PilotResult<()> {
        let put_object = |target: &Sample| {
            wsb_s3api()
                .arg("put-object")
                .args(&["--bucket", &TEST_BUCKET])
                .args(&["--key", &target.object_key])
                .args(&["--body", &target.upload_src.to_string_lossy()])
                .output()
        };
        let sample = Sample {
            object_key: "s3api/put-object/foo/bar/sample1.txt".to_string(),
            upload_src: "./sample.txt".into(),
            download_dst: "./downloaded1.tmp".into(),
        };
        let expected = {
            assert_eq!(put_object(&sample)?.status_code(), 0);
            read_to_string(&sample.upload_src)?
        };
        let actual = {
            assert_eq!(get_object(&sample)?.status_code(), 0);
            read_to_string(&sample.download_dst)?
        };
        assert_eq!(actual, expected, "correctly uploaded.");
        Ok({})
    }

    #[test]
    fn is_non_zero_on_failed() -> PilotResult<()> {
        let output = wsb_s3api().arg("unknown-subcommand").output()?;
        assert_eq!(1, output.status_code(), "return zero if it succeeded.");
        Ok({})
    }

    fn get_object(target: &Sample) -> io::Result<CommandOutput> {
        aws_s3api()
            .arg("get-object")
            .args(&["--bucket", &TEST_BUCKET])
            .args(&["--key", &target.object_key])
            .arg(&target.download_dst)
            .output()
    }

    fn read_to_string(path: &Path) -> io::Result<String> {
        let path_str: &str = &path.to_string_lossy();
        let output = cat().arg(path_str).output_silently()?;
        Ok(output.stdout_to_string())
    }
}

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
