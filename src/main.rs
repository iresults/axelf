use crate::error::{Error, TriState};
use crate::launcher::Launcher;
use crate::pid_file::{Pid, PidFile};
use std::env;

extern crate log;

mod checker;
mod error;
mod launcher;
mod pid_file;
mod tri_state;

fn main() {
    env_logger::init();

    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        let message = format!(
            r"Axel F will take care that executable is running

Usage {} /path/to/executable /path/to/pid-file",
            &args[0]
        );

        return Err(error::Error::Usage(message));
    }

    let executable_path = &args[1];
    let pid_file_path = &args[2];

    let mut pid_file = PidFile::open(pid_file_path)?;

    let must_start_process = match pid_file.read_pid() {
        TriState::Some(pid) => {
            if check_if_process_is_running(pid, executable_path)? {
                log::info!("Process with PID {} is running -> don't start process", pid);
                false
            } else {
                log::info!("Process with PID {} not running -> must start process", pid);
                true
            }
        }
        TriState::None => {
            log::info!("No PID stored -> must start process");
            true
        }
        TriState::Error(e) => return Err(e),
    };

    if must_start_process {
        let launcher = Launcher::new(pid_file, executable_path);
        let pid = launcher.launch()?;
        log::info!("Launched process with PID {}", pid);
    }
    Ok(())
}

fn check_if_process_is_running(pid: Pid, executable_path: &str) -> crate::error::Result<bool> {
    let checker = checker::ProcessChecker::new();
    checker.is_running(pid, executable_path)
}
