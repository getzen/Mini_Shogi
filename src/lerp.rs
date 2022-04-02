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

    /// Get the values based on the given percentage. Percentage above 100%
    /// returns the end values and true as the third tuple argument,
    /// otherwise false.
    pub fn calc_values(&self, percentage: f32) -> (f32, f32, bool) {
        if percentage >= 1.0 {
            return (self.end.0, self.end.1, true);
        }
        let x: f32;
        let y: f32;
        let linear = false;
        if linear {
            x = self.start.0 + (self.end.0 - self.start.0) * percentage;
            y = self.start.1 + (self.end.1 - self.start.1) * percentage;
        } else { // ease out
            x = self.start.0 + (self.end.0 - self.start.0) * f32::sin(3.14159 * 0.5 * percentage);
            y = self.start.1 + (self.end.1 - self.start.1) * f32::sin(3.14159 * 0.5 * percentage);
        }
        (x, y, false)
    }

    /// Update the lerp with the time_delta and return the new values plus
    /// 'true' if the animation is finished or 'false' if it isn't.
    pub fn update(&mut self, time_delta: Duration) -> (f32, f32, bool) {
        self.elasped += time_delta;
        let percentage = self.elasped.as_secs_f32() / self.duration.as_secs_f32();
        self.calc_values(percentage)
    }
}