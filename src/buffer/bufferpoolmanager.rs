use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize};
use std::sync::{Arc, Mutex};

type PageId = u64;


#[derive(Copy, Clone, Debug, PartialEq)]
struct FrameHeader {}

#[derive(Copy, Clone, Debug, PartialEq)]
struct LRUKReplacer {}

struct BufferPoolManager {
    atomic_counter: AtomicUsize,
    latch: Arc<Mutex<()>>,
    frames: Vec<FrameHeader>,
    page_table: HashMap<PageId, usize>,
    free_frames: Vec<usize>,
    replacer: Box<LRUKReplacer>,
}
