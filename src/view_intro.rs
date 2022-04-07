// ViewIntro
// The intro/title view.

use std::collections::HashMap;
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
const START_CORNER: (f32, f32) = (680., 745.);
const START_ID: usize = 0;
const EXIT_CORNER: (f32, f32) = (20., 745.);
const EXIT_ID: usize = 1;

const HUMAN_1_CORNER: (f32, f32) = (295., 340.);
const HUMAN_1_ID: usize = 2;
const MINIMAX_1_CORNER: (f32, f32) = (395., 340.);
const MINIMAX_1_ID: usize = 3;
const MONTE_1_CORNER: (f32, f32) = (515., 340.);
const MONTE_1_ID: usize = 4;

const HUMAN_2_CORNER: (f32, f32) = (295., 535.);
const HUMAN_2_ID: usize = 5;
const MINIMAX_2_CORNER: (f32, f32) = (395., 535.);
const MINIMAX_2_ID: usize = 6;
const MONTE_2_CORNER: (f32, f32) = (515., 535.);
const MONTE_2_ID: usize = 7;

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

    background_tex: Texture2D,
    buttons: HashMap<usize, Button>,
    slider: Slider,
}

impl ViewIntro {
    pub async fn new(tx: Sender<ViewIntroMessage>) -> Self {
        let (widget_tx, widget_rx) = mpsc::channel();
        
        Self {
            tx, widget_tx, widget_rx,
            background_tex: AssetLoader::get_texture("title"),
            buttons: HashMap::new(),

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
        let mut texture;
        let mut button;

        texture = AssetLoader::get_texture("start");
        button = Button::new(START_CORNER, texture, ButtonMode::Push, START_ID);
        self.buttons.insert(START_ID, button);

        texture = AssetLoader::get_texture("exit");
        button = Button::new(EXIT_CORNER, texture, ButtonMode::Push, EXIT_ID);
        self.buttons.insert(EXIT_ID, button);

        // Player 1
        texture = AssetLoader::get_texture("human");
        button = Button::new(HUMAN_1_CORNER, texture, ButtonMode::Radio, HUMAN_1_ID);
        button.group_id = 1;
        button.is_selected = true;
        self.buttons.insert(HUMAN_1_ID, button);

        texture = AssetLoader::get_texture("minimax");
        button = Button::new(MINIMAX_1_CORNER, texture, ButtonMode::Radio, MINIMAX_1_ID);
        button.group_id = 1;
        self.buttons.insert(MINIMAX_1_ID, button);

        texture = AssetLoader::get_texture("monte_carlo");
        button = Button::new(MONTE_1_CORNER, texture, ButtonMode::Radio, MONTE_1_ID);
        button.group_id = 1;
        self.buttons.insert(MONTE_1_ID, button);

        // Player 2
        texture = AssetLoader::get_texture("human");
        button = Button::new(HUMAN_2_CORNER, texture, ButtonMode::Radio, HUMAN_2_ID);
        button.group_id = 2;
        button.is_selected = true;
        self.buttons.insert(HUMAN_2_ID, button);

        texture = AssetLoader::get_texture("minimax");
        button = Button::new(MINIMAX_2_CORNER, texture, ButtonMode::Radio, MINIMAX_2_ID);
        button.group_id = 2;
        self.buttons.insert(MINIMAX_2_ID, button);

        texture = AssetLoader::get_texture("monte_carlo");
        button = Button::new(MONTE_2_CORNER, texture, ButtonMode::Radio, MONTE_2_ID);
        button.group_id = 2;
        self.buttons.insert(MONTE_2_ID, button);

        // Set common elements
        for button in self.buttons.values_mut() {
            button.color = LIGHTGRAY;
            button.selected_color = Some(Color::from_rgba(246, 194, 81, 255));
            button.tx = Some(self.widget_tx.clone());
        }

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
        for button in self.buttons.values_mut() {
            button.process_events();
        }
        self.slider.process_events();
    }

    pub fn check_messages(&mut self) {
        let received = self.widget_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                WidgetMessage::Pushed(id) => {
                    match id {
                        START_ID => {
                            self.tx.send(ViewIntroMessage::ShouldStart).expect("Intro message send error.");
                        },
                        EXIT_ID => {
                            self.tx.send(ViewIntroMessage::ShouldExit).expect("Intro message send error.");
                        }
                        _ => {},
                    }
                },
                WidgetMessage::Toggled(id) => {
                    println!("toggled id: {}", id);
                }
                WidgetMessage::Selected(id) => { // radio-style groupings
                    match id {
                        HUMAN_1_ID | MINIMAX_1_ID | MONTE_1_ID => {
                            self.deselect_buttons(1, id);
                        }
                        HUMAN_2_ID | MINIMAX_2_ID | MONTE_2_ID => {
                            self.deselect_buttons(2, id);
                        }
                        _ => {},
                    }
                }
                WidgetMessage::ValueChanged(id, val) => {
                    println!("slider id: {}, new value: {}", id, val);
                },
            }
        }
    }

    /// Deselects all the buttons with given group_id. Ignores the button with
    /// the except_id.
    fn deselect_buttons(&mut self, group_id: usize, except_id: usize) {
        for button in self.buttons.values_mut() {
            if button.group_id == group_id && button.id != except_id {
                button.is_selected = false;
            }
        }
    }

    pub fn draw(&mut self) {
        // Background
        clear_background(Color::from_rgba(222, 222, 193, 255));
        draw_texture(self.background_tex, TITLE_CORNER.0, TITLE_CORNER.1, WHITE);
        // Widgets
        for button in self.buttons.values_mut() {
            button.draw();
        }
        self.slider.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }
}