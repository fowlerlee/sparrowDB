use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use storage_engine::page::{Page, PageId};

type FrameId = usize;

#[derive(Copy, Clone, Debug, PartialEq)]
struct FrameHeader {}

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

// Notes on algorithm:
//
// Schematic:
//     |------------->|A_k
//     |------>|B_k
// now |----------------------> time (_k=2) We Evict A with MAX k_b_d
//
// syntax: k_b_d == K-backward-distance where K = 2 in this algorithm
// 1. [MRU][A][B][C][D][E][LRU] -> Nodes/Pages
// 2. Access C then A -> [MRU][B][D][E][C][A][LRU] then eventually get order [A][C][E][B][D]
// 3. Evict LRU-K where K = 2 -> Eviction ==
// \A n \in Nodes: IF Node[n].k_b_d = MAXIMUM({Node[n].k_b_d})
//                 THEN Flush(Node[n]) to Disk/Stable storage
// 4. \A n \in Node[n].k_b_d == Time_now - Node[n].HISTORY[K] where K = 2 [this is 3rd element in vector]
// 5. \A n \in Nodes: IF Nodes[n].HISTORY.len() < 2 THEN Nodes[n].k_b_d = INFINITY
// 6. \A n, m \in Nodes: IF Nodes[n].k_b_d == Nodes[m].k_b_d == INIFINITY
//  THEN CHOOSE (n \in Nodes): MINIMUM(Nodes[n].HISTORY[0]) => EVICT (Nodes[n]) ... Flush to disk
// 7. BufPoolManager.size() == LRUKReplacer.size() == n \in Nodes: Nodes[n] = Evictable: Cardinality(Nodes[n])
// 8. INIT: LRUReplacer.size() == 0
// 9. IF {EvictableNodes} > 0 THEN LRUKReplacer.size() = Cardinality({EvictableNodes})
// 10. \A n \in Nodes: IF Nodes[n] == Pinnned OR Nodes[n] == Not_Used where Not_User = f(Nodes[n].HISTORY)
// THEN LRUKReplacer.len() = LRUKReplacer.len() - Nodes[n]_pinned/not_used

#[derive(Debug, Default, Clone, PartialEq)]
struct LRUNode {
    history: Vec<usize>,
    k_: usize,
    fid: FrameId,
    is_evictable: bool,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
struct LRUKReplacer {
    node_store: HashMap<FrameId, LRUNode>,
    current_timestamp: usize,
    current_size: usize,
    replacer_size: usize,
    k_: usize,
    latch: Arc<Mutex<LRUNode>>,
}

#[allow(dead_code)]
impl LRUKReplacer {
    fn new() -> Self {
        Self::default()
    }

    #[allow(non_snake_case)]
    fn Evict(&mut self) -> Option<FrameId> {
        // find Node[n].frame_id = \A n \in Nodes: Node[n].k_b_d = MAXIMUM({Node[n].k_b_d})
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as usize;

        let mut evictable = self
            .node_store
            .values()
            .filter(|node| node.is_evictable)
            .cloned()
            .collect::<Vec<LRUNode>>();

        // [A,B,C,D]
        for i in 0..evictable.len() - 1 {
            // IF Nodes[n].HISTORY.len() < 2 THEN Nodes[n].k_b_d = INFINITY
            //  if left.last().unwrap().history.len() < 2 {
            //             evictable[i].k_ = usize::MAX;
            //         }
            //         if right[i + 1].history.len() < 2 {
            //             evictable[i + 1].k_ = usize::MAX;
            //         }

            //         if evictable[i].k_ == evictable[i + 1].k_ {
            //             if evictable[i].history[0] < evictable[i + 1].history[0] {}
            //         } else {
            //             evictable[i].k_ = now - evictable[i].history[2];
            //             evictable[i + 1].k_ = now - evictable[i + 1].history[2];
            //         }

            // evictable: [A,B,C,D]
            let (left, right) = evictable.split_at_mut(i + 1); // (left: [A], [B,C,D])
            if left.last().unwrap().k_ == right.first().unwrap().k_ {
                if left.last().unwrap().history[0] < right.first().unwrap().history[0] {
                    if let Some(node) = self.node_store.remove(&left.last().unwrap().fid) {
                        return Some(node.fid);
                    } else {
                        let node = self.node_store.remove(&left.last().unwrap().fid);
                        return Some(node.unwrap().fid);
                    }
                }
                break;
            }
            if (now - left.last().unwrap().k_) > (now - right[0].k_) {
                // if A.k_ > B.k_ {
                std::mem::swap(&mut left.last().unwrap(), &mut right.first().unwrap());
                // [B, A, C, D]
            }
        }
        let victim_id = evictable.last().unwrap().fid;

        if let Some(evictee) = self.node_store.remove(&victim_id) {
            return Some(evictee.fid);
        } else {
            return None;
        }
    }

    #[allow(non_snake_case)]
    fn RecordAccess(&mut self, frame_id_t: FrameId) {
        if let Some((_, node)) = self
            .node_store
            .iter_mut()
            .find(|(_, node)| node.fid == frame_id_t)
        {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos();
            node.history.push(now as usize);
        }
    }

    #[allow(non_snake_case)]
    fn Remove(&mut self, frame_id_t: FrameId) {
        self.node_store
            .get_mut(&frame_id_t)
            .unwrap()
            .history
            .clear();
    }

    #[allow(non_snake_case)]
    fn SetEvictable(&mut self, frame_id_t: FrameId, set_evictable: bool) {
        if let Some(x) = self.node_store.get_mut(&frame_id_t) {
            x.is_evictable = set_evictable;
        }
    }

    #[allow(non_snake_case)]
    fn Size(&self) -> usize {
        self.node_store.len()
    }
}

#[allow(dead_code)]
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

    #[allow(non_snake_case)]
    pub fn NewPage(&mut self) -> FrameId {
        self.num_frames += 1;
        // FIXME: change struct for Value in page_table
        self.page_table.insert(self.num_frames, self.num_frames);

        self.num_frames
    }
    #[allow(non_snake_case)]
    pub fn getBufferPoolSize(&self) -> usize {
        self.frames.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cache() {
        LRUKReplacer::new();
    }
}
