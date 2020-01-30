extern crate failure;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "FileNotFound > {}", 0)]
    FileNotFound { path: String, description: String },
}
