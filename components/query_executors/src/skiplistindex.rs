use skiplist::SkipMap;
use std::ops::Bound::Included;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SkipListIndex {
    index: Arc<Mutex<SkipMap<u64, (u64, usize)>>>, // id → (page_id, offset)
}

impl SkipListIndex {
    pub fn new() -> Self {
        Self {
            index: Arc::new(Mutex::new(SkipMap::new())),
        }
    }

    pub fn insert(&self, tuple_id: u64, page_id: u64, offset: usize) {
        self.index
            .lock()
            .unwrap()
            .insert(tuple_id, (page_id, offset));
    }

    pub fn find(&self, tuple_id: u64) -> Option<(u64, usize)> {
        let guard = self.index.lock().unwrap();
        match guard.get(&tuple_id) {
            Some(guard) => Some(*guard),
            None => None,
        }
    }

    pub fn range_query(&self, start: u64, end: u64) -> Vec<(u64, usize)> {
        self.index
            .lock()
            .unwrap()
            .range(Included(&start), Included(&end))
            .map(|(_, loc)| *loc)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::SkipListIndex;

    #[test]
    fn test_insert() {
        let tuple1 = 1u64;
        let page_1 = 5;
        let tuple2 = 2u64;
        let page_2 = 7;
        let tuple3 = 3u64;
        let page_3 = 10;
        let l = SkipListIndex::new();
        l.insert(tuple1, page_1, 10);
        l.insert(tuple2, page_2, 10);
        l.insert(tuple3, page_3, 10);
        let t1 = l.find(tuple1).unwrap();
        let t2 = l.find(tuple2).unwrap();
        let t3 = l.find(tuple3).unwrap();
        assert_eq!(page_1, t1.0);
        assert_eq!(page_2, t2.0);
        assert_eq!(page_3, t3.0);
        assert_eq!(l.range_query(0, 2), vec![(t1), (t2)])
    }
}
