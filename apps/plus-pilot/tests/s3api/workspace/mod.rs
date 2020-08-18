use crate::s3api::{TEST_APPS_DIR, TEST_WORKSPACE_DIR};
use plus_pilot::cmd::CommandRunner;
use plus_pilot::Error::InvalidWorkspace;
use plus_pilot::PilotResult;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug)]
pub struct Workspace {
    dir: PathBuf,
}

impl Workspace {
    pub fn new(path: &[&str]) -> PilotResult<Workspace> {
        let root = PathBuf::new().join(&*TEST_WORKSPACE_DIR);
        let dir = path.iter().fold(root, |acc, name| acc.join(name));
        let canonical = dir.canonicalize().map_err(|e| InvalidWorkspace(dir, e))?;
        Ok(Workspace { dir: canonical })
    }

    pub fn aws_s3api(&self) -> CommandRunner {
        let runner = CommandRunner::new("aws").arg("s3api");
        runner.current_dir(&self.dir)
    }

    pub fn plus_s3api(&self) -> CommandRunner {
        let runner = CommandRunner::new(format!("{}/s3api", *TEST_APPS_DIR));
        runner.current_dir(&self.dir)
    }

    pub fn cat(&self, path: &Path) -> io::Result<String> {
        let full_path = self.dir.join(path);
        fs::read_to_string(full_path)
    }

    pub fn remove_if_exists(&self, path: &Path) -> io::Result<()> {
        let full_path: PathBuf = self.dir.join(path);
        if full_path.exists() {
            fs::remove_file(full_path)
        } else {
            Ok(())
        }
    }
}

#[test]
fn error_if_directory_not_found() {
    let result = Workspace::new(&["invalid", "path"]);
    match result {
        Err(InvalidWorkspace(_, _)) => assert!(true),
        other => assert!(false, "unexpected result: {:#?}", other),
    }
}
