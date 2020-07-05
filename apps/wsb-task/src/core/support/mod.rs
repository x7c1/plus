mod build_mode;
pub use build_mode::HasBuildMode;

mod cc;
use crate::TaskResult;
pub use cc::{confirm_cc, CCRequired};
use shellwork::core::command::{Runner, Unprepared};

pub fn confirm_program(runner: &Runner<Unprepared>) -> TaskResult<()> {
    // todo:
    println!("exists? {}", runner.program);
    Ok(())
}
