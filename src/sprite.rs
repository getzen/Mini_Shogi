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
//use crate::view_game::ASSET_PATH;

#[derive(PartialEq, Eq)]
pub enum SpriteKind {
    Default,
    Square,
    Reserve,
    Piece,
}

pub struct Sprite {
    pub id: Option<usize>, // usually a hash value
    pub kind: SpriteKind,
    pub texture: Texture2D,
    pub position: (f32, f32),
    pub color: Color,
    pub highlighted: bool,
    pub highlight_color: Color,
    pub draw_params: DrawTextureParams,
    pub z_order: usize, // view can use this to sort
    position_lerp: Option<Lerp>,
}

impl Sprite {
    pub fn new(kind: SpriteKind, texture: Texture2D, position: (f32, f32)) -> Self {
        let draw_params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: 0.0,
            flip_x: false, flip_y: false,
            pivot: None};
        Self {
            id: None,
            kind, texture, position,
            color: WHITE,
            highlighted: false,
            highlight_color: LIGHTGRAY,
            draw_params,
            z_order: 0,
            position_lerp: None,
        }
    }

    #[allow(dead_code)]
    pub async fn load_texture(name: &str) -> Texture2D {
        let mut path = "./assets".to_owned();
        path.push_str(name);
        load_texture(&path).await.unwrap()
    }

    pub fn update_texture(&mut self, new_texture: Texture2D) {
        if self.texture != new_texture {
            self.texture = new_texture;
        }
    }

    #[allow(dead_code)]
    /// A convenience function to set the sprite's rotation.
    pub fn set_rotation(&mut self, theta: f32) {
        self.draw_params.rotation = theta;
    }

    #[allow(dead_code)]
    /// A convenience function to set the sprite's draw size.
    pub fn set_size(&mut self, size: Option<(f32, f32)>) {
        if let Some(s) = size {
            self.draw_params.dest_size = Some(Vec2::new(s.0, s.1));
        } else {
            self.draw_params.dest_size = None;
        }
    }

    /// Perform animation updates and the like with the time_delta.
    /// If update did something, return true, otherwise false.
    pub fn update(&mut self, time_delta: Duration) -> bool {
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

    /// Returns the position at which the sprite should be drawn,
    /// effectively centering at self.position.
    fn draw_position(&self) -> (f32, f32) {
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

    pub fn draw(&mut self) {
        let (x, y) = self.draw_position();
        let mut draw_color = &self.color;
        if self.highlighted {
            draw_color = &self.highlight_color;
        }
        draw_texture_ex(self.texture, x, y, *draw_color, self.draw_params.clone());
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

    #[allow(dead_code)]
    // Convenience function to automatically highlight when mouse is over.
    pub fn highlight_on_mouse_over(&mut self) -> bool {
        let contains = self.contains(mouse_position());
        self.highlighted = contains;
        contains
    }
}