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
        let tuple2 = 2u64;
        let page_id = 7;
        let l = SkipListIndex::new();
        l.insert(1, 5, 10);
        l.insert(tuple2, page_id, 10);
        l.insert(0, 5, 10);

        let t2 = l.find(tuple2).unwrap();
        assert_eq!(page_id, t2.0);
    }
}
