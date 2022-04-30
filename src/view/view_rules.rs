// ViewRules

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::view::button::Button;
use crate::view::button::ButtonEvent;
use crate::view::image::Image;

use super::button3::Button3;

pub enum ViewRulesMessage {
    ShouldClose,
}

pub struct ViewRules {
    /// Sends messages to controller.
    tx: Sender<ViewRulesMessage>, 
    image: Image,
    close_button: Button,

    test_button: Button3
}

impl ViewRules {
    pub async fn new(tx: Sender<ViewRulesMessage>) -> Self {       
        let rules_texture = AssetLoader::get_texture("rules_view");
        let close_texture = AssetLoader::get_texture("close");

        Self {
            tx,
            image: Image::new((0., 0.), rules_texture, false, None),
            close_button: Button::new((680., 745.), close_texture, None),

            test_button: Button3::new((0., 0.), 0, "Okay", None),
        }
    }

    pub fn process_events(&mut self) {
        // Key presses.
        if is_key_released(KeyCode::Escape) {
            self.tx.send(ViewRulesMessage::ShouldClose).expect("Rules message send error.");
        }
        // Close button
        let event_opt = self.close_button.process_events();
        if event_opt == Some(ButtonEvent::Pushed(None)) {
            self.tx.send(ViewRulesMessage::ShouldClose).expect("Rules message send error.");
        }

        // Test button
        let test_opt = self.test_button.process_events();
    }

    pub fn draw(&mut self) {
        //self.image.draw();
        self.close_button.draw();
        
        self.test_button.draw();
    }
}