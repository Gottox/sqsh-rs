use std::{io::Read, marker::PhantomData};

use crate::{bindings::*, utils::to_result, utils::Result};

use super::File;

pub struct FileReader<'a> {
    handle: SqshFileReader,
    _phantom: PhantomData<&'a SqshArchive>,
}

impl<'a> FileReader<'a> {
    pub fn new(_file: &File<'a>, file_handle: *const SqshFile) -> Result<Self> {
        let mut handle = SqshFileReader::default();

        let rv = unsafe { sqsh__file_reader_init(&mut handle, file_handle) };

        Ok(Self {
            handle: to_result(handle, rv)?,
            _phantom: PhantomData,
        })
    }

    pub fn advance(&mut self, offset: usize, size: usize) -> Result<()> {
        let rv =
            unsafe { sqsh_file_reader_advance(&mut self.handle, offset, size) };

        to_result((), rv)
    }

    pub fn data(&self) -> &[u8] {
        // TODO: advance invalides these pointers.
        unsafe {
            let data = sqsh_file_reader_data(&self.handle);
            let size = sqsh_file_reader_size(&self.handle);
            std::slice::from_raw_parts(data, size)
        }
    }

    pub fn size(&self) -> usize {
        unsafe { sqsh_file_reader_size(&self.handle) }
    }
}

impl Read for FileReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let current_size = self.size();

        self.advance(current_size, buf.len()).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

        let data = self.data();
        buf[..data.len()].copy_from_slice(data);

        return Ok(data.len());
    }
}

impl Drop for FileReader<'_> {
    fn drop(&mut self) {
        unsafe { sqsh__file_reader_cleanup(&mut self.handle) };
    }
}
