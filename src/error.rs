use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    WouldBlock,
    NotFound,
    AlreadyExists(PathBuf),
    InvalidPath(&'static str),
    Io(IoError),
    Os(Box<StdError>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::AlreadyExists(ref x) => {
                write!(f, "{}: {}", self.description(), x.to_string_lossy())
            }
            Error::InvalidPath(x) => write!(f, "{}: {}", self.description(), x),
            Error::Io(ref x) => x.fmt(f),
            Error::Os(ref x) => x.fmt(f),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidPath(_) => "the given path may not be valid",
            Error::WouldBlock => "the attempted operation would block",
            Error::AlreadyExists(_) => "the given name is already in use",
            Error::NotFound => "the given name could not be found",
            Error::Io(ref x) => x.description(),
            Error::Os(ref x) => x.description(),
        }
    }
}

impl From<IoError> for Error {
    fn from(n: IoError) -> Error {
        Error::Io(n)
    }
}
