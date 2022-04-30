/// DrawText

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::view::*;
use crate::view::transform::Transform;

pub struct DrawText {
    pub visible: bool,
    pub centered_horiz: bool,
    pub centered_vert: bool,
    pub text: String,
    pub color: Color,
    pub text_params: TextParams,
}

impl DrawText {
    pub fn new(
        centered_horiz: bool,
        centered_vert: bool,
        text: &str,
        logi_font_size: u16,
        font_name: Option<&str>) -> Self {

        let mut text_params = TextParams::default();
        text_params.font_size = (logi_font_size as f32 * dpi_scale()) as u16;
        if let Some(name) = font_name {
            text_params.font = AssetLoader::get_font(name);
        }

        Self {
            visible: true,
            centered_horiz,
            centered_vert,
            text: text.to_string(),
            color: BLACK,
            text_params,
        }
    }

    /// Returns the size of the drawn text.
    pub fn draw_size(&self) -> (f32, f32, f32) { // width, height, y offset from baseline
        let font = self.text_params.font;
        let font_size = self.text_params.font_size;
        let font_scale = self.text_params.font_scale;
        let dimensions = measure_text(&self.text, Some(font), font_size, font_scale);
        (dimensions.width, dimensions.height, dimensions.offset_y)
    }

    pub fn draw(&mut self, transform: &Transform, color: Option<Color>) {
        if !self.visible { return }

        let (w, h, baseline) = self.draw_size();
        //println!("{}, {}, {}", w, h, baseline);
        let (mut x, mut y) = transform.phys_position;

        if self.centered_horiz {
            x -= w / 2.0;
        }

        if self.centered_vert {
            y += h / 2.0 - (h - baseline) / 2.0;
        }

        self.text_params.color = if color.is_some() { color.unwrap() } else { self.color };
        draw_text_ex(&self.text, x, y, self.text_params);
    }
}

