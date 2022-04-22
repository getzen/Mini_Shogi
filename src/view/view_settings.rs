// ViewIntro
// The intro/title view.

use std::collections::HashMap;
use std::sync::mpsc::Sender;

use macroquad::prelude::Color;
use macroquad::prelude::{BLACK, LIGHTGRAY};


use crate::asset_loader::AssetLoader;
use crate::view::image::Image;

use crate::controller::Player;
use crate::controller::PlayerKind::*;
use crate::view::button::Button;
use crate::view::button::ButtonEvent;
use crate::view::label::Label;
use crate::view::slider::Slider;
use crate::view::slider::SliderEvent;

// Widget IDs
const OKAY_ID: usize = 0;
const HUMAN_0_ID: usize = 3;
const AI_0_ID: usize = 4;
const HUMAN_1_ID: usize = 6;
const AI_1_ID: usize = 7;

pub enum ViewSettingsMessage {
    ShouldStart(Vec<Player>),
}

pub struct ViewSettings {
    /// Sends messages to controller.
    tx: Sender<ViewSettingsMessage>, 

    background_image: Image,
    buttons: HashMap<usize, Button>,
    slider_0: Slider,
    slider_0_text: Label,
    slider_1: Slider,
    slider_1_text: Label,
    players: Vec<Player>,
}

impl ViewSettings {
    pub async fn new(tx: Sender<ViewSettingsMessage>) -> Self {    
        let texture = AssetLoader::get_texture("view_settings"); 
        Self {
            tx,
            background_image: Image::new((200., 100.), texture, false, None),
            buttons: HashMap::new(),

            slider_0: Slider::new((300., 200.), 200., 1., 1., 1., 0),
            slider_0_text: Label::new((400., 242.), true, "hello".to_string(), 18, Some("Menlo")),

            slider_1: Slider::new((300., 345.), 200., 1., 1., 1., 1),
            slider_1_text: Label::new((400., 387.), true, "world".to_string(), 18, Some("Menlo")),

            players: Vec::new(),
        }
    }

    pub fn prepare(&mut self, players: Vec<Player>) {
        self.players = players;
        let mut texture;
        let mut button;

        texture = AssetLoader::get_texture("okay");
        button = Button::new((365., 410.), texture, Some(OKAY_ID));
        self.buttons.insert(OKAY_ID, button);

        // Player 0
        texture = AssetLoader::get_texture("button_human");
        button = Button::new((385., 140.), texture, Some(HUMAN_0_ID));
        //button.group_id = 0;
        self.buttons.insert(HUMAN_0_ID, button);

        texture = AssetLoader::get_texture("button_ai");
        button = Button::new((480., 140.), texture, Some(AI_0_ID));
        //button.group_id = 0;
        self.buttons.insert(AI_0_ID, button);

        // Player 1
        texture = AssetLoader::get_texture("button_human");
        button = Button::new((400., 290.), texture, Some(HUMAN_1_ID));
        //button.group_id = 1;
        self.buttons.insert(HUMAN_1_ID, button);

        texture = AssetLoader::get_texture("button_ai");
        button = Button::new((495., 290.), texture, Some(AI_1_ID));
        //button.group_id = 1;
        self.buttons.insert(AI_1_ID, button);

        // Set common elements
        for button in self.buttons.values_mut() {
            button.color = LIGHTGRAY;
            button.selected_color = Some(Color::from_rgba(246, 194, 81, 255));
        }

        self.slider_0_text.set_color(BLACK);
        self.slider_1_text.set_color(BLACK);

        self.set_player_controls(0);
        self.set_player_controls(1);
    }

    /// Selects the given button and deselects all others in the group.
    fn select_button(&mut self, group_id: usize, button_id: usize) {
        for button in self.buttons.values_mut() {
            if button.group_id != Some(group_id) { continue; }
            button.selected = button.id.unwrap() == button_id;
        }
    }

    fn set_player_controls(&mut self, player_id: usize) {
        if player_id == 0 {
            let button_id = match self.players[0].kind {
                Human => HUMAN_0_ID,
                AI => AI_0_ID,
            };
            self.select_button(0, button_id);

            match self.players[0].kind {
                Human => {
                    self.slider_0.is_visible = false;
                    self.slider_0_text.draw_text.visible = false;
                },
                AI => {
                    self.slider_0.is_visible = true;
                    self.slider_0_text.draw_text.visible = true;
                    self.slider_0.value = self.players[0].search_depth as f32;
                    self.slider_0.max_value = 9.;
                    self.slider_0.tick_divisions = 7;
                    self.slider_0.snap_to_tick = true;
                },
            }
        }

        if player_id == 1 {
            let button_id = match self.players[1].kind {
                Human => HUMAN_1_ID,
                AI => AI_1_ID,
            };
            self.select_button(1, button_id);
    
            match self.players[1].kind {
                Human => {
                    self.slider_1.is_visible = false;
                    self.slider_1_text.draw_text.visible = false;
                },
                AI => {
                    self.slider_1.is_visible = true;
                    self.slider_1_text.draw_text.visible = true;
                    self.slider_1.value = self.players[1].search_depth as f32;
                    self.slider_1.max_value = 9.;
                    self.slider_1.tick_divisions = 7;
                    self.slider_1.snap_to_tick = true;
                },
            }
        }
    }

    pub fn process_events(&mut self) {
        let mut p_0_dirty = false;
        let mut p_1_dirty = false;

        // Buttons. They return Option<ButtonEvent>.
        for button in self.buttons.values_mut() {
            let event_opt = button.process_events();
            if event_opt.is_none() { continue }

            match event_opt.unwrap() {
                ButtonEvent::Pushed(id) => {
                    // Start and Exit buttons
                    match id {
                        Some(OKAY_ID) => {
                            self.tx.send(
                                ViewSettingsMessage::ShouldStart(self.players.clone()))
                                .expect("Intro message send error.");
                        },
                        Some(HUMAN_0_ID) => {
                            self.players[0].kind = Human;
                            p_0_dirty = true;
                        }
                        Some(AI_0_ID) => {
                            self.players[0].kind = AI;
                            p_0_dirty = true;
                        }
                        Some(HUMAN_1_ID) => {
                            self.players[1].kind = Human;
                            p_1_dirty = true;
                        }
                        Some(AI_1_ID) => {
                            self.players[1].kind = AI;
                            p_1_dirty = true;
                        }
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        if p_0_dirty {
            self.set_player_controls(0);
        }
        if p_1_dirty {
            self.set_player_controls(1);
        }
                
        // Slider 0. Sliders return Option<SliderEvent>.
        if let Some(event) = self.slider_0.process_events() {
            match event {
                SliderEvent::Hovering(_id) => {},
                SliderEvent::ValueChanged(_id, val) => {
                    if self.players[0].kind == AI {
                        self.players[0].search_depth = val as usize;
                    }
                },
            }
        }
        // Slider 1
        if let Some(event) = self.slider_1.process_events() {
            match event {
                SliderEvent::Hovering(_id) => {},
                SliderEvent::ValueChanged(_id, val) => {
                    if self.players[1].kind == AI {
                        self.players[1].search_depth = val as usize;
                    }
                },
            }
        }
    }

    pub fn draw(&mut self) {        
        self.background_image.draw();

        // Widgets
        for button in self.buttons.values_mut() {
            button.draw();
        }

        self.slider_0.draw();

        // Use live values here so user sees the values change when dragging.
        let text_0 = match self.players[0].kind {
            Human => "".to_string(),
            AI => format!("{} move look-ahead", self.slider_0.nearest_snap_value()),
        };
        self.slider_0_text.set_text(text_0);
        self.slider_0_text.draw();

        self.slider_1.draw();

        // Use live values here so user sees the values change when dragging.
        let text_1 = match self.players[1].kind {
            Human => "".to_string(),
            AI => format!("{} move look-ahead", self.slider_1.nearest_snap_value() as usize),
        };
        self.slider_1_text.set_text(text_1);
        self.slider_1_text.draw();
    }
}