use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

use crate::storage::page::Page;

type PageId = usize;
type FrameId = usize;

#[derive(Copy, Clone, Debug, PartialEq)]
struct FrameHeader {}

#[derive(Debug, Default, Clone, PartialEq)]
struct LRUNode {
    history: Vec<usize>,
    k_: usize,
    fid: FrameId,
    is_evictable: bool,
}

// Evict() -> std::optional<frame_id_t> : Evict the frame that has the largest backward k-distance compared to all other evictable frames being tracked by the Replacer. If there are no evictable frames, return std::nullopt.
// RecordAccess(frame_id_t frame_id) : Record that the given frame has been accessed at the current timestamp. This method should be called after a page has been pinned in the BufferPoolManager.
// Remove(frame_id_t frame_id) : Clear all access history associated with a frame. This method should be called only when a page is deleted in the BufferPoolManager.
// SetEvictable(frame_id_t frame_id, bool set_evictable) : This method controls whether a frame is evictable or not. It also controls the LRUKReplacer's size. You'll know when to call this function when you implement the BufferPoolManager. To be specific, when the pin count of a page hits 0, its corresponding frame should be marked as evictable.
// Size() -> size_t : This method returns the number of evictable frames that are currently in the LRUKReplacer.

// [[maybe_unused]] std::unordered_map<frame_id_t, LRUKNode> node_store_;
//   [[maybe_unused]] size_t current_timestamp_{0};
//   [[maybe_unused]] size_t curr_size_{0};
//   [[maybe_unused]] size_t replacer_size_;
//   [[maybe_unused]] size_t k_;
//   [[maybe_unused]] std::mutex latch_;

#[derive(Clone, Debug, Default)]
struct LRUKReplacer {
    node_store: HashMap<FrameId, LRUNode>,
    current_timestamp: usize,
    current_size: usize,
    replacer_size: usize,
    k_: usize,
    latch: Arc<Mutex<LRUNode>>,
}

impl LRUKReplacer {
    fn new(&self) -> Self {
        Self::default()
    }

    //     ? std::numeric_limits<size_t>::max()
    // : std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::system_clock::now().time_since_epoch())
    //           .count() -
    //       node.history_.front();

    // frame_id to evict func of k_dist = Max

    //  |1.K, 2.K...__________| node_state
    // Now | _t1M,_t3,_t4,_t2 LRUNODE1 | _t2,_t3,_t1,_t4 lRUNODE 2
    fn Evict(&mut self) -> Option<FrameId> {
        let mut victim_id: FrameId = 0usize;
        for (_k_, v) in self.node_store.iter_mut() {
            if v.is_evictable {
                // v.history[2] = std::time::SystemTime::now();
                let now = std::time::Instant::now().elapsed().as_nanos();

                v.k_ = now as usize - v.history[v.history.len() - 2] as usize;
            }
        }

        for (k, node) in self.node_store.iter() {
            if node.is_evictable && self.node_store[&k].k_ > self.node_store[&(k + 1)].k_ {
                victim_id = self.node_store.get(k).unwrap().fid;
            } else if node.is_evictable {
                victim_id = self.node_store.get(&(k+1)).unwrap().fid;
            }
            
        }

        for (k , node ) in self.node_store.iter_mut() {
            if  node.fid == victim_id {
            //    let frame = self.node_store.clone();
            }
        }


        return None;
    }

    fn RecordAccess(&self) {}

    fn Remove(&mut self) {}

    fn SetEvictable(&mut self, frame_id_t: FrameId, set_evictable: bool) {
        if let Some(x) = self.node_store.get_mut(&frame_id_t) {
            x.is_evictable = set_evictable;
        }
    }

    fn Size(&self) -> usize {
        return 1 as usize;
    }
}



pub struct BufferPoolManager {
    num_frames: usize,
    next_page: AtomicUsize,
    bpm_latch: Arc<Mutex<Page>>,
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
            num_frames: 0,
            next_page: AtomicUsize::new(0),
            bpm_latch: Arc::new(Mutex::new(Page::default())),
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
