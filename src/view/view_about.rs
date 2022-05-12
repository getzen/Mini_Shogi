/// ViewAbout

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::view::button::Button;
use crate::view::button::ButtonEvent;
use crate::view::image::Image;
use crate::view::transform::Transform;


pub enum ViewAboutMessage {
    ShouldClose,
}

pub struct ViewAbout {
    tx: Sender<ViewAboutMessage>,
    transform: Transform,
    image: Image,
    okay_button: Button,
}

impl ViewAbout {
    pub async fn new(tx: Sender<ViewAboutMessage>) -> Self {       
        let texture = AssetLoader::get_texture("view_about");

        Self {
            tx,
            transform: Transform::new((200., 253.), 0.),
            image: Image::new((0., 0.), texture, false, None),
            okay_button: Button::new((170., 215.), 0, "Okay", None),
        }
    }

    pub fn process_events(&mut self) {
        // Key presses.
        if is_key_released(KeyCode::Escape) {
            self.send_close_message();
        }
        // Button
        let event_opt = self.okay_button.process_events();
        if event_opt == Some(ButtonEvent::Pushed(None)) {
            self.send_close_message();
        }
    }

    fn send_close_message(&self) {
        self.tx.send(ViewAboutMessage::ShouldClose).expect("About message send error.");
    }

    pub fn draw(&mut self) {
        self.image.transform.set_parent(self.transform);
        self.image.draw();

        self.okay_button.texture_transform.set_parent(self.transform);
        self.okay_button.draw();
    }
}