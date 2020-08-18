mod params;
pub use params::{Params, ParamsBuilder};

use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

pub fn create_runner(params: &Params) -> Runner<Unprepared> {
    let recursive = if params.src.is_dir() {
        Some("--recursive")
    } else {
        None
    };
    command::program("cp")
        .arg("--no-target-directory")
        .push_arg(recursive)
        .args(&[&params.src, &params.dst])
}
