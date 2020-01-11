#[macro_use]
extern crate clap;

#[macro_use]
extern crate failure;

mod commands;
mod error;
pub use error::Result as S3ApiResult;

use clap::App;
use clap_task::{Definition, TaskFinder};
use std::process::exit;

fn main() {
    println!("Hello, world!");

    match run() {
        Ok(response) => {
            println!("succeeded: {:#?}", response);
        }
        Err(e) => {
            println!("failed: {:#?}", e);
            exit(1);
        }
    }
}
type ApiResult = S3ApiResult<()>;

type ApiDef<'a, 'b> = Definition<'a, 'b, ApiResult>;

fn run() -> ApiResult {
    let definitions: Vec<ApiDef> = vec![
        commands::get_object::create(),
        commands::put_object::create(),
    ];
    let finder = TaskFinder::new(init(), definitions)?;
    let task = finder.require_task()?;
    task.run()
}

fn init<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
}
