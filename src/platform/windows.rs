use error::*;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, RawHandle};
use std::path::{Path, PathBuf};
use std::ptr;

use winapi::um::fileapi as file;
use winapi::um::handleapi as handle;
use winapi::um::namedpipeapi as namedpipe;
use winapi::um::winbase;
use winapi::um::winnt;

struct Handle(RawHandle);

unsafe impl Send for Handle {}

trait ToWideBytes {
    fn to_wide_bytes(&self) -> Vec<u16>;
}

impl ToWideBytes for OsStr {
    fn to_wide_bytes(&self) -> Vec<u16> {
        self.encode_wide()
            .chain(Some(0).into_iter())
            .collect()
    }
}

fn place_fifo<P: AsRef<Path>>(p: &P) -> Result<PathBuf> {
    super::super::validate_path(p)?;
    let mut output = PathBuf::from(r"\\.\pipe\");
    output.push(p);
    Ok(output)
}

pub struct Sender {
    handle: Handle,
}

impl Drop for Sender {
    fn drop(&mut self) {
        unsafe {
            handle::CloseHandle(self.handle.0);
        }
    }
}

impl Sender {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Sender> {
        let path = place_fifo(&path)?;

        let path_bytes = path.as_os_str().to_wide_bytes();

        let result = unsafe {
            file::CreateFileW(
                // lpFileName
                path_bytes.as_ptr(),

                // dwDesiredAccess
                winnt::GENERIC_WRITE,

                // dwShareMode
                0,

                // lpSecurityAttributes
                ptr::null_mut(),

                // dwCreationDisposition
                file::OPEN_EXISTING,

                // dwFlagsAndAttributes
                winbase::FILE_FLAG_OVERLAPPED,

                // hTemplateFile
                ptr::null_mut(),
            )
        };

        if handle::INVALID_HANDLE_VALUE == result {
            panic!("got invalid handle");
        }

        let result = Sender {
            handle: Handle(result),
        };

        Ok(result)
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
        unsafe {
            handle::CloseHandle(self.handle.0);
        }
    }
}

impl Receiver {
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Receiver> {
        let path = place_fifo(&path)?;

        let path_bytes = path.as_os_str().to_wide_bytes();

        let result = unsafe {
            namedpipe::CreateNamedPipeW(
                // lpName
                path_bytes.as_ptr(),

                // dwOpenMode
                winbase::PIPE_ACCESS_INBOUND
                    | winbase::FILE_FLAG_FIRST_PIPE_INSTANCE
                    | winbase::FILE_FLAG_OVERLAPPED,

                // dwPipeMode
                winbase::PIPE_TYPE_BYTE
                    | winbase::PIPE_READMODE_BYTE
                    | winbase::PIPE_REJECT_REMOTE_CLIENTS,

                // nMaxInstances
                1,

                // nOutBufferSize,
                1024,

                // nInBufferSize,
                1024,

                // nDefaultTimeout,
                0,

                // lpSecurityAttributes,
                ptr::null_mut(),
            )
        };

        if handle::INVALID_HANDLE_VALUE == result {
            panic!("Got an invalid handle");
        }

        let result = Receiver {
            handle: Handle(result),
            path,
        };

        Ok(result)
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
