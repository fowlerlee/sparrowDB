pub mod buffer;
pub mod storage;

use crate::buffer::lib::*;

#[cfg(test)]
fn test_buffering_memory() {
    let bpm = create_buffer_pool_manager();
    bpm.getBufferPoolSize();

    

}
