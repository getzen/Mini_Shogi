/// Transform

use crate::view::{logi_pos, phys_pos};

pub struct Transform {
    pub phys_position: (f32, f32),
    pub rotation: f32,
    // scale?
}

impl Transform {
    pub fn new(phys_position: (f32, f32), rotation: f32) -> Self {
        Self {
            phys_position,
            rotation,
        }
    }

    pub fn add(&self, other: &Transform) -> Transform {
        let x = self.phys_position.0 + other.phys_position.0;
        let y = self.phys_position.1 + other.phys_position.1;
        Self {
            phys_position: (x, y),
            rotation: self.rotation + other.rotation,
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
}