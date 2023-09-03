use std::{ffi::c_int, marker::PhantomData};

use crate::{bindings::*, utils::to_result, utils::Result};

use super::File;

pub struct FileIterator<'a> {
    handle: SqshFileIterator,
    _phantom: PhantomData<&'a SqshArchive>,
}

impl<'a> FileIterator<'a> {
    pub fn new(_file: &File<'a>, file_handle: *const SqshFile) -> Result<Self> {
        let mut handle = SqshFileIterator::default();

        let rv = unsafe { sqsh__file_iterator_init(&mut handle, file_handle) };

        Ok(Self {
            handle: to_result(handle, rv)?,
            _phantom: PhantomData,
        })
    }
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut rv = 0;

        let has_next = unsafe {
            sqsh_file_iterator_next(
                &mut self.handle,
                usize::MAX,
                &mut rv as *mut c_int,
            )
        };

        to_result((), rv).ok()?;

        if !has_next {
            return None;
        }

        unsafe {
            let data = sqsh_file_iterator_data(&self.handle);
            let size = sqsh_file_iterator_size(&self.handle);

            Some(std::slice::from_raw_parts(data, size))
        }
    }
}
