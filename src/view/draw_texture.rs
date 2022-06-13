/// Drawable

use macroquad::prelude::*;

use crate::view::transform::Transform;

// The size of the textures divided by the presentation (draw) size.
// 2.0 is common for high dpi monitors.
pub const TEXTURE_SCALE: f32 = 2.0;

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
        let mut sprite = Self {
            visible: true,
            centered,
            texture,
            size: (0., 0.),
            z_order: 0,
            params: DrawTextureParams::default(),
        };
        sprite.set_texture(texture);
        sprite
    }

    #[allow(dead_code)]
    pub fn set_texture(&mut self, texture: Texture2D) {
        self.texture = texture;
        self.size = (texture.width() / TEXTURE_SCALE, texture.height() / TEXTURE_SCALE);
    }

    pub fn draw(&mut self, transform: &Transform, color: Option<Color>) {
        if !self.visible { return }

        let (mut x, mut y, rot) = transform.combined_x_y_rot();

        if self.centered {
            x -= self.size.0 / 2.0;
            y -= self.size.1 / 2.0;
        }

        self.params.rotation = rot;
        self.params.dest_size = Some(Vec2::new(self.size.0, self.size.1));
        
        let draw_color = color.unwrap_or(WHITE);
        draw_texture_ex(self.texture, x, y, draw_color, self.params.clone());
    }
}