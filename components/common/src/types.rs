
#[derive(Clone, Debug,PartialEq)]
pub enum TxnResult <T, E>{  
    Ok(T),
    Err(E),
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FrameHeader {
    is_dirty: bool,

}

// FrameHeader::FrameHeader(frame_id_t frame_id) : frame_id_(frame_id), data_(BUSTUB_PAGE_SIZE, 0) { Reset(); }

// /**
//  * @brief Get a raw const pointer to the frame's data.
//  *
//  * @return const char* A pointer to immutable data that the frame stores.
//  */
// auto FrameHeader::GetData() const -> const char * { return data_.data(); }

// /**
//  * @brief Get a raw mutable pointer to the frame's data.
//  *
//  * @return char* A pointer to mutable data that the frame stores.
//  */
// auto FrameHeader::GetDataMut() -> char * { return data_.data(); }

// /**
//  * @brief Resets a `FrameHeader`'s member fields.
//  */
// void FrameHeader::Reset() {
//   std::fill(data_.begin(), data_.end(), 0);
//   pin_count_.store(0);
//   is_dirty_ = false;
// }