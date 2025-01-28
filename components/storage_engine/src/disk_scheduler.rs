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
use crate::page::PageId;
use std::fmt::{Debug, Formatter, Result};
use std::sync::{Arc, Condvar, Mutex};
use std::{collections::VecDeque, thread};

#[allow(dead_code)]
struct DiskRequest {
    is_write: bool,
    // data: &'a char, should be a pointer to data but we are getting invariant Type violations
    data: char,
    page_id: PageId,
    callback: Box<dyn Fn(bool) -> ()>,
}

fn my_callback(success: bool) {
    println!(
        "Named function callback: {}",
        if success { "Yes" } else { "No" }
    );
}

// we want to pass the DiskRequest between threads so i need to be bad here
unsafe impl Send for DiskRequest {}
unsafe impl Sync for DiskRequest {}

impl Default for DiskRequest {
    fn default() -> Self {
        Self {
            is_write: false,
            data: 'a',
            page_id: 0,
            callback: Box::new(my_callback),
        }
    }
}

impl Debug for DiskRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("DiskRequest")
            .field("is_write", &self.is_write)
            .field("data", &self.data)
            .field("page_id", &self.page_id)
            .finish()
    }
}

#[allow(dead_code)]
pub struct DiskScheduler<'a> {
    disk_manager: &'a DiskManager,
    channel: Arc<(Mutex<VecDeque<DiskRequest>>, Condvar)>, // Channel to coordinate threads
}

impl<'a> DiskScheduler<'a> {
    #[allow(dead_code)]
    pub fn new(disk_manager: &'a DiskManager) -> Self {
        Self {
            disk_manager,
            channel: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())), // Initialize channel
        }
    }

    // Start a worker thread
    #[allow(dead_code)]
    pub fn start_worker_thread(&self) {
        let channel_clone = Arc::clone(&self.channel);

        thread::spawn(move || {
            let mut guard = channel_clone.0.lock().unwrap();
            if !guard.is_empty() {
                guard.pop_back(); // remove item from back
            }
            // channel_clone.1.wait(guard)
        });
    }
    // TODO: add a condvar relationship here for taking from and putting on the vecdeq
    #[allow(dead_code)]
    pub fn schedule(&mut self, request: DiskRequest) {
        let channel_clone = Arc::clone(&self.channel);
        // let mutcond: Arc<(Mutex<VecDeque<DiskRequest>>, Condvar)> =
        //     Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));

        // let &(ref mtx, ref cnd) = &*self.channel;
        thread::spawn(move || {
            let mut guard = channel_clone.0.lock().unwrap();
            let disk_request = request;
            guard.push_back(disk_request); // add the DiskRequest to queue
            println!(
                "Worker thread executed and set the value to {:?}",
                guard.get(0)
            );
            // channel_clone.1.wait(guard)
        });
    }
}
