use std::fmt::{Display, Formatter};
use std::io::Error as StdError;
pub type Result<T> = std::result::Result<T, Error>;
pub type TriState<T> = crate::tri_state::TriState<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(StdError),
    Pid(String),
    Usage(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::Pid(e) => f.write_str(e.as_str()),
            Error::Usage(e) => f.write_str(e.as_str()),
        }
    }
}

impl From<StdError> for Error {
    fn from(e: StdError) -> Self {
        Error::Io(e)
    }
}
