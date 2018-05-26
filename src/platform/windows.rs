use error::*;

use std::os::windows::io::{AsRawHandle, RawHandle};
use std::path::{Path, PathBuf};

struct Handle(RawHandle);

unsafe impl Send for Handle {}

pub struct Sender {
    handle: Handle,
}

impl Drop for Sender {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl Sender {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Sender> {
        unimplemented!()
    }

    pub fn try_post(&self) -> Result<()> {
        unimplemented!()
    }
}

impl AsRawHandle for Sender {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle.0
    }
}

pub struct Receiver {
    handle: Handle,
    path: PathBuf,
}

impl Drop for Receiver {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl Receiver {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Receiver> {
        unimplemented!()
    }

    pub fn try_wait(&self) -> Result<()> {
        unimplemented!()
    }
}

impl AsRawHandle for Receiver {
    fn as_raw_handle(&self) -> RawHandle {
        self.handle.0
    }
}
