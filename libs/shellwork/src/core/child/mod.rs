use std::process::{Child, ExitStatus};

pub struct ExitedProcess {
    #[allow(dead_code)]
    status: ExitStatus,

    #[allow(dead_code)]
    raw: Child,
}

impl ExitedProcess {
    pub fn wait(mut child: Child) -> crate::Result<Self> {
        let status = child.wait()?;
        let process = ExitedProcess { status, raw: child };
        Ok(process)
    }
    pub fn success(&self) -> bool {
        self.status.success()
    }
    pub fn status_code(&self) -> Option<i32> {
        self.status.code()
    }
}
