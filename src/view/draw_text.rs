/// DrawText

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::view::transform::Transform;

pub struct DrawText {
    pub visible: bool,
    pub centered_horiz: bool,
    pub centered_vert: bool,
    pub text: String,
    pub color: Color,
    pub font: Option<Font>,
    pub font_size: u16,
    pub font_scale: f32,
    //pub text_params: TextParams,
}

impl DrawText {
    pub fn new(
        centered_horiz: bool,
        centered_vert: bool,
        text: &str,
        font_size: u16,
        font_name: Option<&str>) -> Self {
        
        let font = match font_name {
            Some(name) => Some(AssetLoader::get_font(name)),
            None => None,
        };

        Self {
            visible: true,
            centered_horiz,
            centered_vert,
            text: text.to_string(),
            color: WHITE,
            font,
            font_size,
            font_scale: 1.0,
        }
    }

    /// Returns the size of the drawn text.
    pub fn draw_size(&self) -> (f32, f32, f32) { // width, height, y offset from baseline
        let font = match &self.font {
            Some(font) => Some(font),
            None => None,
        };
        let dimensions = measure_text(&self.text, font, self.font_size, self.font_scale);
        (dimensions.width, dimensions.height, dimensions.offset_y)
    }

    pub fn draw(&mut self, transform: &Transform, color: Option<Color>) {
        if !self.visible { return }

        let (mut x, mut y, _rot) = transform.combined_x_y_rot();
        let (w, _h, offset_y) = self.draw_size();
  
        if self.centered_horiz {
            x -= w / 2.0;
        }

        if self.centered_vert {
            // I thought the math was this...
            // y += h / 2.0 - (h - baseline) / 2.0;
            // ...but this works. I don't get it.
            y += offset_y * 0.25;
        }

        let font = match &self.font {
            Some(font) => Some(font),
            None => None,
        };

        let color = color.unwrap_or(self.color);

        let params = TextParams {
            font,
            font_size: self.font_size,
            font_scale: self.font_scale,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color,
        };
        draw_text_ex(&self.text, x, y, params);
    }
}

