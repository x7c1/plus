use clap::Arg;

pub fn arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("target")
        .long("target")
        .required(true)
        .takes_value(true)
        .help("Build for the target label.")
}
