// ViewIntro
// The intro/title view.

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;

use crate::widget_button::Button;
use crate::widget_button::ButtonMode;
use crate::widget_message::WidgetMessage;
use crate::widget_slider::*;

const TITLE_CORNER: (f32, f32) = (0., 0.);
const START_CORNER: (f32, f32) = (675., 730.);
const EXIT_CORNER: (f32, f32) = (25., 730.);

pub enum ViewIntroMessage {
    ShouldStart,
    ShouldExit,
}

pub struct ViewIntro {
    // Sends messages to controller.
    tx: Sender<ViewIntroMessage>, 
    // Receive messages from view's widgets.
    widget_tx: Sender<WidgetMessage>, // copy given to widgets
    widget_rx: Receiver<WidgetMessage>,

    title_tex: Texture2D,
    start_button: Button,
    exit_button: Button,
    slider: Slider,
}

impl ViewIntro {
    pub async fn new(tx: Sender<ViewIntroMessage>) -> Self {
        let (widget_tx, widget_rx) = mpsc::channel();

        let start_tex = AssetLoader::get_texture("start");
        let exit_tex = AssetLoader::get_texture("exit");
        
        Self {
            tx, widget_tx, widget_rx,
            title_tex: AssetLoader::get_texture("title"),
            start_button: Button::new(START_CORNER, start_tex, ButtonMode::Push, 1),
            exit_button: Button::new(EXIT_CORNER, exit_tex, ButtonMode::Push, 2),

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
        self.start_button.tx = Some(self.widget_tx.clone());
        self.exit_button.tx = Some(self.widget_tx.clone());
        self.slider.tx = Some(self.widget_tx.clone());
        self.slider.tick_divisions = 8;
        self.slider.snap_to_tick = true;
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.tx.send(ViewIntroMessage::ShouldExit).expect("Intro message send error.");
        }
        // Widgets. They may send messages.
        self.start_button.process_events();
        self.exit_button.process_events();
        self.slider.process_events();
    }

    pub fn check_messages(&mut self) {
        let received = self.widget_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                WidgetMessage::Pushed(id) => {
                    if id == self.start_button.id {
                        self.tx.send(ViewIntroMessage::ShouldStart).expect("Intro message send error.");
                    }
                    if id == self.exit_button.id {
                        self.tx.send(ViewIntroMessage::ShouldExit).expect("Intro message send error.");
                    }
                },
                WidgetMessage::Toggled(id) => {
                    println!("toggled id: {}", id);
                }
                WidgetMessage::ValueChanged(id, val) => {
                    println!("slider id: {}, new value: {}", id, val);
                },
            }
        }
    }

    pub fn draw(&mut self) {
        // Background
        clear_background(Color::from_rgba(81, 81, 81, 255));
        draw_texture(self.title_tex, TITLE_CORNER.0, TITLE_CORNER.1, WHITE);
        // Widgets
        self.start_button.draw();
        self.exit_button.draw();
        self.slider.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }
}