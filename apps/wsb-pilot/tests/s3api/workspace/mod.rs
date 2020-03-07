use crate::s3api::{TEST_APPS_DIR, TEST_WORKSPACE_DIR};
use std::path::{Path, PathBuf};
use std::{fs, io};
use wsb_pilot::cmd::CommandRunner;

pub struct Workspace {
    dir: PathBuf,
}

impl Workspace {
    pub fn new(path: &[&str]) -> Workspace {
        let root = PathBuf::new().join(&*TEST_WORKSPACE_DIR);
        let dir = path.iter().fold(root, |acc, name| acc.join(name));
        Workspace { dir }
    }

    pub fn aws_s3api(&self) -> CommandRunner {
        let runner = CommandRunner::new("aws").arg("s3api");
        runner.current_dir(&self.dir)
    }

    pub fn wsb_s3api(&self) -> CommandRunner {
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
