use crate::core::targets::BuildTarget;
use crate::TaskResult;
use clap::{Arg, ArgMatches};
use clap_extractor::Matcher;

pub fn arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("target")
        .long("target")
        .required(true)
        .takes_value(true)
        .help("Build for the target label.")
}

pub fn from<'a>(matches: &'a ArgMatches<'a>) -> TaskResult<BuildTarget> {
    let target = matches.single("target").as_required()?;
    Ok(target)
}
