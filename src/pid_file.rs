use crate::error::{Error, TriState};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub type Pid = u32;

#[derive(Debug)]
pub struct PidFile {
    file: File,
}

impl PidFile {
    pub fn open<P: AsRef<Path>>(path: P) -> crate::error::Result<PidFile> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.as_ref())?;

        Ok(Self { file })
    }

    pub fn read_pid(&mut self) -> crate::error::TriState<Pid> {
        let mut text = String::new();
        let bytes_read = match self.file.read_to_string(&mut text) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                return TriState::Error(e.into());
            }
        };

        if bytes_read > 0 {
            println!("{}", bytes_read);

            match text.trim().parse::<Pid>() {
                Ok(e) => TriState::Some(e),
                Err(_) => TriState::Error(Error::Pid(format!(
                    "Could not parse PID file content '{}'",
                    text
                ))),
            }
        } else {
            TriState::None
        }
    }

    pub fn write_pid(&mut self, pid: Pid) -> crate::error::Result<()> {
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(pid.to_string().as_bytes())?;

        Ok(())
    }
}
