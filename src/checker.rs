use crate::pid_file::Pid;
use std::process::Command;

pub struct ProcessChecker {}

impl ProcessChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Check if the given executable is running
    pub fn is_running(&self, pid: Pid, executable_path: &str) -> crate::error::Result<bool> {
        let output = Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .arg("-opid,args")
            .output()?;

        let output_text = String::from_utf8_lossy(&output.stdout);
        log::debug!("ps command exited with status {}", output.status,);
        log::debug!("ps command output:\n{}", output_text);

        // No process for the PID was found
        if !output.status.success() {
            return Ok(false);
        }

        // Check if the information for the PID matches the expected executable
        if !output_text.contains(executable_path) {
            log::warn!(
                "PID {} does not match the given executable '{}'",
                pid,
                executable_path
            );
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl Default for ProcessChecker {
    fn default() -> Self {
        Self::new()
    }
}
