// ViewIntro
// The intro/title view.

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::message_sender::{Message, MessageSender};
use crate::asset_loader::AssetLoader;
use crate::sprite::*;

use crate::widget_button::Button;
use crate::widget_button::ButtonMode;
use crate::widget_message::WidgetMessage;
use crate::widget_slider::*;

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
    start_button: Button,
    exit_button: Button,

    slider: Slider,
    // Receive messages from view's controls.
    control_tx: Sender<WidgetMessage>,
    rx: Receiver<WidgetMessage>,
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
            start_button: Button::new(start_pos, start_tex, ButtonMode::Push, 1),
            exit_button: Button::new(exit_pos, exit_tex, ButtonMode::Push, 2),

            slider: Slider::new(
                (300., 600.), 
                300., 
                1., 
                0., 
                9., 
                0),
        }
    }

    pub fn prepare(&mut self) {
        self.start_button.tx = Some(self.control_tx.clone());
        self.exit_button.tx = Some(self.control_tx.clone());
        self.slider.tx = Some(self.control_tx.clone());
        self.slider.tick_divisions = 8;
        self.slider.snap_to_tick = true;
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.message_sender.send(Message::ShouldExit);
        }
        // Widgets. They may send messages.
        self.start_button.process_events();
        self.exit_button.process_events();
        self.slider.process_events();
    }

    pub fn check_messages(&mut self) {
        let received = self.rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                WidgetMessage::Pushed(id) => {
                    if id == self.start_button.id {
                        self.message_sender.send(Message::IntroEnded);
                    }
                    if id == self.exit_button.id {
                        self.message_sender.send(Message::ShouldExit);
                    }
                },
                WidgetMessage::Toggled(id) => {
                    println!("toggled id: {}", id);
                }
                WidgetMessage::ValueChanged(id, val) => {
                    println!("new value: {}", val);
                },
            }
        }
    }

    pub fn draw(&mut self) {
        clear_background(Color::from_rgba(81, 81, 81, 255));
        self.title.draw();
        self.start_button.draw();
        self.exit_button.draw();
        self.slider.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }
}