use crate::error::{Error, TriState};
use crate::launcher::Launcher;
use crate::pid_file::{Pid, PidFile};
use std::env;
use std::env::Args;

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
    let arguments = get_arguments(env::args())?;
    let mut pid_file = PidFile::open(&arguments.pid_file_path)?;
    let (message, must_start_process) = check_process(&arguments.executable_path, &mut pid_file)?;

    match arguments.command.as_deref() {
        Some("check") => {
            println!("{}", message);
        }
        Some("help") => {
            println!("{}", get_help(&arguments));
        }
        _ => {
            log::info!("{}", message);
            if must_start_process {
                let launcher = Launcher::new(pid_file, &arguments.executable_path);
                let pid = launcher.launch()?;
                log::info!("Launched process with PID {}", pid);
            }
        }
    }
    Ok(())
}

fn get_help(args: &Arguments) -> String {
    format!(
        r"Axel F will take care that executable is running

Usage {} /path/to/executable /path/to/pid-file",
        args.self_path
    )
}

fn check_process(executable_path: &str, pid_file: &mut PidFile) -> Result<(String, bool), Error> {
    match pid_file.read_pid() {
        TriState::Some(pid) => {
            if check_if_process_is_running(pid, executable_path)? {
                Ok((
                    format!("Process with PID {} is running -> don't start process", pid),
                    false,
                ))
            } else {
                Ok((
                    format!("Process with PID {} not running -> must start process", pid),
                    true,
                ))
            }
        }
        TriState::None => Ok((format!("No PID stored -> must start process"), true)),
        TriState::Error(e) => Err(e),
    }
}

struct Arguments {
    self_path: String,
    executable_path: String,
    pid_file_path: String,
    command: Option<String>,
}

fn get_arguments(args: Args) -> Result<Arguments, Error> {
    let mut self_path = None;
    let mut executable_path = None;
    let mut pid_file_path = None;
    let mut command = None;

    for arg in args {
        if self_path.is_none() {
            // The very first argument is the executable path
            self_path = Some(arg.to_owned());
        } else if arg.starts_with("--") {
            command = arg.strip_prefix("--").map(str::to_owned)
        } else if executable_path.is_none() {
            // The first argument without "--" is the executable path
            executable_path = Some(arg.to_owned());
        } else if pid_file_path.is_none() {
            // The second argument without "--" is the PID file path
            pid_file_path = Some(arg.to_owned());
        } else {
            return Err(Error::Usage("To many arguments".to_owned()));
        }
    }

    if self_path.is_none() {
        return Err(Error::Usage("Missing self path".to_owned()));
    }
    if executable_path.is_none() {
        return Err(Error::Usage(
            "Missing argument 'executable_path'".to_owned(),
        ));
    }
    if pid_file_path.is_none() {
        return Err(Error::Usage("Missing argument 'pid_file'".to_owned()));
    }

    Ok(Arguments {
        self_path: self_path.unwrap(),
        executable_path: executable_path.unwrap(),
        pid_file_path: pid_file_path.unwrap(),
        command,
    })
}

fn check_if_process_is_running(pid: Pid, executable_path: &str) -> crate::error::Result<bool> {
    let checker = checker::ProcessChecker::new();
    checker.is_running(pid, executable_path)
}
