mod task;

use task::{Params, Task};

use crate::core::support::release::PackageName;
use crate::tasks::shared;
use crate::tasks::shared::{build_mode, build_target, files};
use crate::TaskResult;
use clap::{App, ArgMatches, SubCommand};
use clap_extractor::Matcher;
use clap_task::ClapTask;

pub fn clap() -> Box<dyn ClapTask<TaskResult<()>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<()>> for Task {
    fn name(&self) -> &str {
        "package-artifacts"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
            .about("Compress and archive artifacts.")
            .arg(build_target::arg())
            .arg(files::arg())
            .arg(build_mode::arg())
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<()> {
        let shared_params = shared::Params::from_matches(matches)?;
        let params = Params {
            files: matches.single("files").as_required()?,
            target_packages: PackageName::asset_packages(),
            target: shared_params.target,
            build_mode: shared_params.build_mode,
        };
        self.start(&params)
    }
}
