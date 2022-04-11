// ViewIntro
// The intro/title view.

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;

use crate::controller::Player;
use crate::controller::PlayerKind::*;

use crate::widget_button::{Button, ButtonMode};
use crate::widget_message::WidgetMessage;
use crate::widget_slider::*;

const TITLE_CORNER: (f32, f32) = (0., 0.);
const START_CORNER: (f32, f32) = (680., 745.);
const START_ID: usize = 0;
const EXIT_CORNER: (f32, f32) = (20., 745.);
const EXIT_ID: usize = 1;

const HUMAN_0_CORNER: (f32, f32) = (295., 340.);
const HUMAN_0_ID: usize = 2;
const MINIMAX_0_CORNER: (f32, f32) = (395., 340.);
const MINIMAX_0_ID: usize = 3;
const MONTE_0_CORNER: (f32, f32) = (515., 340.);
const MONTE_0_ID: usize = 4;
const DIFFICULTY_SLIDER_0_CORNER: (f32, f32) = (295., 410.);

const HUMAN_1_CORNER: (f32, f32) = (295., 535.);
const HUMAN_1_ID: usize = 5;
const MINIMAX_1_CORNER: (f32, f32) = (395., 535.);
const MINIMAX_1_ID: usize = 6;
const MONTE_1_CORNER: (f32, f32) = (515., 535.);
const MONTE_1_ID: usize = 7;

pub enum ViewIntroMessage {
    ShouldStart(Vec<Player>),
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
    slider_0: Slider,

    player_0: Player,
    player_1: Player,
}

impl ViewIntro {
    pub async fn new(tx: Sender<ViewIntroMessage>) -> Self {
        let (widget_tx, widget_rx) = mpsc::channel();
        
        Self {
            tx, widget_tx, widget_rx,
            background_tex: AssetLoader::get_texture("title"),
            buttons: HashMap::new(),

            slider_0: Slider::new(
                DIFFICULTY_SLIDER_0_CORNER, 
                360., 
                1., 
                0., 
                9., 
                0),
            player_0: Player { id: 0, kind: Human, search_depth: 3, search_rounds: 500 },
            player_1: Player { id: 1, kind: AIMinimax, search_depth: 3, search_rounds: 500 },
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

        // Player 0
        texture = AssetLoader::get_texture("human");
        button = Button::new(HUMAN_0_CORNER, texture, ButtonMode::Radio, HUMAN_0_ID);
        button.group_id = 0;
        self.buttons.insert(HUMAN_0_ID, button);

        texture = AssetLoader::get_texture("minimax");
        button = Button::new(MINIMAX_0_CORNER, texture, ButtonMode::Radio, MINIMAX_0_ID);
        button.group_id = 0;
        self.buttons.insert(MINIMAX_0_ID, button);

        texture = AssetLoader::get_texture("monte_carlo");
        button = Button::new(MONTE_0_CORNER, texture, ButtonMode::Radio, MONTE_0_ID);
        button.group_id = 0;
        self.buttons.insert(MONTE_0_ID, button);

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

        // Set common elements
        for button in self.buttons.values_mut() {
            //button.set_scale(0.5); // all files are _2x.
            button.color = LIGHTGRAY;
            button.selected_color = Some(Color::from_rgba(246, 194, 81, 255));
            button.tx = Some(self.widget_tx.clone());
        }

        self.slider_0.tx = Some(self.widget_tx.clone());
        
    }

    fn set_player_controls(&mut self, player: Player) {
        if player.id == 0 {
            let id = match player.kind {
                Human => HUMAN_0_ID,
                AIMinimax => MINIMAX_0_ID,
                AIMonteCarlo => MONTE_0_ID,
                AIRandom | AIMonteCarloTree => panic!(),
            };
            self.select_button(0, id);
    
            if player.kind == AIMinimax {
                self.slider_0.value = player.search_depth as f32;
                self.slider_0.max_value = 9.;
                self.slider_0.tick_divisions = 8;
                self.slider_0.snap_to_tick = true;
            }
            if player.kind == AIMonteCarlo {
                self.slider_0.value = player.search_rounds as f32;
                self.slider_0.max_value = 10_000.;
                self.slider_0.tick_divisions = 0;
                self.slider_0.snap_to_tick = false;
            }
        }

        if player.id == 1 {
            let id = match player.kind {
                Human => HUMAN_1_ID,
                AIMinimax => MINIMAX_1_ID,
                AIMonteCarlo => MONTE_1_ID,
                AIRandom | AIMonteCarloTree => panic!(),
            };
            self.select_button(1, id);
            //self.buttons.get_mut(&id).unwrap().is_selected = true;
    
            // if player.kind == AIMinimax {
            //     self.slider_1.value = player.search_depth as f32;
            //     self.slider_1.max_value = 9.;
            //     self.slider_1.tick_divisions = 8;
            //     self.slider_1.snap_to_tick = true;
            // }
            // if player.kind == AIMonteCarlo {
            //     self.slider_1.value = player.search_rounds as f32;
            //     self.slider_1.max_value = 1_000.;
            //     self.slider_1.tick_divisions = 0;
            //     self.slider_1.snap_to_tick = false;
            // }
        }
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
        self.slider_0.process_events();
    }

    pub fn check_messages(&mut self) {
        let received = self.widget_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                WidgetMessage::Pushed(id) => {
                    match id {
                        START_ID => {
                            let players = vec![self.player_0, self.player_1];
                            self.tx.send(ViewIntroMessage::ShouldStart(players)).expect("Intro message send error.");
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
                        HUMAN_0_ID => {
                            self.player_0.kind = Human;
                            self.set_player_controls(self.player_0);
                        },
                        MINIMAX_0_ID => {
                            self.player_0.kind = AIMinimax;
                            self.set_player_controls(self.player_0);
                        },
                        MONTE_0_ID => {
                            self.player_0.kind = AIMonteCarlo;
                            self.set_player_controls(self.player_0);
                        }
                        // HUMAN_1_ID | MINIMAX_1_ID | MONTE_1_ID => {
                        //     self.deselect_buttons(2, id);
                        // }
                        _ => {},
                    }
                }
                WidgetMessage::ValueChanged(id, val) => {
                    if id == self.slider_0.id {
                        if self.player_0.kind == AIMinimax {
                            self.player_0.search_depth = val as usize;
                        }
                        if self.player_0.kind == AIMonteCarlo {
                            self.player_0.search_rounds = val as usize;
                        }
                    }
                    println!("slider id: {}, new value: {}", id, val);
                },
            }
        }
    }

    /// Selects the given button and deselects all others in the group.
    fn select_button(&mut self, group_id: usize, button_id: usize) {
        for button in self.buttons.values_mut() {
            button.is_selected = button.group_id == group_id && button.id == button_id;
        }
    }

    pub fn draw(&mut self) {
        // Background
        clear_background(Color::from_rgba(222, 222, 193, 255));
        
        draw_texture(self.background_tex, TITLE_CORNER.0, TITLE_CORNER.1, WHITE);

        // let mut params = DrawTextureParams::default();
        // params.dest_size = Some(Vec2::new(800., 800.));
        // draw_texture_ex(self.background_tex, TITLE_CORNER.0, TITLE_CORNER.1, WHITE, params);

        // Widgets
        for button in self.buttons.values_mut() {
            button.draw();
        }
        self.slider_0.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }
}