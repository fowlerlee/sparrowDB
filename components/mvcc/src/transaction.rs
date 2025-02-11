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

pub struct Transaction {}
