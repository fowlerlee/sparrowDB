#[allow(unused_imports)]
use crate::bufferpoolmanager::BufferPoolManager;
mod test {
    #[allow(unused_imports)]
    use crate::bufferpoolmanager::BufferPoolManager;

    #[test]
    fn test_buffer_pool_manager_size() {
        let buffer_pool_manager = BufferPoolManager::new(10, 2);
        assert_eq!(buffer_pool_manager.getBufferPoolSize(), 10);
    }
}
