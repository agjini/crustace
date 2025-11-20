use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Counter {
    count: Arc<Mutex<i32>>,
}

impl Counter {
    pub fn new(initial: i32) -> Self {
        Counter {
            count: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn inc(&mut self) {
        let mut num = self.count.lock().unwrap();
        *num += 1;
    }

    pub fn value(&self) -> i32 {
        let num = self.count.lock().unwrap();
        *num
    }
}
