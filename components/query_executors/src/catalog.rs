use std::{collections::HashMap, sync::atomic::AtomicU32};

use crate::query_types::{Schema, TableHeap};
use crate::skiplistindex::SkipListIndex;
use buffer::bufferpoolmanager::BufferPoolManager;
use mvcc::transaction::Transaction;
use skiplist::SkipMap;

enum IndexType {
    BPlusTreeIndex,
    SkipListIndex,
}

type TableName = String;
type IndexName = String;
type TableId = u32;
type IndexId = u32;

struct TableInfo {
    table_name: String,
    schema: Schema, // perhaps put on the heap?
    table_heap: Box<TableHeap>,
    table_id: TableId,
}

struct IndexInfo {
    schema: Schema,
    index_name: String,
    index: Box<SkipMap>,
    index_id: IndexId,
    table_name: TableName,
    index_key_size: i32,
    is_primary_key: bool,
    index_type: IndexType,
}

struct LockManager {}

struct LogManager {}

struct Catalog {
    bpm: BufferPoolManager,
    logm: LogManager,
    lockm: LockManager,
    tables: HashMap<TableId, Box<TableInfo>>,
    table_names: HashMap<TableName, TableId>,
    table_next_id: AtomicU32<TableId>,
    indexes: HashMap<IndexId, Box<IndexInfo>>,
    index_names: HashMap<IndexName, IndexId>,
    index_next_id: AtomicU32<IndexId>,
}

impl Catalog {
    fn new() -> Self {
        Self {
            bpm: BufferPoolManager,
            logm: LogManager,
            lockm: LockManager,
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
        trxn: Transaction,
        table_name: TableName,
        schema: Schema,
        create_table: bool,
    ) {

    }
}
