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

    pub(crate) fn read_page_data(&self, frame_id: usize) {
        let mut guard = self.operation.lock().unwrap();
        guard.read_data(frame_id);
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

    pub(crate) fn write_page_data(&mut self, frame_id: usize, data: Vec<u8>) -> Option<usize> {
        let mut guard = self.operation.lock().unwrap();
        if let Some(val) = guard.write_data(frame_id, data) {
            Some(val)
        } else {
            None
        }
    }
}
