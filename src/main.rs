use thread_pool::job::WorkerJob;
use thread_pool::thread_pool::ThreadPool;

fn main() {
    let mut pool = ThreadPool::default();

    for id in 0..10_000 {
        pool.schedule_job(Box::new(DivideJob::new(4, 7, 1_000_000, id)))
    }

    pool.start();

    pool.wait();
}

pub struct DivideJob {
    a: u8,
    b: u8,
    count: u32,
    id: usize
}

impl DivideJob {
    pub fn new(a: u8, b: u8, count: u32, id: usize) -> Self {
        Self { a, b, count, id }
    }
}

impl WorkerJob for DivideJob {
    fn work(&mut self) {
        let mut _result = self.a as f32;
        for _ in 0..self.count {
            _result /= self.b as f32;
        }
    }

    fn finished(&mut self) {
        println!("Job {} finished!", self.id);
    }
}