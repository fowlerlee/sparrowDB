//  private:
//   /** Pointer to the disk manager. */
//   DiskManager *disk_manager_ __attribute__((__unused__));
//   /** A shared queue to concurrently schedule and process requests. When the DiskScheduler's destructor is called,
//    * `std::nullopt` is put into the queue to signal to the background thread to stop execution. */
//   Channel<std::optional<DiskRequest>> request_queue_;
//   /** The background thread responsible for issuing scheduled requests to the disk manager. */
//   std::optional<std::thread> background_thread_;

// Schedule(DiskRequest r) : Schedules a request for the DiskManager to execute. The DiskRequest struct specifies whether the request is for
// a read or write, where the data should be read from / written into, and the page ID for the operation. The DiskRequest also includes a std::promise whose
// value should be set to true once the request is processed. See below for more information about std::promise.
//
// StartWorkerThread() : The startup method for the background worker thread which processes the scheduled requests. The worker thread is created in the
// DiskScheduler constructor and calls this method. This worker thread is responsible for receiving queued requests and dispatching them to the DiskManager.
// Remember to set the value correctly on the DiskRequest's callback to signal to the request issuer that the request has been completed.
// This should not return until the DiskScheduler's destructor is called.

use crate::disk_manager::DiskManager;
use crate::page::Page;
use std::sync::{Arc, Condvar, Mutex};
use std::{collections::VecDeque, thread};

struct DiskRequest {}

struct DiskScheduler<'a> {
    disk_manager: &'a DiskManager<'a>,
    channel: Arc<(Mutex<VecDeque<Page>>, Condvar)>, // Channel to coordinate threads
}

impl<'a> DiskScheduler<'a> {
    pub fn new(disk_manager: &'a DiskManager) -> Self {
        Self {
            disk_manager,
            channel: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())), // Initialize channel
        }
    }

    // Start a worker thread
    pub fn start_worker_thread(&self) {
        let channel_clone = Arc::clone(&self.channel);

        thread::spawn(move || {
            let mut guard = channel_clone.0.lock().unwrap();
            let binding = Page::new(1usize, vec![]);
            guard.push_back(binding); // add the page
            println!(
                "Worker thread executed and set the value to {:?}",
                guard.get(0)
            );
            // channel_clone.1.wait(guard)
        });
    }
}
