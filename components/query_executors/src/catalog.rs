use std::collections::HashMap;

use buffer::bufferpoolmanager::BufferPoolManager;

type TableId = u64;
type IndexId = u64;

struct TableInfo {}
struct IndexInfo {}

struct LockManager {}

struct LogManager {}

struct Catalog {
    bpm: BufferPoolManager,
    logm: LogManager,
    lockm: LockManager,
    tables: HashMap<TableId, Box<TableInfo>>,
    indexes: HashMap<IndexId, Box<IndexInfo>>,
}

impl Catalog {
    pub fn create_table(&mut self) {}
}
