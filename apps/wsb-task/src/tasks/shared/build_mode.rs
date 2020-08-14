use crate::core::build_mode::BuildMode;
use crate::TaskResult;
use clap::{Arg, ArgMatches};

pub fn arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("release")
        .long("release")
        .required(false)
        .takes_value(false)
        .help("Build artifacts in release mode, with optimizations")
}

pub fn from<'a>(matches: &'a ArgMatches<'a>) -> TaskResult<BuildMode> {
    let build_mode = if matches.is_present("release") {
        BuildMode::Release
    } else {
        BuildMode::Debug
    };
    Ok(build_mode)
}
