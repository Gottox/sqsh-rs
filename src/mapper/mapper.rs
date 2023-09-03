use std::{
    ffi::{c_void, CString},
    path::PathBuf,
};

use crate::bindings::*;

#[derive(Debug)]
pub struct Source<T> {
    mapper_impl: *const SqshMemoryMapperImpl,
    size: usize,
    store: T,
}

pub trait SourcePtr {
    fn as_ptr(&self) -> *const c_void;
}

impl From<PathBuf> for Source<Vec<u8>> {
    fn from(value: PathBuf) -> Self {
        Self::from(value.to_str().unwrap())
    }
}

impl From<String> for Source<Vec<u8>> {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Source<Vec<u8>> {
    fn from(value: &str) -> Self {
        let path = CString::new(value).unwrap();
        let store = path.as_bytes().to_vec();
        let mapper_impl = unsafe { sqsh_mapper_impl_mmap };
        Self {
            mapper_impl,
            size: 0,
            store,
        }
    }
}

/*impl<'a> From<&'a [u8]> for Source<&'a [u8]> {
    fn from(value: &'a [u8]) -> Self {
        let mapper_impl = unsafe { sqsh_mapper_impl_static };
        let store = value;
        let size = value.len();
        Self {
            mapper_impl,
            size,
            store,
        }
    }
}*/

impl<T> Source<T> {
    pub(crate) fn mapper_impl(&self) -> *const SqshMemoryMapperImpl {
        self.mapper_impl
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }
}

impl SourcePtr for Source<Vec<u8>> {
    fn as_ptr(&self) -> *const c_void {
        self.store.as_ptr() as *const c_void
    }
}

impl SourcePtr for Source<&[u8]> {
    fn as_ptr(&self) -> *const c_void {
        self.store.as_ptr() as *const c_void
    }
}
