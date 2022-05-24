/// Animators

use std::time::Duration;

use macroquad::prelude::Color;

pub struct ColorAnimator {
    pub complete: bool,
    /// The current color
    pub color: Color, 

    start_color: Color,
    end_color: Color,
    duration: Duration,
    elapsed: Duration, 
}

impl ColorAnimator {
    pub fn new(start_color: Color, end_color: Color, duration: Duration) -> Self {
        Self {
            complete: false,
            color: start_color,
            start_color,
            end_color,
            duration,
            elapsed: Duration::ZERO,
        }
    }
    /// Update the lerp with the time_delta and return the new values plus
    /// 'true' if the animation is running or 'false' if it isn't.
    pub fn update(&mut self, time_delta: Duration) {
        if self.complete { return }

        self.elapsed += time_delta;
        
        let percentage = self.elapsed.as_secs_f32() / self.duration.as_secs_f32();
        if percentage >= 1.0 {
            self.complete = true;
            self.color = self.end_color;
            return;
        }
        self.color.r = self.start_color.r + (self.end_color.r - self.start_color.r) * percentage;
        self.color.g = self.start_color.g + (self.end_color.g - self.start_color.g) * percentage;
        self.color.b = self.start_color.b + (self.end_color.b - self.start_color.b) * percentage;
        self.color.a = self.start_color.a + (self.end_color.a - self.start_color.a) * percentage;
    }
}

pub struct PositionAnimator {
    pub complete: bool,
    /// The current position
    pub position: (f32, f32),

    start_position: (f32, f32),
    end_position: (f32, f32),
    duration: Duration,
    elapsed: Duration, 
}

impl PositionAnimator {
    pub fn new(start_position: (f32, f32), end_position: (f32, f32), duration: Duration) -> Self {
        Self {
            complete: false,
            position: start_position,
            start_position,
            end_position,
            duration,
            elapsed: Duration::ZERO,
        }
    }

    /// Update the lerp with the time_delta and return the new values plus
    /// 'true' if the animation is running or 'false' if it isn't.
    pub fn update(&mut self, time_delta: Duration) {
        if self.complete { return }

        self.elapsed += time_delta;

        let percentage = self.elapsed.as_secs_f32() / self.duration.as_secs_f32();
        if percentage >= 1.0 {
            self.complete = true;
            self.position = self.end_position;
            return;
        }
        //self.calc_ease_out(percentage)
        self.calc_linear(percentage)
    }

    #[allow(dead_code)]
    pub fn calc_linear(&mut self, percentage: f32) {
        self.position.0 = self.start_position.0 + (self.end_position.0 - self.start_position.0) * percentage;
        self.position.1 = self.start_position.1 + (self.end_position.1 - self.start_position.1) * percentage;
    }

    #[allow(dead_code)]
    pub fn calc_ease_out(&mut self, percentage: f32) {
        self.position.0 = self.start_position.0 + (self.end_position.0 - self.start_position.0) * f32::sin(std::f32::consts::PI * 0.5 * percentage);
        self.position.1 = self.start_position.1 + (self.end_position.1 - self.start_position.1) * f32::sin(std::f32::consts::PI * 0.5 * percentage);
    }
}