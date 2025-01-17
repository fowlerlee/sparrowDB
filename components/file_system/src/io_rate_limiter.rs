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
}
