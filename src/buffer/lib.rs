use crate::buffer::bufferpoolmanager::BufferPoolManager;

pub fn createBufferPoolManager() -> BufferPoolManager {
    let bufferPoolManager = BufferPoolManager::new(5);
    return bufferPoolManager;
}


