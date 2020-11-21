mod task;

use task::Params;
use task::Task;

use crate::core::support::release::PackageName;
use crate::tasks::shared::files;
use crate::TaskResult;
use clap::{App, Arg, ArgMatches, SubCommand};
use clap_extractor::Matcher;
use clap_task::ClapTask;

pub fn clap() -> Box<dyn ClapTask<TaskResult<()>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<()>> for Task {
    fn name(&self) -> &str {
        "release-apps"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
            .about("Release applications.")
            .long_about("Run cargo publish, git tag, git push, etc")
            .arg(files::arg())
            .arg(
                Arg::with_name("dry-run")
                    .long("dry-run")
                    .help("Perform 'cargo publish --dry-run'"),
            )
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<()> {
        let params = Params {
            files: matches.single("files").as_required()?,
            target_packages: PackageName::asset_packages(),
        };
        if matches.is_present("dry-run") {
            self.release_dry_run(&params)
        } else {
            self.release(&params)
        }
    }
}
