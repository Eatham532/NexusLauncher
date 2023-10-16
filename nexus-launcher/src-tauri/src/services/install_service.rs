use std::collections::VecDeque;
use std::sync::{Arc};
use std::task;
use crossbeam_queue::SegQueue;
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Option;
use tauri::Window;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use crate::config::structs::instances::NexusInstance;


#[derive(Clone)]
pub struct InstallationService {
    queue: Arc<SegQueue<NexusInstance>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Payload {
    id: String,
    progress: String
}

impl InstallationService {
    pub fn new() -> Self {
        let service = InstallationService {
            queue: Arc::new(SegQueue::new()),
        };
        service
    }

    pub async fn add_instance(&self, instance: NexusInstance, window: Window) {
        self.queue.push(instance);

        println!("Added instance to install queue");
        // Run the task
        self.process_queue(window).await;
    }

    async fn process_queue(&self, window: Window) {
        println!("starting process_queue");
        let next_job = self.queue.pop();
        match next_job {
            Some(mut job) => {
                // Perform the installation process for the instance
                // ...
                // Once the installation is completed, continue processing the queue
                println!("Starting Installation of {:?}...", job.name);
                job.install(window).await;
                println!("Installation Complete!");
            },
            _ => {}
        }
    }
}