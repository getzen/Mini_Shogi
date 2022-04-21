// ViewRules

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::view::image::Image;
use crate::widget_button::*;

pub enum ViewRulesMessage {
    ShouldClose,
}

pub struct ViewRules {
    /// Sends messages to controller.
    tx: Sender<ViewRulesMessage>, 
    image: Image,
    close_button: Button,
}

impl ViewRules {
    pub async fn new(tx: Sender<ViewRulesMessage>) -> Self {       
        let close_texture = AssetLoader::get_texture("close");
        let mut button = Button::new((680., 745.), close_texture, ButtonMode::Push, 0);
        button.color = LIGHTGRAY;
        button.selected_color = Some(Color::from_rgba(246, 194, 81, 255));

        let rules_texture = AssetLoader::get_texture("rules_view"); 

        Self {
            tx,
            image: Image::new((0., 0.), rules_texture, false, None),
            close_button: button,
        }
    }

    pub fn process_events(&mut self) {
        // Key presses.
        if is_key_released(KeyCode::Escape) {
            self.tx.send(ViewRulesMessage::ShouldClose).expect("Rules message send error.");
        }
        // Close button
        let event_opt = self.close_button.process_events();
        if event_opt == Some(ButtonEvent::Pushed(0)) {
            self.tx.send(ViewRulesMessage::ShouldClose).expect("Rules message send error.");
        }
    }

    pub fn draw(&mut self) {
        self.image.draw();
        self.close_button.draw();
    }
}