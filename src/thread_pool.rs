use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::job::{JobProvider, JobScheduler, WorkerJob};

pub struct ThreadPool {
    worker_count: usize,
    thread_handles: Vec<JoinHandle<()>>,
    job_scheduler: Arc<Mutex<JobScheduler>>,
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self {
            worker_count: num_cpus::get(),
            thread_handles: vec![],
            job_scheduler: Arc::new(Mutex::new(JobScheduler::new(ProcessStrategy::FIFO))),
        }
    }
}

impl ThreadPool {
    pub fn new(worker_count: usize, process_strategy: ProcessStrategy) -> Self {
        Self { worker_count, thread_handles: vec![], job_scheduler: Arc::new(Mutex::new(JobScheduler::new(process_strategy))) }
    }
}

impl ThreadPool {
    pub fn worker_count(&self) -> &usize {
        &self.worker_count
    }

    pub fn process_strategy(&self) -> ProcessStrategy {
        self.job_scheduler.lock().unwrap().process_strategy().clone()
    }
}

impl ThreadPool {
    pub fn schedule_job(&mut self, job: Box<dyn WorkerJob + Send + Sync>) {
        self.job_scheduler.lock().unwrap().schedule_job(job);
    }

    pub fn start(&mut self) {
        for _ in 0..self.worker_count {
            let local_scheduler = self.job_scheduler.clone();
            let handle = std::thread::spawn(move || {
                let mut job = local_scheduler.lock().unwrap().next_job();
                while job.is_some() {
                    let mut job_unwrapped = job.unwrap();
                    job_unwrapped.work();
                    job_unwrapped.finished();

                    job = local_scheduler.lock().unwrap().next_job();
                }
            });
            self.thread_handles.push(handle);
        }
    }

    pub fn wait(&mut self) {
        while !self.thread_handles.is_empty() {
            self.thread_handles.pop().unwrap().join().unwrap();
        }
    }
}

#[derive(Clone)]
pub enum ProcessStrategy {
    FIFO,
    LIFO,
    Random,
}

