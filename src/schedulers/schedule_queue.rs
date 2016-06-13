use std::time::Instant;

pub struct ScheduleQueue<T> {
    sorted: bool,
    vec: Vec<(T, Instant)>,
}

impl<T> ScheduleQueue<T> {
    pub fn new() -> Self {
        ScheduleQueue {
            sorted: true,
            vec: Vec::new(),
        }
    }

    pub fn dequeue(&mut self) -> Option<(T, Instant)> {
        if self.vec.is_empty() {
            None
        } else {
            if !self.sorted {
                self.vec.sort_by_key(|r| r.1);
                self.sorted = true;
            }
            Some(self.vec.remove(0))
        }
    }

    pub fn enqueue(&mut self, record: (T, Instant)) {
        self.sorted = false;
        self.vec.push(record);
    }
}
