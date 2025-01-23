#[allow(dead_code)]
pub enum TranasActionState {
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
