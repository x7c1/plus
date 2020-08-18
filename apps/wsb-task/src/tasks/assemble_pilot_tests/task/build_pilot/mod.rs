mod params;
pub use params::{OutputKind, Params};

use crate::core::support::{CCRequired, HasBuildMode};
use shellwork::core::command;
use shellwork::core::command::{program, Runner, Unprepared};

pub fn create_runner(params: &Params) -> Runner<Unprepared> {
    let default = command::program("cargo")
        .arg("test")
        .push_arg(params.build_mode())
        .args(&["--target", params.target.as_triple()])
        .args(&["--package", "plus-pilot"])
        .arg("--no-run")
        .env("RUSTFLAGS", params.opt_level())
        .env_entry(params.cc());

    // call via OutputKind::Default in advance to see compilation errors,
    // since OutputKind::FileName hides them by the --message-format option.
    match params.output_kind {
        OutputKind::Default => default,
        OutputKind::FileName => pipe_to_get_file_name(default),
    }
}

fn pipe_to_get_file_name(base: Runner<Unprepared>) -> Runner<Unprepared> {
    let jq = program("jq").args(&["-r", "select(.profile.test == true) | .filenames[]"]);
    let grep = program("grep").arg("plus_pilot_tests");
    let tr = program("tr").args(&["-d", "\n"]);
    base.args(&["--message-format", "json"])
        .pipe(jq)
        .pipe(grep)
        .pipe(tr)
}
