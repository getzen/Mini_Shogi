// Button
// A wrapper around a Sprite with some conveniences.

use macroquad::prelude::Texture2D;

use crate::sprite::Sprite;

#[allow(dead_code)]
pub enum ButtonMode {
    Push,
    Toggle,
}

pub struct Button {
    sprite: Sprite,
    pub texture_off: Texture2D,
    pub texture_on: Texture2D,
    pub mode: ButtonMode,
    is_on: bool,
}

impl Button {
    pub fn new(
        position: (f32, f32), 
        texture_off: Texture2D, 
        texture_on: Texture2D,
        mode: ButtonMode) -> Self {
        Self {
            sprite: Sprite::new(position, texture_off),
            texture_off, texture_on, mode,
            is_on: false,
        }
    }

    #[allow(dead_code)]
    /// Get the button's position.
    pub fn position(&self) -> (f32, f32) {
        self.sprite.position
    }

    #[allow(dead_code)]
    /// Set the button's position.
    pub fn set_position(&mut self, position: (f32, f32)) {
        self.sprite.position = position;
    }

    #[allow(dead_code)]
    fn contains(&self, point: (f32, f32)) -> bool {
        self.sprite.contains(point)
    }

    // #[allow(dead_code)]
    // // Convenience function to automatically highlight when mouse is over.
    // pub fn select_on_mouse_over(&mut self) {
    //     let contains = self.contains(mouse_position());
    //     if contains {
    //         self.sprite.texture = self.texture_on;
    //     } else {
    //         self.sprite.texture = self.texture_off;
    //     }
    // }

    pub fn update(&mut self, mouse_pos: (f32, f32), button_released: bool) -> bool {
        let contains = self.contains(mouse_pos);
        match &self.mode {
            ButtonMode::Push => {
                self.is_on = contains;
                contains && button_released
            },
            ButtonMode::Toggle => {
                if contains && button_released {
                    self.is_on = !self.is_on;
                }
                self.is_on
            },
        }
    }

    fn draw(&mut self) {
        if self.is_on {
            self.sprite.update_texture(self.texture_on);
        } else {
            self.sprite.update_texture(self.texture_off);
        }
        self.sprite.draw();
    }
}