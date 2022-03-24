// Text
// Draws text.
// These settings are reached through the text_params struct:
//  - font: Font,
//  - font_size: u16,
//  - font_scale: f32,
//  - font_scale_aspect: f32,
//  - color: Color,

use macroquad::prelude::*;

const FONT_PATH: &str = "./assets/";

pub struct Text {
    pub text: String,
    pub position: (f32, f32),
    pub centered: bool,
    pub text_params: TextParams,
}

impl Text {
    pub async fn new(text: String, position: (f32, f32), font_size: u16, font_name: Option<&str>) -> Self {
        let mut slf =
        Self {
            text,
            position,
            text_params: TextParams { font_size, ..Default::default() },
            centered: true,
        };
        if let Some(name) = font_name {
            slf.text_params.font = Text::load_font(name).await;
        }
        slf
    }

    /// Loads the ttf or ttc font.
    pub async fn load_font(name: &str) -> Font {
        let mut path = FONT_PATH.to_owned();
        path.push_str(name);
        load_ttf_font(&path).await.unwrap()
    }

    #[allow(dead_code)]
    /// A convenience function to set the sprite's rotation.
    pub fn set_color(&mut self, color: Color) {
        self.text_params.color = color;
    }

    /// Returns the position at which the text should be drawn, considering centering.
    fn draw_position(&self) -> (f32, f32) {
        let (x, y) = self.position;
        let (w, h) = self.draw_size();
        if self.centered {
            (x - w / 2.0, y - h / 2.0)
        } else {
            (x, y)
        }
    }

    /// Returns the size of the drawn text.
    fn draw_size(&self) -> (f32, f32) {
        let font = self.text_params.font;
        let font_size = self.text_params.font_size;
        let font_scale = self.text_params.font_scale;
        let dimensions = measure_text(&self.text, Some(font), font_size, font_scale);
        (dimensions.width, dimensions.height)
    }

    pub fn draw(&mut self) {
        let (x, y) = self.draw_position();
        draw_text_ex(&self.text, x, y, self.text_params);
    }
}