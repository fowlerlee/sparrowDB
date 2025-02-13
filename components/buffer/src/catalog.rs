use std::cell::RefCell;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, sync::atomic::AtomicU32};

use crate::bufferpoolmanager::BufferPoolManager;
use crate::query_types::{Schema, TableHeap};
use crate::skiplistindex::SkipListIndex;
use common::transaction::Transaction;
// use skiplist::SkipMap;
#[allow(dead_code)]
enum IndexType {
    BPlusTreeIndex,
    SkipListIndex,
}
#[allow(dead_code)]
type TableName = String;
#[allow(dead_code)]
type IndexName = String;
#[allow(dead_code)]
type TableId = u32;
#[allow(dead_code)]
type IndexId = u32;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TableInfo {
    table_name: String,
    schema: Schema, // perhaps put on the heap?
    table_heap: Arc<Mutex<TableHeap>>,
    table_id: TableId,
}

impl Clone for TableInfo {
    fn clone(&self) -> Self {
        Self {
            table_name: self.table_name.clone(),
            schema: self.schema.clone(),
            table_heap: self.table_heap.clone(),
            table_id: self.table_id.clone(),
        }
    }
}
#[allow(dead_code)]
impl TableInfo {
    fn new(table_name: String, schema: Schema, table_heap: TableHeap, table_id: TableId) -> Self {
        Self {
            table_name,
            schema,
            table_heap: Arc::new(Mutex::new(table_heap)),
            table_id,
        }
    }
}

#[allow(dead_code)]
struct IndexInfo {
    schema: Schema,
    index_name: String,
    index: Box<SkipListIndex>,
    index_id: IndexId,
    table_name: TableName,
    index_key_size: i32,
    is_primary_key: bool,
    index_type: IndexType,
}

#[derive(Default)]
#[allow(dead_code)]
struct LockManager {}

#[derive(Default)]
#[allow(dead_code)]
struct LogManager {}

#[allow(dead_code)]
pub struct Catalog {
    pub bpm: BufferPoolManager,
    logm: LogManager,
    lockm: LockManager,
    tables: HashMap<TableId, RefCell<TableInfo>>,
    table_names: HashMap<TableName, TableId>,
    table_next_id: AtomicU32,
    indexes: HashMap<IndexId, Box<IndexInfo>>,
    index_names: HashMap<TableName, HashMap<IndexName, IndexId>>,
    index_next_id: AtomicU32,
}

#[allow(dead_code)]
impl Catalog {
    pub fn new() -> Self {
        Self {
            bpm: BufferPoolManager::new(0, 0),
            logm: LogManager::default(),
            lockm: LockManager::default(),
            tables: HashMap::new(),
            table_names: HashMap::new(),
            table_next_id: AtomicU32::new(0),
            indexes: HashMap::new(),
            index_names: HashMap::new(),
            index_next_id: AtomicU32::new(0),
        }
    }

    pub fn create_table(
        &mut self,
        _trxn: Transaction,
        table_name: TableName,
        schema: Schema,
        create_table: bool,
    ) -> TableInfo {
        let mut table_heap: Option<TableHeap> = None;
        if create_table {
            table_heap = Some(TableHeap::new(1));
        }
        let table_name = RefCell::new(table_name);
        let table_id = self.table_next_id.fetch_add(1, Ordering::SeqCst);
        let table_info = RefCell::new(TableInfo::new(
            table_name.borrow().parse().unwrap(),
            schema,
            table_heap.unwrap(),
            table_id,
        ));
        self.tables.insert(table_id, table_info.clone());
        self.table_names
            .insert(table_name.borrow().parse().unwrap(), table_id);
        self.index_names
            .entry(table_name.borrow().parse().unwrap())
            .or_insert_with(|| HashMap::new());
        self.bpm.table_heap = table_info.borrow().clone().table_heap;
        // .insert(IndexName, IndexId);
        table_info.into_inner()
    }

    pub fn get_table(&self, _table_name: Option<String>) -> Vec<String> {
        let keys: Vec<String> = self.table_names.keys().cloned().collect();
        keys
    }
}

// // Update the internal tracking mechanisms
// tables_.emplace(table_oid, meta);
// table_names_.emplace(table_name, table_oid);
// index_names_.emplace(table_name, std::unordered_map<std::string, index_oid_t>{});
