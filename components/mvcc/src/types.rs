use std::collections::HashMap;

// TODO: move these types to a config package
#[allow(dead_code)]
type TimeStamp = u64;
type SlottedOffset = u64;
type TxnId = i64;

#[allow(dead_code)]
pub struct PageVersionInfo {
    prev_link: HashMap<SlottedOffset, UndoLink>,
}
#[allow(dead_code)]
pub struct Watermark {}
#[allow(dead_code)]
struct UndoLink {
    prev_txn: TxnId,
}
#[allow(dead_code)]
impl UndoLink {
    pub fn is_valid(&self) -> bool {
        self.prev_txn != -1 // TODO: make a INVALID_TXN_ID var for this -1 value
    }
}
