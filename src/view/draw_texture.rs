/// Drawable

use macroquad::prelude::*;

use crate::view::*;
use crate::view::transform::Transform;

pub struct DrawTexture {
    pub visible: bool,
    pub centered: bool,
    pub texture: Texture2D,
    pub size: (f32, f32),
    pub z_order: usize,

    // Private
    params: DrawTextureParams,
}

impl DrawTexture {
    pub fn new(texture: Texture2D, centered: bool) -> Self {
        Self {
            visible: true,
            centered, texture,
            size: (texture.width() * adj_scale(), texture.height() * adj_scale()),
            z_order: 0,
            params: DrawTextureParams::default(),
        }
    }

    #[allow(dead_code)]
    /// Get the size of the diplayed texture in logical pixels.
    pub fn get_logi_size(&self) -> (f32, f32) {
        (self.size.0 / dpi_scale(), self.size.1 / dpi_scale())
    }

    #[allow(dead_code)]
    /// Set the size of the displayed texture using logical pixel size.
    pub fn set_logi_size(&mut self, logi_size: (f32, f32)) {
        self.size.0 = logi_size.0 * adj_scale();
        self.size.1 = logi_size.1 * adj_scale();
    }

    pub fn set_texture(&mut self, texture: Texture2D) {
        self.texture = texture;
        self.size = (texture.width() * adj_scale(), texture.height() * adj_scale());
    }

    pub fn draw(&mut self, transform: &Transform, color: Option<Color>) {
        if !self.visible { return }

        let (x, y) = match self.centered {
            true => {
                (transform.phys_position.0 - self.size.0 / 2.0,
                 transform.phys_position.1 - self.size.1 / 2.0)
            },
            false => transform.phys_position
        };

        self.params.rotation = transform.rotation;
        self.params.dest_size = Some(Vec2::new(self.size.0, self.size.1));

        let draw_color = if color.is_some() { color.unwrap() } else { WHITE };
        draw_texture_ex(self.texture, x, y, draw_color, self.params.clone());
    }
}