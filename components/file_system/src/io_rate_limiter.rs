use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct IoRateLimiter {}

impl IoRateLimiter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
    #[allow(dead_code)]
    #[allow(unused_variables, unused_mut)]
    pub fn request(&self, io_type: IoType, io_op: IoOp, mut bytes: usize) -> usize {
        1usize
    }
}

// FIXME: should not assume we default flush
#[derive(Default)]
pub enum IoType {
    #[default]
    Flush = 3,
    #[allow(dead_code)]
    Compaction = 5,
}

pub enum IoOp {
    Read,
    Write,
}

lazy_static! {
    static ref IO_RATE_LIMITER: Mutex<Option<Arc<IoRateLimiter>>> = Mutex::new(None);
}

pub fn get_io_rate_limiter() -> Option<Arc<IoRateLimiter>> {
    (*IO_RATE_LIMITER.lock()).clone()
}
