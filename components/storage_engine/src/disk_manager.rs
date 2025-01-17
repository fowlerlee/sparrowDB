use crate::page::PageId;
use file_system::file::File;
use std::io::{Read, Result};

#[allow(dead_code)]
pub struct DiskManager {
    file: File, // FIXME : check if we should own or reference
    pages: usize,
}

#[allow(dead_code)]
impl DiskManager {
    pub fn new(file: File) -> Self {
        Self {
            file,
            pages: 0usize,
        }
    }

    pub fn read_page(&mut self, _pageid: PageId, page_data: &mut [u8]) -> Result<usize> {
        let mut buffer = [0; 64];
        let n = self.file.read(&mut buffer[..])?;
        page_data[..n].copy_from_slice(&buffer[..n]);
        Ok(n)
    }
}
