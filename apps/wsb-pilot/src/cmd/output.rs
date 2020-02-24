use crate::PilotResult;
use serde_json::Value;
use std::process::Output;

pub struct CommandOutput {
    output: Output,
}

impl CommandOutput {
    pub fn new(output: Output) -> Self {
        CommandOutput { output }
    }

    pub fn stdout(&self) -> &[u8] {
        &self.output.stdout
    }

    pub fn stderr(&self) -> &[u8] {
        &self.output.stderr
    }

    pub fn status_code(&self) -> i32 {
        self.output
            .status
            .code()
            .expect("Process terminated by signal")
    }

    pub fn to_json(&self) -> PilotResult<Value> {
        let value: Value = serde_json::from_slice(self.stdout())?;
        Ok(value)
    }

    pub fn to_stdout_string(&self) -> String {
        String::from_utf8_lossy(self.stdout()).to_string()
    }

    pub fn dump(&self) {
        let to_string = |vec| String::from_utf8_lossy(vec).to_string();
        println!("{}", to_string(&self.stdout()));

        let e = to_string(&self.stderr());
        if e.len() > 0 {
            println!("stderr: {}", e);
        }
    }

    pub fn dump_if_failed(&self) {
        if !self.output.status.success() {
            self.dump();
        }
    }
}
