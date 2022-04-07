// ViewIntro
// The intro/title view.

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::button::Button;
use crate::button::ButtonMessage;
use crate::button::ButtonMode::*;

use crate::message_sender::{Message, MessageSender};
use crate::asset_loader::AssetLoader;
use crate::slider::*;
use crate::sprite::*;

const TITLE_CORNER: (f32, f32) = (0., 0.);
const START_CORNER: (f32, f32) = (350., 480.);
const EXIT_CORNER: (f32, f32) = (350., 620.);

/// Given the corner position, returns the center of the given texture.
fn texture_position(texture: &Texture2D, corner: (f32, f32)) -> (f32, f32) {
    (corner.0 + texture.width() / 2.0, corner.1 + texture.height() / 2.0)
}

pub struct ViewIntro {
    message_sender: MessageSender, // sends event messages to controller
    title: Sprite,
    start_button: Sprite,
    exit_button: Button,

    slider: Slider,
    // Receive messages from view's controls.
    control_tx: Sender<ButtonMessage>,
    rx: Receiver<ButtonMessage>,
}

impl ViewIntro {
    pub async fn new(tx: Sender<Message>) -> Self {
        // Create message passing transmitter for Buttons to use to communicate
        // with View as receiver.
        let (control_tx, rx) = mpsc::channel();

        let title_tex = AssetLoader::get_texture("title");
        let title_pos = texture_position(&title_tex, TITLE_CORNER);
        let start_tex = AssetLoader::get_texture("start");
        let start_pos = texture_position(&start_tex, START_CORNER);

        let exit_tex = AssetLoader::get_texture("exit");
        let exit_pos = texture_position(&exit_tex, EXIT_CORNER);
        
        Self {
            control_tx, rx,
            message_sender: MessageSender::new(tx, None),
            title: Sprite::new(title_pos, title_tex),
            start_button: Sprite::new(start_pos, start_tex),

            exit_button: Button::new(exit_pos, exit_tex, Push, 2),

            slider: Slider::new((500., 600.), 100., 33., 0., 100., 1),
        }
    }

    pub fn prepare(&mut self) {
        self.exit_button.tx = Some(self.control_tx.clone());
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.message_sender.send(Message::ShouldExit);
        }

        // Mouse position and buttons.
        let left_button_released = is_mouse_button_released(MouseButton::Left);

        let on_start_button = self.start_button.contains(mouse_position());
        if on_start_button && left_button_released {
            self.message_sender.send(Message::IntroEnded);
        }

        self.exit_button.process_mouse_events();

        let button_down = is_mouse_button_down(MouseButton::Left);
        self.slider.update(mouse_position(), button_down)
    }

    pub fn check_messages(&mut self) {
        let received = self.rx.try_recv();
        if received.is_ok() {
            dbg!("ok!");
            match received.unwrap() {
                ButtonMessage::Pushed(id) => {
                   if id == self.exit_button.id {
                       println!("exit");
                       self.message_sender.send(Message::ShouldExit);
                   }

                },
                ButtonMessage::Toggled(id) => {

                }
                _ => {},
            }
        }
    }

    pub fn draw(&mut self) {

        clear_background(Color::from_rgba(81, 81, 81, 255));
        //let text = String::from("Welcome to the game.");
        //draw_text(&text, 20.0, 300.0, 72.0, WHITE);

        self.title.draw();
        self.start_button.draw();
        self.exit_button.draw();

        self.slider.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }
}