use common::types::FrameHeader;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub(crate) struct ReadPageGuard {
    is_valid: bool,
    operation: Arc<Mutex<FrameHeader>>,
}
#[allow(dead_code)]
impl ReadPageGuard {
    pub(crate) fn new() -> Self {
        Self {
            is_valid: true,
            operation: Arc::new(Mutex::new(FrameHeader::default())),
        }
    }

    pub(crate) fn read_page_data(&mut self, data: Vec<u8>) {
        let mut guard = self.operation.lock().unwrap();
        guard.read_data(data);
    }
}

#[allow(dead_code)]
pub(crate) struct WritePageGuard {
    is_valid: bool,
    operation: Arc<Mutex<FrameHeader>>,
}

impl WritePageGuard {
    pub(crate) fn new() -> Self {
        Self {
            is_valid: true,
            operation: Arc::new(Mutex::new(FrameHeader::default())),
        }
    }

    pub(crate) fn write_page_data(&mut self, data: Vec<u8>) {
        let mut guard = self.operation.lock().unwrap();
        guard.write_data(data);
    }
}
