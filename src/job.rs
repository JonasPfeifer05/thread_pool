use rand::Rng;
use crate::thread_pool::{ProcessStrategy};

pub trait WorkerJob {
    fn work(&mut self);
    fn finished(&mut self);
}

pub trait JobProvider {
    fn next_job(&mut self) -> Option<Box<dyn WorkerJob + Send + Sync>>;
}

pub struct JobScheduler{
    process_strategy: ProcessStrategy,
    job_list: Vec<Box<dyn WorkerJob + Send + Sync>>,
}

impl JobScheduler {
    pub fn new(process_strategy: ProcessStrategy) -> Self {
        Self { process_strategy, job_list: vec![] }
    }

    pub fn jobs_left(&self) -> usize {
        self.job_list.len()
    }

    pub fn process_strategy(&self) -> &ProcessStrategy {
        &self.process_strategy
    }
}

impl JobProvider for JobScheduler {
    fn next_job(&mut self) -> Option<Box<dyn WorkerJob + Send + Sync>> {
        return match self.job_list.is_empty() {
            true => None,
            false => {
                match self.process_strategy {
                    ProcessStrategy::FIFO => {
                        Some(self.job_list.remove(0))
                    }
                    ProcessStrategy::LIFO => {
                        self.job_list.pop()
                    }
                    ProcessStrategy::Random => {
                        let random_index = rand::thread_rng().gen_range(0usize..self.job_list.len());
                        Some(self.job_list.remove(random_index))
                    }
                }
            }
        };
    }
}

impl JobScheduler {
    pub fn schedule_job(&mut self, job: Box<dyn WorkerJob + Send + Sync>) {
        self.job_list.push(job);
    }
}