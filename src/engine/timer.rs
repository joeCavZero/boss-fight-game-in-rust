use std::time::Instant;

pub struct Timer {
    pub initial_time: Instant,
    pub wait_time: f32,
}

impl Timer {
    pub fn new(wait_time: f32) -> Timer {
        Timer {
            initial_time: Instant::now(),
            wait_time,
        }
    }

    pub fn reset(&mut self) {
        self.initial_time = Instant::now();
    }

    pub fn is_ready(&self) -> bool {
        self.initial_time.elapsed().as_secs_f32() as f32 >= self.wait_time
    }
}