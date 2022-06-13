/// Transform

use crate::view::{logi_pos, phys_pos};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: (f32, f32),
    pub rotation: f32,
    pub parent_position: (f32, f32),
    pub parent_rotation: f32,
}

impl Transform {
    pub fn new(position: (f32, f32), rotation: f32) -> Self {
        Self {
            position,
            rotation,
            parent_position: (0.0, 0.0),
            parent_rotation: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            rotation: 0.0,
            parent_position: (0.0, 0.0),
            parent_rotation: 0.0,
         }
    }

    pub fn set_parent(&mut self, parent: Transform) {
        self.parent_position = parent.position;
        self.parent_rotation = parent.rotation;
    }

    /// Returns a Transform with parent position and rotation added to the base
    /// position and rotation.
    pub fn combined(&self) -> Transform {
        let (x, y, rot) = self.combined_x_y_rot();
        Transform::new((x, y), rot)
    }

    /// Returns the x, y positions and the rotation plus the parent's x,y and r.
    pub fn combined_x_y_rot(&self) -> (f32, f32, f32) {
        (self.position.0 + self.parent_position.0,
        self.position.1 + self.parent_position.1,
        self.rotation + self.parent_rotation)
    }
}
