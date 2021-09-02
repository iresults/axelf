use crate::pid_file::{Pid, PidFile};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Launcher<'a> {
    pid_file: PidFile,
    executable: &'a str,
}

impl<'a> Launcher<'a> {
    pub fn new(pid_file: PidFile, executable: &'a str) -> Self {
        Self {
            pid_file,
            executable,
        }
    }

    pub fn launch(mut self) -> crate::error::Result<Pid> {
        let child = Command::new(self.executable)
            .stdout(Stdio::null())
            .spawn()?;

        let child_pid = child.id();
        self.pid_file.write_pid(child_pid)?;

        Ok(child_pid)
    }
}
