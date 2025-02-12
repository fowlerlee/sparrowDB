use std::sync::atomic::{AtomicI64, AtomicU64};

#[allow(dead_code)]
pub enum TransactionState {
    RUNNING,
    TAINTED,
    COMMITTED,
    ABORTED,
}
#[allow(dead_code)]
pub enum IsolationLevel {
    READUNCOMMITTED,
    SNAPSHOTISOLATION,
    SERIALIZABLE,
}
#[allow(dead_code)]
pub struct Transaction {
    id: AtomicU64,
    finished: bool,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: AtomicU64::new(0),
            finished: false,
        }
    }
}
