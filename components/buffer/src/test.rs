
mod test {
    #[test]
    fn test_buffer_pool_manager_size() {
        let buffer_pool_manager = crate::bufferpoolmanager::BufferPoolManager::new(10, 2);
        assert_eq!(buffer_pool_manager.getBufferPoolSize(), 10);
    }
}
