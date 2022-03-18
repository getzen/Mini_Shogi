// Sprite
// A basic sprite implementation.
// Draws a centered texture that may be resized, rotated, and highlighted.
// These settings are reached through the draw_params struct:
//  - dest_size: Option<(f32, f32)>,
//  - source: Option<Rect>,
//  - rotation: f32, (radians clockwise)
//  - flip_x, flip_y: bool,
//  - pivot: Option<Vec2>

use macroquad::prelude::*;

use crate::game::Coord;

const TEXTURE_PATH: &str = "./assets/";

pub struct Sprite {
    pub texture: Texture2D,
    pub position: (f32, f32),
    pub color: Color,
    pub highlighted: bool,
    pub highlight_color: Color,
    pub draw_params: DrawTextureParams,
    // For this game in particular:
    pub id: usize,
    pub coord: Coord,
}

impl Sprite {
    pub fn new(texture: Texture2D, position: (f32, f32)) -> Self {
        let draw_params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: 0.0,
            flip_x: false, flip_y: false,
            pivot: None};
        Self {
            texture, position,
            color: WHITE,
            highlighted: false,
            highlight_color: LIGHTGRAY,
            draw_params,
            id: 0,
            coord: Coord(0,0),
        }
    }

    pub async fn load_texture(name: &str) -> Texture2D {
        let mut path = TEXTURE_PATH.to_owned();
        path.push_str(name);
        load_texture(&path).await.unwrap()
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