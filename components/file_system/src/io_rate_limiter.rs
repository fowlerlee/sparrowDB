pub struct IoRateLimiter {}

impl IoRateLimiter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn request(&self, io_type: IoType, io_op: IoOp, mut bytes: usize) -> usize {
        1usize
    }
}

// FIXME: should not assume we default flush
#[derive(Default)]
pub enum IoType {
    #[default]
    Flush = 3,
    Compaction = 5,
}

pub enum IoOp {
    Read,
}
