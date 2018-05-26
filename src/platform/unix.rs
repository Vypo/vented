use error::*;

use nix::errno::Errno;
use nix::Error as NixError;
use nix::fcntl::{self, OFlag};
use nix::sys::stat;
use nix::unistd;

use std::os::unix::io::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};

use xdg::{BaseDirectories, BaseDirectoriesError};

impl From<BaseDirectoriesError> for Error {
    fn from(n: BaseDirectoriesError) -> Error {
        Error::Os(Box::new(n))
    }
}

impl From<NixError> for Error {
    fn from(n: NixError) -> Error {
        match n {
            NixError::Sys(Errno::EAGAIN) => Error::WouldBlock,
            NixError::Sys(Errno::ENOENT) => Error::NotFound,
            other => Error::Os(Box::new(other))
        }
    }
}

fn place_fifo<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    super::super::validate_path(&path)?;
    let dirs = BaseDirectories::new()?;
    Ok(dirs.place_runtime_file(path)?)
}

pub struct Sender {
    fd: RawFd,
}

impl Drop for Sender {
    fn drop(&mut self) {
        unistd::close(self.fd).ok();
    }
}

impl Sender {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Sender> {
        let path = place_fifo(path)?;

        let result = Sender {
            fd: fcntl::open(&path,
                            OFlag::O_NONBLOCK | OFlag::O_WRONLY | OFlag::O_CLOEXEC,
                            stat::Mode::S_IRWXU)?
        };

        Ok(result)
    }

    pub fn try_post(&self) -> Result<()> {
        let wrote = unistd::write(self.fd, &[0; 1])?;

        if 1 != wrote {
            panic!("should have written exactly one byte");
        }

        Ok(())
    }
}

impl AsRawFd for Sender {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

pub struct Receiver {
    fd: RawFd,
    path: PathBuf,
}

impl Drop for Receiver {
    fn drop(&mut self) {
        unistd::unlink(&self.path).ok();
        unistd::close(self.fd).ok();
    }
}

impl Receiver {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Receiver> {
        // TODO: Should be using mkfifoat and openat (probably)
        let path = place_fifo(path)?;

        match unistd::mkfifo(&path, stat::Mode::S_IRWXU) {
            Ok(_) => (),
            Err(NixError::Sys(Errno::EEXIST)) => {
                return Err(Error::AlreadyExists(path.to_owned()));
            }
            Err(err) => return Err(err.into()),
        }

        let result = Receiver {
            fd: fcntl::open(&path,
                            OFlag::O_NONBLOCK | OFlag::O_RDONLY | OFlag::O_CLOEXEC,
                            stat::Mode::S_IRWXU)?,
            path: path,
        };

        Ok(result)
    }

    pub fn try_wait(&self) -> Result<()> {
        let mut buf = [0u8];
        let result = unistd::read(self.fd, &mut buf)?;

        match result {
            0 => Err(Error::WouldBlock),
            1 => Ok(()),
            _ => panic!("should have received exactly one byte"),
        }
    }
}

impl AsRawFd for Receiver {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}
