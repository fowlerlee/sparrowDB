use common::types::PageId;
use common::types::TxnResult;
use file_system::file::File;
use std::io::{Read, Result, Write};
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub struct DiskManager {
    file: File, // FIXME : check if we should own or reference
    pages: Arc<Mutex<usize>>,
    capacity: usize,
}

#[allow(dead_code)]
impl DiskManager {
    pub fn new(file: File) -> Self {
        Self {
            file,
            pages: Arc::new(Mutex::new(0)),
            capacity: 0,
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
        let buffer = [0; 64];
        page_data[..].copy_from_slice(&buffer[..]);
        TxnResult::Ok(self.file.write(&buffer[..]).unwrap())
    }

    pub fn increase_disk_space(&mut self, pages: usize) {
        let mut guard = self.pages.lock().unwrap();
        if *guard < self.capacity {
            return;
        }
        *guard = pages;
        while self.capacity < pages {
            self.capacity *= 2;
        }
        self.file.set_len(self.capacity as u64).unwrap();
    }

    pub fn shutdown(&self) {}
}
