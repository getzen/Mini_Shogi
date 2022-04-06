// Button
// The code is similar to Sprite, but with button-specific features.
// Also, the position refers to the top-left corner of the texture instead
// of the center.

use macroquad::prelude::*;

#[allow(dead_code)]
pub enum ButtonMode {
    Push,
    Toggle,
}

pub struct Button {
    pub position: (f32, f32),

    pub texture: Texture2D,
    pub disabled_texture: Option<Texture2D>,
    pub selected_texture: Option<Texture2D>,

    pub color: Color,
    pub disabled_color: Option<Color>,
    pub selected_color: Option<Color>,

    pub draw_params: DrawTextureParams,
    pub z_order: usize,
    pub mode: ButtonMode,

    pub is_visible: bool,
    pub is_enabled: bool,
    pub is_mouse_over: bool,
    pub is_selected: bool,
}

impl Button {
    pub fn new(
        position: (f32, f32), 
        texture: Texture2D, 
        mode: ButtonMode) -> Self {

        let draw_params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: 0.0,
            flip_x: false, flip_y: false,
            pivot: None
        };

        Self {
            position, texture, mode,
            disabled_texture: None,
            selected_texture: None,
            color: WHITE,
            disabled_color: Some(DARKGRAY),
            selected_color: Some(YELLOW),
            draw_params,
            z_order: 0,
            is_visible: true,
            is_enabled: true,
            is_mouse_over: false,
            is_selected: false,
        }
    }

    #[allow(dead_code)]
    /// Test whether the given point lies in the texture rectangle, considering rotation.
    pub fn contains(&self, point: (f32, f32)) -> bool {
        let (w, h) = self.draw_size();
        // Get the net test point relative to the sprite's position.
        let net_x = point.0 - self.position.0 - w / 2.0;
        let net_y = point.1 - self.position.1 - h / 2.0;
        // Rotate the point clockwise (the same direction as Macroquad's rotation). This is a
        // little different than the standard rotation formulas.
        let theta = self.draw_params.rotation;
        let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
        let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
        // See if the rotated point is in the unrotated sprite rectangle.
        f32::abs(rot_x) < w / 2.0 && f32::abs(rot_y) < h / 2.0
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

    pub fn handle_mouse_events(&mut self) {
        if !self.is_visible || !self.is_enabled { return; }

        self.is_mouse_over = self.contains(mouse_position());
        let button_pressed = is_mouse_button_down(MouseButton::Left);
        // See if button was released *this frame*.
        let button_released = is_mouse_button_released(MouseButton::Left);

        match &self.mode {
            ButtonMode::Push => {
                self.is_selected = self.is_mouse_over && button_pressed;
                if self.is_selected && button_released {
                    // send pushed event, then...
                    self.is_selected = false;
                }
            },
            ButtonMode::Toggle => {
                if self.is_mouse_over && button_released {
                    self.is_selected = !self.is_selected;
                }
            },
        }
    }

    pub fn draw(&mut self) {
        if !self.is_visible { return ;}
        
        let mut draw_texture = self.texture;
        let mut draw_color = self.color;

        if self.is_enabled {
            if self.is_selected {
                if self.selected_texture.is_some() {
                    draw_texture = self.selected_texture.unwrap();
                }
                if self.selected_color.is_some() {
                    draw_color = self.selected_color.unwrap();
                }
            }
        } else { // disabled
            if self.disabled_texture.is_some() {
                draw_texture = self.disabled_texture.unwrap();
            }
            if self.disabled_color.is_some() {
                draw_color = self.disabled_color.unwrap();
            }
        }
        let (x, y) = self.position;
        draw_texture_ex(draw_texture, x, y, draw_color, self.draw_params.clone());
    }
}