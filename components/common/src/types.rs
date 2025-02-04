use std::sync::atomic::AtomicU64;

#[derive(Clone, Debug, PartialEq)]
pub enum TxnResult<T, E> {
    Ok(T),
    Err(E),
}

pub type PageId = usize;

#[derive(Debug)]
#[allow(dead_code)]
pub struct FrameHeader {
    is_dirty: bool,
    data: Vec<u8>,
    frame_id: usize,
    pin_count: AtomicU64,
}

impl Default for FrameHeader {
    fn default() -> Self {
        Self {
            is_dirty: false,
            data: vec![0],
            frame_id: 0,
            pin_count: AtomicU64::new(0),
        }
    }
}

impl FrameHeader {
    #[allow(dead_code)]
    fn new(frame_id: usize) -> Self {
        Self {
            is_dirty: false,
            data: vec![0],
            frame_id,
            pin_count: AtomicU64::new(0),
        }
    }
}
