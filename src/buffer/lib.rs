use crate::buffer::bufferpoolmanager::BufferPoolManager;

pub fn create_buffer_pool_manager() -> BufferPoolManager {
    return BufferPoolManager::new(5);
}
