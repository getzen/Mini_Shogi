// Sprite
// A basic sprite implementation.
//
// An array of sprites can be sorted by z_order like so:
// my_sprites.sort_by(|a, b| a.z_order.cmp(&b.z_order));

use std::time::Duration;

use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::view::draw_texture::DrawTexture;
use crate::view::eventable::Event;
use crate::view::eventable::Eventable;
use crate::view::transform::Transform;

use crate::view::*;

use super::animators::PositionAnimator;

pub struct Sprite {
    pub id: Option<usize>,
    pub transform: Transform,
    pub drawable: DrawTexture,
    pub eventable: Eventable,

    // This app
    mover: Option<PositionAnimator>,

    pub alt_texture: Option<Texture2D>,
    pub use_alt_texture: bool,

    pub alt_color: Option<Color>,
    pub use_alt_color: bool,
}

impl Sprite {
    /// Creates a new Sprite with the given logical position, texture, and optonal id.
    pub fn new(logi_position: (f32, f32), texture: Texture2D, id: Option<usize>) -> Self {
        let phys_position = phys_pos(logi_position);

        Self {
            id,
            transform: Transform::new(phys_position, 0.0),
            drawable: DrawTexture::new(texture, true),
            eventable: Eventable::new(),

            mover: None,
            alt_texture: None,
            use_alt_texture: false,
            alt_color: None,
            use_alt_color: false,
        }
    }

    pub fn move_to(&mut self, logi_end_pos: (f32, f32), duration: Duration) {
        let phys_end_pos = phys_pos(logi_end_pos);
        self.mover = Some(PositionAnimator::new(self.transform.phys_position, phys_end_pos, duration));
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        if let Some(mover) = &mut self.mover {
            mover.update(time_delta);
            self.transform.phys_position = mover.position;
            if mover.complete {
                self.mover = None;
            }
            return true;
        }
        false
    }

    // Convenience method
    pub fn contains_phys_position(&self, phy_position: (f32, f32)) -> bool {
        self.eventable.contains_phys_position(phy_position, &self.transform, &self.drawable)
    }

    #[allow(dead_code)]
    pub fn process_events(&mut self) -> Option<Event> {
        self.eventable.process_events(&self.transform, &self.drawable)
    }

    pub fn draw(&mut self) {
        if self.use_alt_color {
            self.drawable.draw(&self.transform, self.alt_color);
        } else {
            self.drawable.draw(&self.transform, None);
        }
        
    }
}