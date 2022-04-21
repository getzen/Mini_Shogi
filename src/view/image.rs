/// Image
/// Draws a non-centered texture with a position and rotation.

use std::time::Duration;

use macroquad::prelude::{Texture2D, Color, WHITE};

use crate::view::draw_texture::DrawTexture;
use crate::view::transform::Transform;

use crate::view::*;

use super::animators::ColorAnimator;

pub struct Image {
    pub id: Option<usize>,
    pub transform: Transform,
    pub drawable: DrawTexture,

    fader: Option<ColorAnimator>,
}

impl Image {
    pub fn new(logi_position: (f32, f32), texture: Texture2D, centered: bool, id: Option<usize>) -> Self {
        let phys_position = phys_pos(logi_position);

        Self {
            id,
            transform: Transform::new(phys_position, 0.0),
            drawable: DrawTexture::new(texture, centered),
            fader: None,
        }
    }

    // Convenience methods

    pub fn fade_out(&self, duration: Duration) {
        let end_color = Color::from_rgba(255, 255, 255, 0);
        self.fader = Some(ColorAnimator::new(WHITE, end_color, duration));
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        if let Some(fader) = self.fader {
            fader.update(time_delta);
            if fader.complete {
                self.fader = None;
            }
            return true;
        }
        false
    }

    pub fn draw(&self) {
        if let Some(fader) = self.fader {
            self.drawable.draw(&self.transform, Some(fader.color));
        } else {
            self.drawable.draw(&self.transform, None);
        }
    }
}