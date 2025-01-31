use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};

pub type PageId = usize;
// Note: we use u8 not char, so its mem efficient but we have
// to do String::from_utf8(data.clone()).expect("Invalid UTF-8 data")
#[derive(Default, Debug)]
pub struct Page {
    #[allow(dead_code)]
    id: usize,
    pub data: Mutex<Vec<u8>>,
    pin_count: AtomicUsize,
    is_dirty: AtomicBool,
}

impl Page {
    pub fn new(id: usize, data: Vec<u8>) -> Self {
        Self {
            id,
            data: Mutex::new(data),
            pin_count: AtomicUsize::new(0),
            is_dirty: AtomicBool::new(false),
        }
    }
    #[allow(dead_code)]
    fn reset_memory(&mut self) {
        self.id = 0usize;
        let mut mutex_guard = self.lock_data();
        mutex_guard.clear();
        self.is_dirty.store(false, Ordering::SeqCst);
        self.pin_count.store(0, Ordering::SeqCst);
    }

    pub fn mark_dirty(&self) {
        self.is_dirty.store(true, Ordering::Release);
    }

    pub fn pin(&self) {
        self.pin_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn unpin(&self) {
        self.pin_count.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn get_pin_count(&self) -> usize {
        self.pin_count.load(Ordering::Acquire)
    }

    pub fn lock_data(&self) -> MutexGuard<Vec<u8>> {
        self.data.lock().expect("Mutex lock failed")
    }
}
