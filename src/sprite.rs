// Sprite
// A basic sprite implementation.
//
// An array of sprites can be sorted by z_order like so:
// my_sprites.sort_by(|a, b| a.z_order.cmp(&b.z_order));

use std::time::Duration;

use macroquad::prelude::*;

use crate::lerp::Lerp;

pub struct Sprite {
    pub position: (f32, f32),
    pub rotation: f32, // radians clockwise
    pub size: (f32, f32),
    //pub pivot: Option<(f32, f32)>, // Implementing this would require changes to 'contains' logic.

    pub texture: Texture2D,
    pub alt_texture: Option<Texture2D>,
    pub use_alt_texture: bool,

    pub color: Color,
    pub alt_color: Option<Color>,
    pub use_alt_color: bool,

    pub z_order: usize, // view can use this to sort
    pub is_visible: bool,
    pub id: Option<usize>,

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
        Self {
            position,
            rotation: 0.0,
            size: (texture.width(), texture.height()),

            /// See note above.
            /// Rotate around this point.
            /// When `None`, rotate around the texture's center.
            /// When `Some`, the coordinates are in screen-space.
            /// E.g. pivot (0,0) rotates around the top left corner of the screen, not of the
            //pivot: None,

            texture,
            alt_texture: None,
            use_alt_texture: false,

            color: WHITE,
            alt_color: None,
            use_alt_color: false,

            z_order: 0,
            is_visible: true,
            id: None,
            position_lerp: None,
        }
    }

    #[allow(dead_code)]
    pub fn scale_by(&mut self, scale: (f32, f32)) {
        self.size.0 = self.texture.width() * scale.0;
        self.size.1 = self.texture.height() * scale.1;
    }

    #[allow(dead_code)]
    /// Test whether the given point lies in the texture rectangle, considering rotation.
    pub fn contains(&self, point: (f32, f32)) -> bool {
        // Get the net test point relative to the sprite's position.
        let net_x = point.0 - self.position.0;
        let net_y = point.1 - self.position.1;
        // Rotate the point clockwise (the same direction as Macroquad's rotation). This is a
        // little different than the standard rotation formulas.
        let theta = self.rotation;
        let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
        let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
        // See if the rotated point is in the unrotated sprite rectangle.
        let (w, h) = self.size;
        f32::abs(rot_x) < w / 2.0 && f32::abs(rot_y) < h / 2.0
    }

    /// Returns the position at which the texture should be drawn,
    /// effectively centering at self.position.
    fn centered_position(&self) -> (f32, f32) {
        let (x, y) = self.position;
        let (w, h) = self.size;
        (x - w / 2.0, y - h / 2.0)
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

        let (x, y) = self.centered_position();

        let mut draw_color = self.color;
        if self.use_alt_color && self.alt_color.is_some() {
            draw_color = self.alt_color.unwrap();
        }

        let mut params = DrawTextureParams::default();
        params.dest_size = Some(Vec2::new(self.size.0, self.size.1));
        params.rotation = self.rotation;
        if let Some(piv) = self.pivot {
            params.pivot = Some(Vec2::new(piv.0, piv.1));
        }
        // params source, flip_x, etc. =

        // This 'if' statement avoids copying the texture.
        if !self.use_alt_texture {
            draw_texture_ex(self.texture, x, y, draw_color, params);
        } else {
            draw_texture_ex(self.alt_texture.unwrap(), x, y, draw_color, params);
        }
    }
}