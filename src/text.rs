// Text
// Draws text.
// These settings are reached through the text_params struct:
//  - font: Font,
//  - font_size: u16,
//  - font_scale: f32,
//  - font_scale_aspect: f32,
//  - color: Color,

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;

pub struct Text {
    pub phys_position: (f32, f32),
    pub text: String,
    pub centered: bool,
    pub text_params: TextParams,
    pub is_visible: bool,
}

impl Text {
    /// Creates a new Text with the given logical position, text, font size, and font name.
    /// The font size will be automatically adjusted to the dpi scale.
    pub async fn new(
        logi_position: (f32, f32), 
        text: String, 
        logi_font_size: u16, 
        font_name: Option<&str>) -> Self {

        let mut params = TextParams::default();
        params.font_size = (logi_font_size as f32 * View::dpi_scale()) as u16;
        if let Some(name) = font_name {
            params.font = AssetLoader::get_font(name);
        }

        Self {
            phys_position: View::phys_pos(logi_position), 
            text,
            text_params: params,
            centered: false,
            is_visible: true,
        }
    }

    #[allow(dead_code)]
    /// Loads the ttf or ttc font. Use AssetLoader instead.
    pub async fn load_font(name: &str) -> Font {
        let mut path = ".assets/".to_owned();
        path.push_str(name);
        load_ttf_font(&path).await.unwrap()
    }

    #[allow(dead_code)]
    /// Get the logical position of the sprite.
    pub fn get_logi_position(&self) -> (f32, f32) {
        View::logi_pos(self.phys_position)
    }

    #[allow(dead_code)]
    /// Set the logical position of the sprite.
    pub fn set_logi_position(&mut self, logi_position: (f32, f32)) {
        self.phys_position = View::phys_pos(logi_position);
    }

    #[allow(dead_code)]
    /// A convenience function to set the text color.
    pub fn set_color(&mut self, color: Color) {
        self.text_params.color = color;
    }

    /// Returns the physical position at which the text should be drawn, considering centering.
    fn draw_position(&self) -> (f32, f32) {
        let (x, y) = self.phys_position;
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
        if !self.is_visible { return; }
        let (x, y) = self.draw_position();
        draw_text_ex(&self.text, x, y, self.text_params);
    }
}