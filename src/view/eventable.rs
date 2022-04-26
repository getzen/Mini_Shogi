/// Eventable

use macroquad::prelude::*;

use crate::view::transform::Transform;
use crate::view::draw_texture::DrawTexture;

pub enum Event {
    MouseEntered,
    MouseExited,
    LeftMousePressed,
    LeftMouseReleased,
    // Right
    // DragStarted
}

pub struct Eventable {
    pub enabled: bool,
    pub mouse_over: bool,
    pub left_mouse_down: bool,
    // right_
    // dragging: bool, drag_start_pos, drag_pos_now
}

impl Eventable {
    pub fn new() -> Self {
        Self {
            enabled: true,
            mouse_over: false,
            left_mouse_down: false,
        }
    }

    /// Test whether the physical point lies in the texture rectangle, considering rotation.
    /// Note: Macroquad's mouse_position() gives the physical location of the mouse.
    pub fn contains_phys_position(&self, phy_position: (f32, f32), transform: &Transform, draw: &DrawTexture) -> bool {
        // Get the net test point relative to the sprite's position.
        let net_x = phy_position.0 - transform.phys_position.0;
        let net_y = phy_position.1 - transform.phys_position.1;
        // Rotate the point clockwise (the same direction as Macroquad's rotation). This is a
        // little different than the standard rotation formulas.
        let theta = transform.rotation;
        let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
        let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
        // See if the rotated point is in the unrotated sprite rectangle.
        let (w, h) = draw.size;
        if draw.centered {
            f32::abs(rot_x) <= w / 2.0 && f32::abs(rot_y) <= h / 2.0
        } else {
            rot_x >= 0.0 && rot_x <= w && rot_y >= 0.0 && rot_y <= h
        }        
    }

    pub fn process_events(&mut self, transform: &Transform, draw: &DrawTexture) -> Option<Event> {
        if !self.enabled { return None }

        let mouse_over = self.contains_phys_position(mouse_position(), transform, draw);

        if mouse_over && !self.mouse_over {
            self.mouse_over = true;
            return Some(Event::MouseEntered);
        }

        if !mouse_over && self.mouse_over {
            self.mouse_over = false;
            return Some(Event::MouseExited);
        }
       
        let left_mouse_down = is_mouse_button_down(MouseButton::Left);

        if mouse_over && left_mouse_down {
            if !self.left_mouse_down {
                self.left_mouse_down = true;
                return Some(Event::LeftMousePressed);
            }
        }
        self.left_mouse_down = false;

        if mouse_over && is_mouse_button_released(MouseButton::Left) {
            return Some(Event::LeftMouseReleased);
        }
        None
    }
}