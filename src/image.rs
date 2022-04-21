/// Image
/// Draws a non-centered texture with a position and rotation.

use std::time::Duration;

use macroquad::prelude::*;

use crate::lerp::Lerp;
use crate::view::*;

pub struct Image {
    /// Position in physical pixels of the center.
    /// Use set_logi_position for logical pixel positioning.
    pub phys_position: (f32, f32),
    /// Rotation in radians clockwise
    pub rotation: f32,

    /// The displayed size of the texture in physical pixels. The initial size is automatically
    /// scaled by the window dpi scale.
    /// Use set_logi_size for logical pixel sizing.
    pub size: (f32, f32),

    pub texture: Texture2D,
    pub color: Color,
    pub z_order: usize, // view can use this to sort
    pub is_visible: bool,
    pub id: Option<usize>,

    // Private
    params: DrawTextureParams,
    fade_lerp: Option<Lerp>,
}

impl Image {
    #[allow(dead_code)]
    // Use AssetLoader instead.
    async fn load_texture(name: &str) -> Texture2D {
        let mut path = "./assets".to_owned();
        path.push_str(name);
        load_texture(&path).await.unwrap()
    }

    /// Creates a new Image with the given logical position and texture. The texture will
    /// be automatically scaled, if needed, for the dpi scale. In view.rs, see
    /// IMAGE_ASSETS_SCALE.
    pub fn new(logi_position: (f32, f32), texture: Texture2D) -> Self {
        Self {
            phys_position: phys_pos(logi_position),
            rotation: 0.0,
            size: (texture.width() * adj_scale(), texture.height() * adj_scale()),
            texture,
            color: WHITE,
            z_order: 0,
            is_visible: true,
            id: None,
            params: DrawTextureParams::default(),
            fade_lerp: None,
        }
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
    /// Get the size of the diplayed texture in logical pixels.
    pub fn get_logi_size(&self) -> (f32, f32) {
        (self.size.0 / View::dpi_scale(), self.size.1 / View::dpi_scale())
    }

    #[allow(dead_code)]
    /// Set the size of the displayed texture using logical pixel size.
    pub fn set_logi_size(&mut self, logi_size: (f32, f32)) {
        self.size.0 = logi_size.0 * View::adj_scale();
        self.size.1 = logi_size.1 * View::adj_scale();
    }

    /// Set the texture size using the given scale. Considers dpi scale.
    pub fn scale_by(&mut self, scale: (f32, f32)) {
        self.size.0 = self.texture.width() * View::adj_scale() * scale.0;
        self.size.1 = self.texture.height() * View::adj_scale() * scale.1;
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

    /// Draw the image.
    pub fn draw(&mut self) {
        if !self.is_visible { return; }

        let (x, y) = self.phys_position;
        self.params.dest_size = Some(Vec2::new(self.size.0, self.size.1));
        self.params.rotation = self.rotation;
        draw_texture_ex(self.texture, x, y, self.color, self.params.clone());
    }
}