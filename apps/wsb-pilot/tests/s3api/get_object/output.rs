use crate::s3api::get_object::{aws_s3api, wsb_s3api, Sample};
use crate::s3api::TEST_BUCKET;
use serde_json::Value;
use std::io;
use wsb_pilot::cmd::{CommandOutput, CommandRunner};
use wsb_pilot::{MutableSelf, PilotResult};

#[test]
fn e_tag_is_correct() -> PilotResult<()> {
    // todo: call setup()

    assert_eq!(OUTPUT.wsb["ETag"], OUTPUT.aws["ETag"]);
    Ok({})
}

lazy_static! {
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
    static ref OUTPUT: Fixture = download_sample().unwrap();
}

fn download_sample() -> PilotResult<Fixture> {
    let by_wsb = {
        let sample = create_sample().mutate(|mut x| x.outfile_dst = "./sample1.wsb.tmp".into());
        wsb_s3api().run(download, &sample)?
    };
    let by_aws = {
        let sample = create_sample().mutate(|mut x| x.outfile_dst = "./sample1.aws.tmp".into());
        aws_s3api().run(download, &sample)?
    };
    Ok(Fixture {
        wsb: by_wsb.stdout_to_json()?,
        aws: by_aws.stdout_to_json()?,
    })
}

fn create_sample() -> Sample {
    Sample {
        object_key: "s3api/get-object/foo/bar/sample1.txt.tmp".to_string(),
        outfile_dst: "./sample1.tmp".into(),
    }
}

fn download(runner: CommandRunner, target: &Sample) -> io::Result<CommandOutput> {
    runner
        .arg("get-object")
        .args(&["--bucket", &TEST_BUCKET])
        .args(&["--key", &target.object_key])
        .arg(&target.outfile_dst)
        .output()
}

struct Fixture {
    wsb: Value,
    aws: Value,
}
