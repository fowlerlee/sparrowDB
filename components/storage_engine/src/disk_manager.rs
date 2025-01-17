use crate::page::PageId;
use common::types::TxnResult;
use file_system::file::File;
use std::io::{Read, Result, Write};

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

    // FIXME: must read at pageid
    pub fn read_page(&mut self, _pageid: PageId, page_data: &mut [u8]) -> Result<usize> {
        let mut buffer = [0; 64];
        let n = self.file.read(&mut buffer[..])?;
        page_data[..n].copy_from_slice(&buffer[..n]);
        Ok(n)
    }

    // FIXME: must write at pageid
    pub fn write_page(&mut self, _pageid: PageId, page_data: &mut [u8]) -> TxnResult<usize, ()> {
        let mut buffer = [0; 64];
        page_data[..].copy_from_slice(&buffer[..]);
        TxnResult::Ok(self.file.write(&mut buffer[..]).unwrap())
    }
}
