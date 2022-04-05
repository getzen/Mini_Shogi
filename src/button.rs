// Button

use macroquad::prelude::Texture2D;

use crate::sprite::Sprite;

pub struct Button {
    sprite: Sprite,
    texture_normal: Texture2D,
    texture_selected: Texture2D,
}

impl Button {
    pub fn new(
        position: (f32, f32), 
        texture_normal: Texture2D, 
        texture_selected: Texture2D) -> Self {
        Self {
            sprite: Sprite::new(position, texture_normal),
            texture_normal, texture_selected,
        }
    }
}