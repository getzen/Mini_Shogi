/// Drawable

use std::time::Duration;
use macroquad::prelude::*;
use crate::view::*;
use crate::view::lerp::Lerp;
use crate::view::transform::Transform;

pub struct Drawable {
    pub visible: bool,
    pub centered: bool,
    pub texture: Texture2D,
    pub size: (f32, f32),
    pub color: Color,
    pub z_order: usize,

    // Private
    params: DrawTextureParams,
    fade_lerp: Option<Lerp>, // set vis to false at end?
    // size, color
}

impl Drawable {
    pub fn new(texture: Texture2D, centered: bool) -> Self {
        Self {
            visible: true,
            centered, texture,
            size: (texture.width() * adj_scale(), texture.height() * adj_scale()),
            color: WHITE,
            z_order: 0,
            params: DrawTextureParams::default(),
            fade_lerp: None,
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

    #[allow(dead_code)]
    /// Perform animation updates and the like with the time_delta.
    /// If update did something, return true, otherwise false.
    pub fn update(&mut self, time_delta: Duration) -> bool {
        // Fade animation
        if let Some(lerp) = &mut self.fade_lerp {
            let results = lerp.update(time_delta);
            self.color.a = results.0;
            if !results.2 {
                self.fade_lerp = None;
            }
            return true;
        }
        false
    }

    #[allow(dead_code)]
    /// Use the Lerp struct to fade out the sprite.
    pub fn animate_fade_out(&mut self, duration: Duration) {
        self.fade_lerp = Some(Lerp::new((1.0, 0.0), (0.0, 0.0), duration));
    }

    pub fn draw(&mut self, transform: &Transform) {
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
        
        draw_texture_ex(self.texture, x, y, self.color, self.params.clone());
    }
}