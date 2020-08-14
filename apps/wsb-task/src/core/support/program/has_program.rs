use shellwork::core::command::{Runner, Unprepared};

pub trait HasProgram {
    fn get_program(&self) -> &str;
}

impl HasProgram for Runner<Unprepared> {
    fn get_program(&self) -> &str {
        self.get_program()
    }
}

impl HasProgram for String {
    fn get_program(&self) -> &str {
        self
    }
}
