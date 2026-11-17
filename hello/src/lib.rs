pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        ThreadPool
    }

    pub fn execute<F: FnOnce() -> ()>(&self, job: F) {
        job()
    }
}
