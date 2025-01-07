use crate::buffer::bufferpoolmanager::BufferPoolManager;

pub fn createBufferPoolManager() -> BufferPoolManager {
    let bufferPoolManager = BufferPoolManager::default();
    return bufferPoolManager;
}


