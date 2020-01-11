use clap::{App, ArgMatches};

// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/

pub mod put_object;

pub struct Definition<'a, 'b, T> {
    pub name: String,
    pub define: fn() -> App<'a, 'b>,
    pub run: fn(matches: &ArgMatches) -> T,
}

pub struct Task<'a, 'b, T> {
    pub definition: &'a Definition<'a, 'b, T>,
    pub matches: &'a ArgMatches<'a>,
}

impl<T> Task<'_, '_, T> {
    pub fn run(&self) -> T {
        let run = self.definition.run;
        run(self.matches)
    }
}
