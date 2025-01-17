#[cfg(test)]
mod test {
    #[allow(dead_code)]
    fn test_buffering_memory() {
        let bpm = buffer::bufferpoolmanager::BufferPoolManager::new(10);
        assert_eq!(bpm.getBufferPoolSize(), 10);
    }
}
