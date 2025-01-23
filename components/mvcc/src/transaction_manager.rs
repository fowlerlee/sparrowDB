
use std::{collections::HashMap, sync::atomic::AtomicUsize};
use storage_engine::page::PageId;
use std::cell::RefCell;

use crate::types::{PageVersionInfo, Watermark};




#[allow(dead_code)]
struct TransactionManager {
    version_info: HashMap<PageId, RefCell<PageVersionInfo>>,
    running_txns: Watermark,
    last_commit_ts: AtomicUsize,
}   

