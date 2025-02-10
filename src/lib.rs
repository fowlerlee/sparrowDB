#[cfg(test)]
mod test {

    #[allow(dead_code)]
    #[test]
    fn test_buffering_memory() {
        let pages = 10;
        let mut bpm = buffer::bufferpoolmanager::BufferPoolManager::new(pages, 2);
        assert_eq!(bpm.get_buffer_manager_size(), 10);
        let table_heap =
            common::query_types::get_demo_table_heap_with_n_page_m_tuples_each(pages, 20);
        bpm.set_table_heap(table_heap);
        bpm.table_heap.create_index();
    }
}
