use std::marker::PhantomData;

use crate::bindings::*;

use super::archive::Archive;

pub struct Superblock<'a, S> {
    pub(crate) handle: *const SqshSuperblock,
    pub(crate) _phantom: PhantomData<&'a Archive<S>>,
}

impl<S> Superblock<'_, S> {
    pub(crate) fn new<'a>(archive_handle: &'a SqshArchive) -> Self {
        let handle = unsafe { sqsh_archive_superblock(archive_handle) };

        Self {
            handle,
            _phantom: PhantomData,
        }
    }

    pub fn compression_id(&self) -> SqshSuperblockCompressionId {
        unsafe { sqsh_superblock_compression_id(self.handle) }
    }

    pub fn directory_table_start(&self) -> u64 {
        unsafe { sqsh_superblock_directory_table_start(self.handle) }
    }

    pub fn fragment_table_start(&self) -> Option<u64> {
        let has_fragments =
            unsafe { sqsh_superblock_has_fragments(self.handle) };
        if has_fragments {
            Some(unsafe { sqsh_superblock_fragment_table_start(self.handle) })
        } else {
            None
        }
    }

    pub fn inode_count(&self) -> u32 {
        unsafe { sqsh_superblock_inode_count(self.handle) }
    }

    pub fn version_major(&self) -> u16 {
        unsafe { sqsh_superblock_version_major(self.handle) }
    }

    pub fn version_minor(&self) -> u16 {
        unsafe { sqsh_superblock_version_minor(self.handle) }
    }

    pub fn inode_table_start(&self) -> u64 {
        unsafe { sqsh_superblock_inode_table_start(self.handle) }
    }

    pub fn id_table_start(&self) -> u64 {
        unsafe { sqsh_superblock_id_table_start(self.handle) }
    }

    pub fn id_count(&self) -> u16 {
        unsafe { sqsh_superblock_id_count(self.handle) }
    }

    pub fn export_table_start(&self) -> Option<u64> {
        let has_exports =
            unsafe { sqsh_superblock_has_export_table(self.handle) };
        if has_exports {
            Some(unsafe { sqsh_superblock_export_table_start(self.handle) })
        } else {
            None
        }
    }

    pub fn xattr_id_table_start(&self) -> Option<u64> {
        let has_xattrs =
            unsafe { sqsh_superblock_has_xattr_table(self.handle) };
        if has_xattrs {
            Some(unsafe { sqsh_superblock_xattr_id_table_start(self.handle) })
        } else {
            None
        }
    }

    pub fn inode_root_ref(&self) -> u64 {
        unsafe { sqsh_superblock_inode_root_ref(self.handle) }
    }

    pub fn has_compression_options(&self) -> bool {
        unsafe { sqsh_superblock_has_compression_options(self.handle) }
    }

    pub fn block_size(&self) -> u32 {
        unsafe { sqsh_superblock_block_size(self.handle) }
    }

    pub fn modification_time(&self) -> u32 {
        unsafe { sqsh_superblock_modification_time(self.handle) }
    }

    pub fn fragment_entry_count(&self) -> u32 {
        unsafe { sqsh_superblock_fragment_entry_count(self.handle) }
    }

    pub fn bytes_used(&self) -> u64 {
        unsafe { sqsh_superblock_bytes_used(self.handle) }
    }
}
