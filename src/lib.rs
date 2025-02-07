#[cfg(test)]
mod test {

    #[allow(dead_code)]
    #[test]
    fn test_buffering_memory() {
        let bpm = buffer::bufferpoolmanager::BufferPoolManager::new(10, 2);
        assert_eq!(bpm.get_buffer_manager_size(), 10);
        let _table_heap = common::query_types::get_demo_schema();
        // FIXME: must add bufferpoolmanager to the Table Heap
        // table_heap.set_buffer_pool_manager(bpm);
    }
}
