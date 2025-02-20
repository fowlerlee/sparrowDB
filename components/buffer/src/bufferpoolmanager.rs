use crate::frameheader::FrameHeader;
use crate::page_guard::{ReadPageGuard, WritePageGuard};
#[allow(unused)]
use crate::query_types::{get_demo_table_heap_with_n_page_m_tuples_each, TableHeap};
use common::types::PageId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use storage_engine::disk_manager::DiskManager;
use storage_engine::disk_scheduler::DiskScheduler;

type FrameId = usize;

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

impl LRUNode {
    #[allow(dead_code)]
    pub fn new(frame_id: FrameId) -> Self {
        Self {
            history: vec![],
            k_: 0,
            fid: frame_id,
            is_evictable: false,
        }
    }
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
    k_b_d_value: usize,
}

#[allow(dead_code)]
impl LRUKReplacer {
    #[allow(dead_code, unused)]
    fn new(k_b_d: usize) -> Self {
        Self {
            node_store: HashMap::new(),
            current_timestamp: 0,
            current_size: 0,
            replacer_size: 0,
            k_: 0,
            latch: Arc::new(Mutex::new(LRUNode {
                history: vec![0; 5],
                k_: 0,
                fid: 0,
                is_evictable: false,
            })),
            k_b_d_value: k_b_d,
        }
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
            // TODO: try the following: evitable.iter().fold(0, |k, &x| { k + x })

            // evictable: [A,B,C,D]
            let (left, right) = evictable.split_at_mut(i + 1); // (left: [A], [B,C,D])
            if left.last().unwrap().k_ == right.first().unwrap().k_
                && left.last().unwrap().history.len() >= self.k_b_d_value
                && right.first().unwrap().history.len() >= self.k_b_d_value
            {
                if left.last().unwrap().history[self.k_b_d_value]
                    < right.first().unwrap().history[self.k_b_d_value]
                {
                    if let Some(node) = self.node_store.remove(&left.last().unwrap().fid) {
                        return Some(node.fid);
                    } else {
                        let node = self.node_store.remove(&left.last().unwrap().fid);
                        return Some(node.unwrap().fid);
                    }
                }
                break;
            }
            if (now - left.last().unwrap().k_) > (now - right.first().unwrap().k_) {
                // if A.k_ > B.k_ {
                std::mem::swap(&mut left.last().unwrap(), &mut right.first().unwrap());
                // [B, A, C, D]
            }
        }
        let victim_id = evictable.last().unwrap().fid;
        if let Some(evictee) = self.node_store.remove(&victim_id) {
            Some(evictee.fid)
        } else {
            None
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
            node.k_ = now as usize;
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

// auto Size() const -> size_t;
//   auto NewPage() -> page_id_t;
//   auto DeletePage(page_id_t page_id) -> bool;
//   auto CheckedWritePage(page_id_t page_id, AccessType access_type = AccessType::Unknown)
//       -> std::optional<WritePageGuard>;
//   auto CheckedReadPage(page_id_t page_id, AccessType access_type = AccessType::Unknown) -> std::optional<ReadPageGuard>;
//   auto WritePage(page_id_t page_id, AccessType access_type = AccessType::Unknown) -> WritePageGuard;
//   auto ReadPage(page_id_t page_id, AccessType access_type = AccessType::Unknown) -> ReadPageGuard;
//   auto FlushPageUnsafe(page_id_t page_id) -> bool;
//   auto FlushPage(page_id_t page_id) -> bool;
//   void FlushAllPagesUnsafe();
//   void FlushAllPages();
//   auto GetPinCount(page_id_t page_id) -> std::optional<size_t>;

#[allow(dead_code)]
pub struct BufferPoolManager {
    num_frames: usize,
    next_page: AtomicUsize,
    atomic_counter: AtomicUsize,
    frames: Vec<FrameHeader>,
    page_table: Arc<Mutex<HashMap<PageId, FrameId>>>,
    free_frames: Vec<[u8; 4096]>,
    replacer: Box<LRUKReplacer>,
    disk_scheduler: DiskScheduler,
    pub table_heap: Arc<Mutex<TableHeap>>,
}

//                     +-----------------------------+
//                     |   Real-Time AI Query Engine |
//                     +-----------------------------+
//                                 |
//     +------------------+-----------------------+
//     |   AI Cache (LRU) |  Vectorized Execution |
//     +------------------+-----------------------+
//                        |                       |
// +---------------------------+        +-------------------------------------+
// | HTAP Storage (KestrelDB)  |        |  Real-Time Streaming Engine (OSS)   |
// |  - Buffer Pool Manager    |        |  - Kafka / Redpanda                 |
// |  - LRU Cache Optimization |        |  - AI-enhanced queries              |
// +---------------------------+        +-------------------------------------+

#[allow(unused)]
impl BufferPoolManager {
    pub fn new(capacity: usize, k_b_d: usize) -> Self {
        let new_file = file_system::file::File::create("disk_file.dat").unwrap();
        // allocate all in-memory frames upfront
        let mut frames: Vec<FrameHeader> = Vec::new();
        let mut free_frames = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            frames.push(FrameHeader::default());
            free_frames.push([0; 4096]);
        }

        Self {
            num_frames: 0,
            next_page: AtomicUsize::new(0),
            atomic_counter: AtomicUsize::new(0),
            frames,
            page_table: Arc::new(Mutex::new(HashMap::with_capacity(capacity))),
            free_frames,
            replacer: Box::new(LRUKReplacer::new(k_b_d)),
            disk_scheduler: DiskScheduler::new(DiskManager::new(new_file)),
            table_heap: Arc::new(Mutex::new(TableHeap::new(capacity))),
        }
    }

    pub fn new_page(&mut self) -> Option<FrameId> {
        // acquire mutex
        let mut guard = self.page_table.lock().unwrap();
        self.num_frames += 1;
        self.atomic_counter.fetch_add(1, Ordering::SeqCst);
        self.frames.pop();
        let frame = FrameHeader::default();
        self.frames.push(frame);
        self.free_frames.pop();
        // self.table_heap.add_table_page(frame); //TODO: should this keep track of the pages in memory?
        guard.insert(self.num_frames, self.num_frames)
    }

    pub fn get_buffer_manager_size(&self) -> usize {
        self.frames.len()
    }

    pub fn delete_page(&mut self, page_id: PageId) -> bool {
        let mut guard = self.page_table.lock().unwrap();
        self.num_frames -= 1;
        self.atomic_counter.fetch_sub(1, Ordering::SeqCst);
        self.frames.remove(page_id);
        self.frames.push(FrameHeader::default());
        self.free_frames.push([0; 4096]);
        guard.remove(&page_id);
        true
    }

    fn check_write_page(&self, frame_id: FrameId, data: Vec<u8>) -> Result<bool, String> {
        let writer = Arc::new(RwLock::new(WritePageGuard::new()));
        let result = match writer.write() {
            Ok(mut guard) => {
                if let Some(val) = guard.write_page_data(frame_id, data) {
                    // TODO: persist the frame_id from the write in meta data
                    let frame_id = val;
                    Ok(true)
                } else {
                    // TODO: when this fails to write to mem then make new_page and retry write
                    Ok(false)
                }
            }
            Err(_) => Err("failed to get the guard and write data".to_string()),
        };
        result
    }

    fn check_read_page(&self, frame_id: FrameId) -> Result<bool, String> {
        let reader = Arc::new(RwLock::new(ReadPageGuard::new()));
        let result = match reader.read() {
            Ok(mut guard) => {
                guard.read_page_data(frame_id);
                Ok(true)
            }
            Err(_) => Err("failed to get the guard".to_string()),
        };
        result
    }

    fn check_page_exists_in_buffer(&self, data: Vec<u8>) -> bool {
        true
        // let frame = self.frames.iter().filter(|&x| x.data == data).collect::<[FrameHeader]>();
    }

    pub fn set_table_heap(&mut self, table_heap: TableHeap) {
        self.table_heap = Arc::new(Mutex::new(table_heap));
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_cache_eviction() {
        let mut lru = LRUKReplacer::new(2);
        lru.node_store.insert(0, LRUNode::default());
        lru.node_store.insert(1, LRUNode::new(1));
        lru.node_store.insert(2, LRUNode::new(2));
        lru.node_store.insert(3, LRUNode::new(3));
        lru.RecordAccess(0);
        lru.RecordAccess(0);
        lru.RecordAccess(0);
        lru.RecordAccess(1);
        lru.RecordAccess(1);
        lru.RecordAccess(1);

        lru.SetEvictable(1, true);
        lru.SetEvictable(0, true);
        let victim = lru.Evict().unwrap();

        assert!(Some(victim).is_some());
    }

    #[test]
    fn test_bpm_new_page() {
        let mut bpm = BufferPoolManager::new(10, 2);
        let _frame_id_0 = bpm.new_page();
        let _frame_id_1 = bpm.new_page();
        assert_eq!(bpm.get_buffer_manager_size(), 10);

        let successful_delete = bpm.delete_page(0);
        assert_eq!(bpm.get_buffer_manager_size(), 10);
        assert!(successful_delete);
        assert_eq!(bpm.num_frames, 1);
    }

    #[test]
    fn test_page_guard_write_blocking() {
        let bpm = Arc::new(Mutex::new(BufferPoolManager::new(10, 2)));
        for _ in 0..10 {
            let fake = Arc::clone(&bpm);
            std::thread::spawn(move || {
                let wrote_page = fake
                    .lock()
                    .unwrap()
                    .check_write_page(1usize, vec![5; 12])
                    .unwrap();
                assert!(wrote_page);
            });
        }
        // TODO: Test whether after 100 is the pages editable again with write
        // and simultaneous reads
        // let mut guard = bpm.lock().unwrap();
        // guard.frames.push(FrameHeader::default());
        // let _default_frame = guard.frames.get(0).unwrap();
    }

    #[test]
    fn test_page_guard_read_blocking() {
        let bpm = Arc::new(Mutex::new(BufferPoolManager::new(10, 2)));
        for _ in 0..100 {
            let fake = Arc::clone(&bpm);
            std::thread::spawn(move || {
                let read_page = fake.lock().unwrap().check_read_page(1usize).unwrap();
                assert!(read_page);
            });
        }
    }

    #[test]
    fn test_page_no_data_loss() {
        let bpm = Arc::new(std::sync::RwLock::new(BufferPoolManager::new(10, 2)));
        for _ in 0..100 {
            let fake = Arc::clone(&bpm);
            std::thread::spawn(move || {
                let wrote_page = fake
                    .write()
                    .unwrap()
                    .check_write_page(1usize, vec![5; 4096])
                    .unwrap();
                assert!(wrote_page);
            });
        }

        for _ in 0..100 {
            let fake = Arc::clone(&bpm);
            std::thread::spawn(move || {
                let read_page = fake.read().unwrap().check_read_page(1usize).unwrap();
                assert!(read_page);
            });
        }
    }

    #[test]
    fn test_check_page_exists_in_buffer() {
        // TODO: complete FrameHeader impl and this test
    }

    #[test]
    fn test_bpm_page_heap_and_pages_in_writes_and_reads() {
        let mut bpm = BufferPoolManager::new(10, 2);
        let table_heap = get_demo_table_heap_with_n_page_m_tuples_each(5, 20);
        bpm.set_table_heap(table_heap);
        // have data in the table heap.
    }

    #[test]
    fn test_bpm_create_table_heap() {
        let mut bpm = BufferPoolManager::new(10, 2);

        // bpm.table_heap.add_table_page(page);
        bpm.new_page();
    }
}
