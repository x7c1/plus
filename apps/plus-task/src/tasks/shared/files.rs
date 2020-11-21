use clap::Arg;

pub fn arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("files")
        .long("files")
        .required(true)
        .takes_value(true)
        .help("All added and modified files.")
}
