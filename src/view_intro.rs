// ViewIntro
// The intro/title view.

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::message_sender::{Message, MessageSender};
use crate::asset_loader::AssetLoader;
use crate::sprite::*;

const TITLE_CORNER: (f32, f32) = (0., 0.);
const START_CORNER: (f32, f32) = (350., 480.);
const EXIT_CORNER: (f32, f32) = (350., 620.);

fn texture_position(texture: &Texture2D, corner: (f32, f32)) -> (f32, f32) {
    (corner.0 + texture.width() / 2.0, corner.1 + texture.height() / 2.0)
}

pub struct ViewIntro {
    message_sender: MessageSender, // sends event messages to controller
    title: Sprite,
    start_button: Sprite,
    exit_button: Sprite,
}

impl ViewIntro {
    pub async fn new(tx: Sender<Message>) -> Self {
        let title_tex = AssetLoader::get_texture("title");
        let title_pos = texture_position(&title_tex, TITLE_CORNER);
        let start_tex = AssetLoader::get_texture("start");
        let start_pos = texture_position(&start_tex, START_CORNER);
        let exit_tex = AssetLoader::get_texture("exit");
        let exit_pos = texture_position(&exit_tex, EXIT_CORNER);
        Self {
            message_sender: MessageSender::new(tx, None),
            title: Sprite::new(title_tex, title_pos),
            start_button: Sprite::new(start_tex, start_pos),
            exit_button: Sprite::new(exit_tex, exit_pos),
        }
    }

    pub fn prepare(&mut self) {
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.message_sender.send(Message::ShouldExit);
        }

        // Mouse position and buttons.
        let left_button_released = is_mouse_button_released(MouseButton::Left);

        let on_start_button = self.start_button.highlight_on_mouse_over();
        if on_start_button && left_button_released {
            self.message_sender.send(Message::IntroEnded);
        }
        let on_exit_button = self.exit_button.highlight_on_mouse_over();
        if on_exit_button && left_button_released {
            self.message_sender.send(Message::ShouldExit);
        }
    }

    pub fn draw(&mut self) {

        clear_background(Color::from_rgba(81, 81, 81, 255));
        //let text = String::from("Welcome to the game.");
        //draw_text(&text, 20.0, 300.0, 72.0, WHITE);

        self.title.draw();
        self.start_button.draw();
        self.exit_button.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }
}