// Text
// Draws text.
// These settings are reached through the text_params struct:
//  - font: Font,
//  - font_size: u16,
//  - font_scale: f32,
//  - font_scale_aspect: f32,
//  - color: Color,

use std::time::Duration;

use macroquad::prelude::*;

use crate::view::*;
use crate::view::animators::ColorAnimator;
use crate::view::transform::Transform;

use super::draw_text::DrawText;

pub struct Label {
    pub id: Option<usize>,
    pub transform: Transform,
    pub draw_text: DrawText,

    fader: Option<ColorAnimator>,
}

impl Label {
    pub fn new(
        logi_position: (f32, f32),
        centered: bool,
        text: String,
        logi_font_size: u16,
        font_name: Option<&str>) -> Self {

        let phys_position = phys_pos(logi_position);

        Self {
            id: None,
            transform: Transform::new(phys_position, 0.0),
            draw_text: DrawText::new(centered, text, logi_font_size, font_name),
            fader: None,
        }
    }

    #[allow(dead_code)]
    /// A convenience function to set the text.
    pub fn set_text(&mut self, text: String) {
        self.draw_text.text = text;
    }

    #[allow(dead_code)]
    /// A convenience function to set the text color.
    pub fn set_color(&mut self, color: Color) {
        self.draw_text.text_params.color = color;
    }

    /// The width of the text.
    pub fn width(&self) -> f32 {
        let (width, _, _) = self.draw_text.draw_size();
        width
    }

    /// The center of the text, ignoring the bits below the baseline.
    pub fn center(&self) -> (f32, f32) {
        let (width, height, offset) = self.draw_text.draw_size();
        (width / 2.0, height / 2.0 + offset / 2.0)
    }

    
    pub fn fade_out(&mut self, duration: Duration) {
        let end_color = Color::from_rgba(255, 255, 255, 0);
        self.fader = Some(ColorAnimator::new(WHITE, end_color, duration));
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        if let Some(fader) = &mut self.fader {
            fader.update(time_delta);
            if fader.complete {
                self.fader = None;
            }
            return true;
        }
        false
    }

    pub fn draw(&mut self) {
        if let Some(fader) = &self.fader {
            self.draw_text.draw(&self.transform, Some(fader.color));
        } else {
            self.draw_text.draw(&self.transform, None);
        }
    }
}