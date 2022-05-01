/// ViewSettings
/// The player settings view.

use std::sync::mpsc::Sender;

use macroquad::prelude::WHITE;

use crate::asset_loader::AssetLoader;
use crate::controller::Player;
use crate::controller::PlayerKind::*;
use crate::view::button::Button;
use crate::view::button_bar::ButtonBar;
use crate::view::button_bar::ButtonBarOrientation;
use crate::view::image::Image;
use crate::view::label::Label;
use crate::view::slider::Slider;
use crate::view::slider::SliderEvent;

// Widget IDs
const HUMAN_ID: usize = 0;
const AI_ID: usize = 1;

pub enum ViewSettingsMessage {
    ShouldStart(Vec<Player>),
}

pub struct ViewSettings {
    /// Sends messages to controller.
    tx: Sender<ViewSettingsMessage>, 

    background_image: Image,
    okay_button: Button,

    button_bar_0: ButtonBar,
    slider_0: Slider,
    slider_0_label: Label,

    button_bar_1: ButtonBar,
    slider_1: Slider,
    slider_1_label: Label,

    players: Vec<Player>,
}

impl ViewSettings {
    pub async fn new(tx: Sender<ViewSettingsMessage>) -> Self {    
        let texture = AssetLoader::get_texture("view_settings"); 

        Self {
            tx,
            background_image: Image::new((200., 200.), texture, false, None),

            okay_button: Button::new((370., 510.), 0, "Okay", None),

            // Top player (1)
            button_bar_1: ButtonBar::new((379., 245.), ButtonBarOrientation::Horizontal, 25., true),
            slider_1: Slider::new((300., 300.), 200., 1., 1., 1., 0),
            slider_1_label: Label::new((400., 325.), true, "slider 1", 14, Some("Menlo")),

            // Botton player (0)
            button_bar_0: ButtonBar::new((379., 391.), ButtonBarOrientation::Horizontal, 25., true),
            slider_0: Slider::new((300., 445.), 200., 1., 1., 1., 1),
            slider_0_label: Label::new((400., 470.), true, "slider 0", 14, Some("Menlo")),
          
            players: Vec::new(),
        }
    }

    pub fn prepare(&mut self, players: Vec<Player>) {        
        self.players = players;
        let mut button;

        // Player 0
        button = Button::new((0., 0.), 1, "Human", Some(HUMAN_ID));
        self.button_bar_0.add_button(button);

        button = Button::new((0., 0.), 1, "AI", Some(AI_ID));
        self.button_bar_0.add_button(button);

        // Player 1
        button = Button::new((0., 0.), 1, "Human", Some(HUMAN_ID));
        self.button_bar_1.add_button(button);

        button = Button::new((0., 0.), 1, "AI", Some(AI_ID));
        self.button_bar_1.add_button(button);

        self.set_player_controls(0);
        self.set_player_controls(1);
    }

    /// Selects the given button and deselects all others in the group.
    fn select_button(&mut self, player: usize, button_id: usize) {
        if player == 0 {
            self.button_bar_0.select_only(button_id);
        }
        if player == 1 {
            self.button_bar_1.select_only(button_id);
        }
    }

    fn set_player_controls(&mut self, player_id: usize) {
        if player_id == 0 {
            let button_id = match self.players[0].kind {
                Human => HUMAN_ID,
                AI => AI_ID,
            };
            self.select_button(0, button_id);

            match self.players[0].kind {
                Human => {
                    self.slider_0.is_visible = false;
                    self.slider_0_label.draw_text.visible = false;
                },
                AI => {
                    self.slider_0.is_visible = true;
                    self.slider_0_label.draw_text.visible = true;
                    self.slider_0.value = self.players[0].search_depth as f32;
                    self.slider_0.max_value = 9.;
                    self.slider_0.tick_divisions = 7;
                    self.slider_0.snap_to_tick = true;
                },
            }
        }

        if player_id == 1 {
            let button_id = match self.players[1].kind {
                Human => HUMAN_ID,
                AI => AI_ID,
            };
            self.select_button(1, button_id);
    
            match self.players[1].kind {
                Human => {
                    self.slider_1.is_visible = false;
                    self.slider_1_label.draw_text.visible = false;
                },
                AI => {
                    self.slider_1.is_visible = true;
                    self.slider_1_label.draw_text.visible = true;
                    self.slider_1.value = self.players[1].search_depth as f32;
                    self.slider_1.max_value = 9.;
                    self.slider_1.tick_divisions = 7;
                    self.slider_1.snap_to_tick = true;
                },
            }
        }
    }

    pub fn process_events(&mut self) {

        if self.okay_button.process_events().is_some() {
            self.tx.send(ViewSettingsMessage::ShouldStart(self.players.clone()))
            .expect("Intro message send error.");
        }

        // ButtonBar 0
        if let Some(button_id) = self.button_bar_0.process_events() {
            self.button_bar_0.select_only(button_id);
            self.players[0].kind = if button_id == HUMAN_ID { Human } else { AI };
            self.set_player_controls(0);
        }

        // ButtonBar 1
        if let Some(button_id) = self.button_bar_1.process_events() {
            self.button_bar_1.select_only(button_id);
            self.players[1].kind = if button_id == HUMAN_ID { Human } else { AI };
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

        self.okay_button.draw();

        self.button_bar_0.draw();
        self.button_bar_1.draw();

        self.slider_0.draw();

        // Use live values here so user sees the values change when dragging.
        let text_0 = match self.players[0].kind {
            Human => "".to_string(),
            AI => format!("{} move look-ahead", self.slider_0.nearest_snap_value()),
        };
        self.slider_0_label.set_text(text_0);
        self.slider_0_label.draw();

        self.slider_1.draw();

        // Use live values here so user sees the values change when dragging.
        let text_1 = match self.players[1].kind {
            Human => "".to_string(),
            AI => format!("{} move look-ahead", self.slider_1.nearest_snap_value() as usize),
        };
        self.slider_1_label.set_text(text_1);
        self.slider_1_label.draw();
    }
}