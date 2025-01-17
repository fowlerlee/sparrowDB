mod bufferpoolmanager;

use bufferpoolmanager::BufferPoolManager;

pub fn create_buffer_pool_manager() -> BufferPoolManager {
    return BufferPoolManager::new(5);
}

mod tests {


    #[test]
    fn test_buffer_pool_manager_size() {
        let buffer_pool_manager = BufferPoolManager::new(10);
        assert_eq!(buffer_pool_manager.getBufferPoolSize(), 10);
    }
}
