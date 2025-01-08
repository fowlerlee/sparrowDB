use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

type PageId = u64;

#[derive(Copy, Clone, Debug, PartialEq)]
struct FrameHeader {}

// Evict() -> std::optional<frame_id_t> : Evict the frame that has the largest backward k-distance compared to all other evictable frames being tracked by the Replacer. If there are no evictable frames, return std::nullopt.
// RecordAccess(frame_id_t frame_id) : Record that the given frame has been accessed at the current timestamp. This method should be called after a page has been pinned in the BufferPoolManager.
// Remove(frame_id_t frame_id) : Clear all access history associated with a frame. This method should be called only when a page is deleted in the BufferPoolManager.
// SetEvictable(frame_id_t frame_id, bool set_evictable) : This method controls whether a frame is evictable or not. It also controls the LRUKReplacer's size. You'll know when to call this function when you implement the BufferPoolManager. To be specific, when the pin count of a page hits 0, its corresponding frame should be marked as evictable.
// Size() -> size_t : This method returns the number of evictable frames that are currently in the LRUKReplacer.

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

// NewPage() -> page_id_t
// DeletePage(page_id_t page_id) -> bool
// CheckedWritePage(page_id_t page_id) -> std::optional<WritePageGuard>
// CheckedReadPage(page_id_t page_id) -> std::optional<ReadPageGuard>
// FlushPage(page_id_t page_id) -> bool
// FlushAllPages()
// GetPinCount(page_id_t page_id)

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
