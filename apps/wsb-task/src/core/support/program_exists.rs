use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Runner, Unprepared};

pub fn program_exists(runner: &Runner<Unprepared>) -> TaskResult<()> {
    // https://stackoverflow.com/a/39983421
    let line = format!("command -v {}", runner.program);
    let runner = command::program("sh")
        .args(&["-c", &line])
        .prepare(no_op::<crate::Error>)?;

    runner.capture()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::core::support::program_exists;
    use crate::TaskResult;
    use shellwork::core::command;

    #[test]
    fn return_ok_if_program_exists() -> TaskResult<()> {
        let runner = command::program("ls");
        let result = runner.prepare(program_exists);
        assert_eq!(true, result.is_ok());

        let runner = command::program("mkdir");
        let result = runner.prepare(program_exists);
        assert_eq!(true, result.is_ok());
        Ok(())
    }

    #[test]
    fn return_error_if_program_not_exists() -> TaskResult<()> {
        let runner = command::program("unknown_command");
        let result = runner.prepare(program_exists);
        assert_eq!(true, result.is_err());
        Ok(())
    }
}
