/// ViewIntro
/// A splash intro/title view that fades out.

use std::time::Duration;

use crate::asset_loader::AssetLoader;
use crate::sprite::Sprite;

pub struct ViewIntro {
    /// When true, this view should be update and drawn.
    pub visible: bool,
    // Private
    sprite: Sprite,
    elapsed_time: Duration,
    fade_active: bool,
}

impl ViewIntro {
    pub async fn new() -> Self {       
        let texture = AssetLoader::get_texture("intro");

        Self {
            visible: true,
            sprite: Sprite::new((400., 380.), texture),
            elapsed_time: Duration::ZERO,
            fade_active: false,
        }
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        if !self.visible { return false }

        if self.fade_active {
            // Update fade.
            self.fade_active = self.sprite.update(time_delta);
            self.visible = self.fade_active;
        } else {
            // Start fade if it's time.
            self.elapsed_time += time_delta;
            if self.elapsed_time > Duration::from_secs(3) {
                self.sprite.animate_fade_out(Duration::from_secs(3));
                self.fade_active = true;
            }
        }
        self.fade_active
    }

    pub fn draw(&mut self) {
        if !self.visible { return }
        self.sprite.draw();
    }
}