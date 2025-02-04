#[cfg(test)]
mod test {
    
    #[allow(dead_code)]
    #[test]
    fn test_buffering_memory() {
        let bpm = buffer::bufferpoolmanager::BufferPoolManager::new(10, 2);
        assert_eq!(bpm.get_buffer_manager_size(), 10);
    }
}
