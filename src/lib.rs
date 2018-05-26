#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        extern crate nix;
        extern crate xdg;
    }
}

pub mod error;
mod platform;

use error::*;

pub use platform::{Receiver, Sender};

use std::path::Path;

/// Attempts to prevent paths that would cause problems on any platform to
/// make porting between platforms less painful.
///
/// See:
///   * https://msdn.microsoft.com/en-us/library/windows/desktop/aa365150(v=vs.85).aspx
fn validate_path<P: AsRef<Path>>(path: &P) -> Result<()> {
    let txt = match path.as_ref().to_str() {
        Some(x) => x,
        None => return Err(Error::InvalidPath("must be valid UTF-8")),
    };

    // Pipe paths on windows look like "\\.\pipe\pipename" and have a max of 256
    // characters.
    if txt.len() > 247 {
        return Err(Error::InvalidPath("must be less than 248 bytes long"));
    }

    // Also a windows restriction
    if txt.contains("\\") {
        return Err(Error::InvalidPath("must not contain backslash"));
    }

    // Forward slash is a directory separator in unix
    if txt.contains("/") {
        return Err(Error::InvalidPath("must not contain slash"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
