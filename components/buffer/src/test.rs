#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_buffer_pool_manager_size() {
        let buffer_pool_manager = crate::bufferpoolmanager::BufferPoolManager::new(10, 2);
        assert_eq!(buffer_pool_manager.getBufferPoolSize(), 10);
    }

    #[test]
    fn test_page_io() -> std::io::Result<()> {
        let storage = Arc::new(PageStorage::new("database_pages.bin")?);
        let mut frame_manager = FrameManager::new(storage.clone(), 4);

        // Load a page into frame 0
        frame_manager.load_page(0, 0)?;

        // Modify a portion of the frame
        frame_manager.modify_frame(0, 100, b"Hello, Database!");

        // Write back dirty frames to disk
        frame_manager.flush()?;

        Ok(())
    }
}

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

const PAGE_SIZE: usize = 4096; // 4KB per page

/// Represents a database page in memory (frame)
#[derive(Clone)]
struct PageFrame {
    data: [u8; PAGE_SIZE], // Fixed-size page
    dirty: bool,           // Flag to indicate if the page needs to be written back
}

/// Disk-backed paged storage system
struct PageStorage {
    file: Mutex<File>, // File storing multiple pages
}

impl PageStorage {
    /// Open or create a paged file storage
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(Self {
            file: Mutex::new(file),
        })
    }

    /// Read a page from disk into a frame
    pub fn read_page(&self, page_id: usize) -> std::io::Result<PageFrame> {
        let mut file = self.file.lock().unwrap();
        let mut buffer = [0u8; PAGE_SIZE];

        file.seek(SeekFrom::Start((page_id * PAGE_SIZE) as u64))?;
        file.read_exact(&mut buffer)?;

        Ok(PageFrame {
            data: buffer,
            dirty: false,
        })
    }

    /// Write a frame back to disk if it is dirty
    pub fn write_page(&self, page_id: usize, frame: &PageFrame) -> std::io::Result<()> {
        if frame.dirty {
            let mut file = self.file.lock().unwrap();
            file.seek(SeekFrom::Start((page_id * PAGE_SIZE) as u64))?;
            file.write_all(&frame.data)?;
            file.flush()?;
        }
        Ok(())
    }
}

/// Memory manager to handle frames
struct FrameManager {
    frames: Vec<Option<PageFrame>>, // Frame buffer in memory
    storage: Arc<PageStorage>,
}

impl FrameManager {
    /// Create a new frame manager with a given number of frame slots
    pub fn new(storage: Arc<PageStorage>, frame_count: usize) -> Self {
        Self {
            frames: vec![None; frame_count], // Initialize empty frames
            storage,
        }
    }

    /// Load a page from disk into a frame slot
    pub fn load_page(&mut self, frame_id: usize, page_id: usize) -> std::io::Result<()> {
        let frame = self.storage.read_page(page_id)?;
        self.frames[frame_id] = Some(frame);
        Ok(())
    }

    /// Modify a frame in memory
    pub fn modify_frame(&mut self, frame_id: usize, offset: usize, data: &[u8]) {
        if let Some(frame) = &mut self.frames[frame_id] {
            if offset + data.len() <= PAGE_SIZE {
                frame.data[offset..offset + data.len()].copy_from_slice(data);
                frame.dirty = true; // Mark the frame as modified
            }
        }
    }

    /// Flush all dirty frames to disk
    pub fn flush(&mut self) -> std::io::Result<()> {
        for (i, frame) in self.frames.iter().enumerate() {
            if let Some(f) = frame {
                self.storage.write_page(i, f)?;
            }
        }
        Ok(())
    }
}
