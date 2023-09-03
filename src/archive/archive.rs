use std::{ffi::c_int, path::PathBuf};

use crate::{
    bindings::*,
    file::File,
    mapper::{Source, SourcePtr},
    utils::{to_result, Result},
};

use super::superblock::Superblock;

pub struct Archive<S> {
    pub(crate) handle: SqshArchive,
    _source: Source<S>,
}

#[derive(Debug)]
pub struct ArchiveBuilder<S> {
    config: SqshConfig,
    source: Source<S>,
}

impl<S: Default> ArchiveBuilder<S>
where
    Source<S>: SourcePtr,
{
    pub fn new<I: Into<Source<S>>>(source: I) -> Self {
        let source = source.into();
        Self {
            config: SqshConfig {
                source_size: source.size() as u64,
                source_mapper: source.mapper_impl(),
                ..Default::default()
            },
            source,
        }
    }
    pub fn archive_offset(mut self, offset: u64) -> Self {
        self.config.archive_offset = offset;
        self
    }
    pub fn mapper_block_size<I: Into<Option<u64>>>(
        mut self,
        mapper_block_size: I,
    ) -> Self {
        self.config.mapper_block_size =
            mapper_block_size.into().unwrap_or(0) as c_int;
        self
    }
    pub fn mapper_lru_size<I: Into<Option<u64>>>(
        mut self,
        mapper_lru_size: I,
    ) -> Self {
        self.config.mapper_lru_size =
            mapper_lru_size.into().unwrap_or(0) as c_int;
        self
    }
    pub fn compression_lru_size<I: Into<Option<u64>>>(
        mut self,
        compression_lru_size: I,
    ) -> Self {
        self.config.compression_lru_size =
            compression_lru_size.into().unwrap_or(0) as c_int;
        self
    }
    pub fn max_symlink_depth<I: Into<Option<u64>>>(
        mut self,
        max_symlink_depth: I,
    ) -> Self {
        self.config.max_symlink_depth =
            max_symlink_depth.into().unwrap_or(0) as usize;
        self
    }

    pub fn build(self) -> Result<Archive<S>> {
        Archive::new(self.source, self.config)
    }
}

impl<S> Archive<S>
where
    Source<S>: SourcePtr,
{
    fn new(source: Source<S>, config: SqshConfig) -> Result<Self> {
        let mut handle: SqshArchive = Default::default();
        let rv = unsafe {
            sqsh__archive_init(&mut handle, source.as_ptr(), &config)
        };
        Ok(Self {
            handle: to_result(handle, rv)?,
            _source: source,
        })
    }

    pub fn superblock<'a>(&'a self) -> Superblock<S> {
        Superblock::new(&self.handle)
    }

    pub fn open<P: Into<PathBuf>>(&mut self, path: P) -> Result<File> {
        File::open(&mut self.handle, path.into())
    }
}

impl<S> Drop for Archive<S> {
    fn drop(&mut self) {
        unsafe { sqsh__archive_cleanup(&mut self.handle) };
    }
}
