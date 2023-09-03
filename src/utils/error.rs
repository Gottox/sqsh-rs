use std::{
    ffi::{c_int, CStr},
    fmt::{self, Formatter},
};

use crate::bindings::*;

#[derive(Debug)]
pub struct Error {
    handle: SqshError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let handle = self.handle as i32;
        let err_str = unsafe { CStr::from_ptr(sqsh_error_str(handle)) };
        let err_str = err_str.to_str().unwrap();
        write!(f, "{}", err_str)
    }
}

impl Error {
    pub(crate) fn from_c_err(error: c_int) -> Self {
        let handle = error as SqshError;
        Self { handle }
    }
}

impl std::error::Error for Error {}
