use std::ffi::c_int;
use std::ffi::CString;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::bindings::*;
use crate::utils::to_result;
use crate::utils::Result;

use super::FileIterator;
use super::FileReader;

pub struct File<'a> {
    handle: *mut SqshFile,
    _phantom: PhantomData<&'a SqshArchive>,
}

impl<'a> File<'a> {
    pub fn open(
        archive_handle: &'a mut SqshArchive,
        path: PathBuf,
    ) -> Result<Self> {
        let mut rv = 0;
        let path = CString::new(path.to_str().unwrap()).unwrap();
        let handle = unsafe {
            sqsh_open(archive_handle, path.as_ptr(), &mut rv as *mut c_int)
        };
        Ok(Self {
            handle: to_result(handle, rv)?,
            _phantom: PhantomData,
        })
    }

    pub fn reader(&self) -> Result<FileReader<'a>> {
        FileReader::new(self, self.handle)
    }
}

impl<'a> IntoIterator for &File<'a> {
    type Item = &'a [u8];
    type IntoIter = FileIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let handle = self.handle;
        FileIterator::new(self, handle).unwrap()
    }
}

impl Drop for File<'_> {
    fn drop(&mut self) {
        unsafe {
            sqsh_close(self.handle);
        }
    }
}
