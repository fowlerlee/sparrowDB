use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

type PageId = u64;

#[derive(Copy, Clone, Debug, PartialEq)]
struct FrameHeader {}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
struct LRUKReplacer {}

impl LRUKReplacer {
    fn new() -> Self {
        Self::default()
    }

    fn Evict(&self) {
        eprint!("unimplemented");
    }
}

#[derive(Default)]
pub struct BufferPoolManager {
    atomic_counter: AtomicUsize,
    latch: Arc<Mutex<()>>,
    frames: Vec<FrameHeader>,
    page_table: HashMap<PageId, usize>,
    free_frames: Vec<usize>,
    replacer: Box<LRUKReplacer>,
}

impl BufferPoolManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            atomic_counter: AtomicUsize::new(0),
            latch: Arc::new(Mutex::new(())),
            frames: Vec::new(),
            page_table: HashMap::with_capacity(capacity),
            free_frames: Vec::new(),
            replacer: Box::new(LRUKReplacer::default()),
        }
    }

    pub fn getBufferPoolSize(&self) -> usize {
        self.frames.len()
    }
}
