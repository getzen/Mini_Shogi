/// Transform
/// 

use std::time::Duration;
use crate::view::*;
use crate::view::lerp::Lerp;

pub struct Transform {
    pub phys_position: (f32, f32),
    pub rotation: f32,
    // scale?

    // Private
    position_lerp: Option<Lerp>,
    // rotation, scale
}

impl Transform {
    pub fn new(phys_position: (f32, f32), rotation: f32) -> Self {
        Self {
            phys_position, rotation,
            position_lerp: None,
        }
    }

    #[allow(dead_code)]
    /// Get the logical position of the sprite.
    pub fn get_logi_position(&self) -> (f32, f32) {
        logi_pos(self.phys_position)
    }

    #[allow(dead_code)]
    /// Set the logical position of the sprite.
    pub fn set_logi_position(&mut self, logi_position: (f32, f32)) {
        self.phys_position = phys_pos(logi_position);
    }

    /// Perform animation updates and the like with the time_delta.
    /// If update did something, return true, otherwise false.
    pub fn update(&mut self, time_delta: Duration) -> bool {
        // Position animation
        if let Some(lerp) = &mut self.position_lerp {
            let results = lerp.update(time_delta);
            self.phys_position = (results.0, results.1);
            if !results.2 {
                self.position_lerp = None;
            }
            return true;
        }
        false
    }

    #[allow(dead_code)]
    /// Use the Lerp struct to move the sprite.
    pub fn animate_position(&mut self, to_logi_position: (f32, f32), duration: Duration) {
        let end = phys_pos(to_logi_position);
        self.position_lerp = Some(Lerp::new(self.phys_position, end, duration));
    }

}