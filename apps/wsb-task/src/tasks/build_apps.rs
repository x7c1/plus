use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;
use std::io::{stdout, Read};
use std::process::{Command, Stdio};

pub fn define() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

struct Task;

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "build-apps"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("build wasabi applications.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        println!("running!!");

        // rf. [rust - How would you stream output from a Process?](https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process)
        let mut child = Command::new("cargo")
            .arg("build")
            .args(&["--target", "x86_64-unknown-linux-musl"])
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        let status = child.wait()?;
        println!("status...{:?}", status.code());

        Ok(TaskOutput::empty())
    }
}
