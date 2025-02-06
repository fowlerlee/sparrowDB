use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, PartialEq)]
pub enum TxnResult<T, E> {
    Ok(T),
    Err(E),
}

pub type PageId = usize;

#[derive(Debug)]
#[allow(dead_code)]
pub struct FrameHeader {
    is_dirty: bool,
    pub data: Vec<u8>,
    pub frame_id: usize,
    pin_count: AtomicU64,
}

impl PartialEq for FrameHeader {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.frame_id == other.frame_id
    }
}

impl Eq for FrameHeader {}

impl Default for FrameHeader {
    fn default() -> Self {
        Self {
            is_dirty: false,
            data: vec![0],
            frame_id: 0,
            pin_count: AtomicU64::new(0),
        }
    }
}

// impl FromIterator<u8> for FrameHeader {
//     fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
//         let data: Vec<u8> = iter.into_iter().collect();
//         FrameHeader {
//             is_dirty: false,
//             data,
//             frame_id: 0,
//             pin_count: AtomicU64::new(0),
//         }
//     }
// }

// impl<'a> IntoIterator for &'a mut FrameHeader {
//     type Item = &'a mut u8;
//     type IntoIter = std::slice::IterMut<'a, u8>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.data.iter_mut()
//     }
// }

#[allow(dead_code)]
impl FrameHeader {
    fn new(frame_id: usize) -> Self {
        Self {
            is_dirty: false,
            data: vec![0],
            frame_id,
            pin_count: AtomicU64::new(0),
        }
    }

    pub fn write_data(&mut self, data: Vec<u8>) {
        self.data = data;
        self.is_dirty = true;
        self.pin_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn read_data(&mut self, data: Vec<u8>) {
        self.data = data;
        self.is_dirty = true;
        self.pin_count.fetch_add(1, Ordering::SeqCst);
    }
}
