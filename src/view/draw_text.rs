/// DrawText

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::view::*;
use crate::view::transform::Transform;

pub struct DrawText {
    pub visible: bool,
    pub centered: bool,
    pub text: String,
    pub color: Color,
    pub text_params: TextParams,
}

impl DrawText {
    pub fn new(
        centered: bool,
        text: String,
        logi_font_size: u16,
        font_name: Option<&str>) -> Self {

        let mut text_params = TextParams::default();
        text_params.font_size = (logi_font_size as f32 * dpi_scale()) as u16;
        if let Some(name) = font_name {
            text_params.font = AssetLoader::get_font(name);
        }

        Self {
            visible: true,
            centered,
            text,
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

        let (x, y) = match self.centered {
            true => {
                let draw_size = self.draw_size();
                (transform.phys_position.0 - draw_size.0 / 2.0,
                 transform.phys_position.1 - draw_size.1 / 2.0)
            },
            false => transform.phys_position
        };

        self.text_params.color = if color.is_some() { color.unwrap() } else { self.color };
        draw_text_ex(&self.text, x, y, self.text_params);
    }
}

