// Lerp

use std::time::Duration;

pub struct Lerp {
    start: (f32, f32),
    end: (f32, f32),
    duration: Duration,
    elasped: Duration,
}

impl Lerp {
    pub fn new(start: (f32, f32), end: (f32, f32), duration: Duration) -> Self {
        Self {
            start, end, duration,
            elasped: Duration::from_secs(0),
        }
    }

    pub fn calc_position(&self, percentage: f32) -> (f32, f32, bool) {
        if percentage >= 1.0 {
            return (self.end.0, self.end.1, true);
        }
        let x = self.start.0 + (self.end.0 - self.start.0) * percentage;
        let y = self.start.1 + (self.end.1 - self.start.1) * percentage;
        (x, y, false)
    }

    pub fn update(&mut self, time_delta: Duration) -> (f32, f32, bool) {
        self.elasped += time_delta;
        let percentage = self.elasped.as_secs_f32() / self.duration.as_secs_f32();
        self.calc_position(percentage)
    }
}