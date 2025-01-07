mod tests {

    #[cfg(test)]
    fn test_buffer_pool_manager_size() {
        use crate::buffer::{self, bufferpoolmanager::BufferPoolManager};
        let buffer_pool_manager = BufferPoolManager::new(10);
        assert_eq!(buffer_pool_manager.getBufferPoolSize(), 10);
    }
}
