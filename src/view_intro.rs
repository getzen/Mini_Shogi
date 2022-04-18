/// ViewIntro
/// A splash intro/title view that fades out.

use std::time::Duration;

//use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::sprite::Sprite;

pub struct ViewIntro {
    sprite: Sprite,
    /// True when the animation is active.
    active: bool,
}

impl ViewIntro {
    pub async fn new() -> Self {       
        let texture = AssetLoader::get_texture("intro");

        Self {
            sprite: Sprite::new((400., 250.), texture),
            active: false,
        }
    }

    pub fn prepare(&self) {
        self.sprite.animate_fade_out(Duration::from_secs(3));
        self.active = true;
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        self.active = self.sprite.update(time_delta);
        self.active
    }

    pub fn draw(&mut self) {
        self.sprite.draw();
    }
}