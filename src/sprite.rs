// Sprite
// A basic sprite implementation.
// Draws a centered texture that may be resized, rotated, and highlighted.
// These settings are reached through the draw_params struct:
//  - dest_size: Option<(f32, f32)>,
//  - source: Option<Rect>,
//  - rotation: f32, (radians clockwise)
//  - flip_x, flip_y: bool,
//  - pivot: Option<Vec2>
//
// An array of sprites can be sorted by z_order like so:
// my_sprites.sort_by(|a, b| a.z_order.cmp(&b.z_order));

use std::time::Duration;

use macroquad::prelude::*;

use crate::lerp::Lerp;

pub struct Sprite {
    pub position: (f32, f32),

    pub texture: Texture2D,
    pub alt_texture: Option<Texture2D>,
    pub use_alt_texture: bool,

    pub color: Color,
    pub alt_color: Option<Color>,
    pub use_alt_color: bool,

    pub draw_params: DrawTextureParams,
    pub rotation: f32, // breaking out separately from draw_params for convenience
    pub z_order: usize, // view can use this to sort
    pub is_visible: bool,

    pub id: Option<usize>, // usually a hash value

    // Private
    position_lerp: Option<Lerp>, // created automatically for animation
    // rotation_lerp, fade_lerp...
}

impl Sprite {
    #[allow(dead_code)]
    // Use AssetLoader instead.
    async fn load_texture(name: &str) -> Texture2D {
        let mut path = "./assets".to_owned();
        path.push_str(name);
        load_texture(&path).await.unwrap()
    }

    pub fn new(position: (f32, f32), texture: Texture2D) -> Self {
        let draw_params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: 0.0,
            flip_x: false, flip_y: false,
            pivot: None};

        Self {
            position,
            texture,
            alt_texture: None,
            use_alt_texture: false,
            color: WHITE,
            alt_color: None,
            use_alt_color: false,
            draw_params,
            rotation: 0.0,
            z_order: 0,
            is_visible: true,
            id: None,
            position_lerp: None,
        }
    }

    #[allow(dead_code)]
    pub fn get_size(&self) -> Option<(f32, f32)> {
        let opt_size = self.draw_params.dest_size;
        if let Some(size) = opt_size {
            return Some((size.x, size.y));
        }
        None
    }

    #[allow(dead_code)]
    pub fn set_size(&mut self, size: Option<(f32, f32)>) {
        if let Some(s) = size {
            self.draw_params.dest_size = Some(Vec2::new(s.0, s.1));
        } else {
            self.draw_params.dest_size = None;
        }
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: f32) {
        let dest_size = Vec2::new(
            self.texture.width() * scale, 
            self.texture.height() * scale);
        self.draw_params.dest_size = Some(dest_size);
    }

    #[allow(dead_code)]
    /// Test whether the given point lies in the texture rectangle, considering rotation.
    pub fn contains(&self, point: (f32, f32)) -> bool {
        // Get the net test point relative to the sprite's position.
        let net_x = point.0 - self.position.0;
        let net_y = point.1 - self.position.1;
        // Rotate the point clockwise (the same direction as Macroquad's rotation). This is a
        // little different than the standard rotation formulas.
        let theta = self.draw_params.rotation;
        let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
        let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
        // See if the rotated point is in the unrotated sprite rectangle.
        let (w, h) = self.draw_size();
        f32::abs(rot_x) < w / 2.0 && f32::abs(rot_y) < h / 2.0
    }

    /// Returns the position at which the texture should be drawn,
    /// effectively centering at self.position.
    fn centered_position(&self) -> (f32, f32) {
        let (x, y) = self.position;
        let (w, h) = self.draw_size();
        (x - w / 2.0, y - h / 2.0)
    }

    /// Returns the size of the drawn sprite.
    fn draw_size(&self) -> (f32, f32) {
        let mut width = self.texture.width();
        let mut height = self.texture.height();
        if let Some(dest_size) = self.draw_params.dest_size {
            width = dest_size.x;
            height = dest_size.y;
        }
        (width, height)
    }

    /// Perform animation updates and the like with the time_delta.
    /// If update did something, return true, otherwise false.
    pub fn update(&mut self, time_delta: Duration) -> bool {
        // Lerp animation
        if let Some(lerp) = &mut self.position_lerp {
            let results = lerp.update(time_delta);
            self.position = (results.0, results.1);
            if !results.2 {
                self.position_lerp = None;
            }
            return true;
        }
        false
    }

    /// Use the Lerp struct to move the sprite.
    pub fn animate_move(&mut self, to: (f32, f32), duration: Duration) {
        self.position_lerp = Some(Lerp::new(self.position, to, duration));
    }

    pub fn draw(&mut self) {
        if !self.is_visible { return; }
        let mut draw_texture = self.texture;
        let mut draw_color = self.color;

        if self.use_alt_texture && self.alt_texture.is_some() {
            draw_texture = self.alt_texture.unwrap();
        }
        if self.use_alt_color && self.alt_color.is_some() {
            draw_color = self.alt_color.unwrap();
        }

        let (x, y) = self.centered_position();
        self.draw_params.rotation = self.rotation;
        draw_texture_ex(draw_texture, x, y, draw_color, self.draw_params.clone());
    }
}