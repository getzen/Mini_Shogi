// ViewIntro
// The intro/title view.

use std::collections::HashMap;
use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;

use crate::controller::Player;
use crate::controller::PlayerKind::*;
use crate::text::Text;
use crate::View;
use crate::widget_button::*;
use crate::widget_slider::*;

// Widget IDs
const START_ID: usize = 0;
const EXIT_ID: usize = 1;
const RULES_ID: usize = 2;
const HUMAN_0_ID: usize = 3;
const MINIMAX_0_ID: usize = 4;
const MONTE_0_ID: usize = 5;
const HUMAN_1_ID: usize = 6;
const MINIMAX_1_ID: usize = 7;
const MONTE_1_ID: usize = 8;

pub enum ViewIntroMessage {
    ShouldStart(Vec<Player>),
    ShowRules,
    ShouldExit,
}

pub struct ViewIntro {
    /// Sends messages to controller.
    tx: Sender<ViewIntroMessage>, 

    background_tex: Texture2D,
    buttons: HashMap<usize, Button>,
    slider_0: Slider,
    slider_0_text: Text,
    slider_1: Slider,
    slider_1_text: Text,

    player_0: Player,
    player_1: Player,
}

impl ViewIntro {
    pub async fn new(tx: Sender<ViewIntroMessage>) -> Self {        
        Self {
            tx,
            background_tex: AssetLoader::get_texture("title"),
            buttons: HashMap::new(),

            slider_0: Slider::new((295., 410.), 360., 1., 1., 1., 0),
            slider_0_text: Text::new((445., 447.), "hello".to_string(), 18, Some("Menlo")).await,

            slider_1: Slider::new((295., 600.), 360., 1., 1., 1., 1),
            slider_1_text: Text::new((445., 637.), "world".to_string(), 18, Some("Menlo")).await,

            player_0: Player { id: 0, kind: Human, search_depth: 3, search_rounds: 500 },
            player_1: Player { id: 1, kind: AIMinimax, search_depth: 3, search_rounds: 500 },
        }
    }

    pub fn prepare(&mut self) {
        let mut texture;
        let mut button;

        texture = AssetLoader::get_texture("start");
        button = Button::new((680., 745.), texture, ButtonMode::Push, START_ID);
        self.buttons.insert(START_ID, button);

        texture = AssetLoader::get_texture("exit");
        button = Button::new((20., 745.), texture, ButtonMode::Push, EXIT_ID);
        self.buttons.insert(EXIT_ID, button);

        texture = AssetLoader::get_texture("rules");
        button = Button::new((350., 745.), texture, ButtonMode::Push, RULES_ID);
        self.buttons.insert(RULES_ID, button);

        // Player 0
        texture = AssetLoader::get_texture("human");
        button = Button::new((295., 340.), texture, ButtonMode::Radio, HUMAN_0_ID);
        button.group_id = 0;
        self.buttons.insert(HUMAN_0_ID, button);

        texture = AssetLoader::get_texture("minimax");
        button = Button::new((395., 340.), texture, ButtonMode::Radio, MINIMAX_0_ID);
        button.group_id = 0;
        self.buttons.insert(MINIMAX_0_ID, button);

        texture = AssetLoader::get_texture("monte_carlo");
        button = Button::new((515., 340.), texture, ButtonMode::Radio, MONTE_0_ID);
        button.group_id = 0;
        self.buttons.insert(MONTE_0_ID, button);

        // Player 1
        texture = AssetLoader::get_texture("human");
        button = Button::new((295., 535.), texture, ButtonMode::Radio, HUMAN_1_ID);
        button.group_id = 1;
        self.buttons.insert(HUMAN_1_ID, button);

        texture = AssetLoader::get_texture("minimax");
        button = Button::new((395., 535.), texture, ButtonMode::Radio, MINIMAX_1_ID);
        button.group_id = 1;
        self.buttons.insert(MINIMAX_1_ID, button);

        texture = AssetLoader::get_texture("monte_carlo");
        button = Button::new((515., 535.), texture, ButtonMode::Radio, MONTE_1_ID);
        button.group_id = 1;
        self.buttons.insert(MONTE_1_ID, button);

        // Set common elements
        for button in self.buttons.values_mut() {
            button.color = LIGHTGRAY;
            button.selected_color = Some(Color::from_rgba(246, 194, 81, 255));
        }

        self.slider_0_text.set_color(BLACK);
        self.slider_0_text.centered = true;

        self.slider_1_text.set_color(BLACK);
        self.slider_1_text.centered = true;

        self.set_player_controls(0);
        self.set_player_controls(1);
    }

    /// Selects the given button and deselects all others in the group.
    fn select_button(&mut self, group_id: usize, button_id: usize) {
        for button in self.buttons.values_mut() {
            if button.group_id != group_id { continue; }
            button.is_selected = button.id == button_id;
        }
    }

    fn set_player_controls(&mut self, player_id: usize) {
        if player_id == 0 {
            let button_id = match self.player_0.kind {
                Human => HUMAN_0_ID,
                AIMinimax => MINIMAX_0_ID,
                AIMonteCarlo => MONTE_0_ID,
                AIRandom | AIMonteCarloTree => panic!(),
            };
            self.select_button(0, button_id);

            match self.player_0.kind {
                Human => {
                    self.slider_0.is_visible = false;
                    self.slider_0_text.is_visible = false;
                },
                AIMinimax => {
                    self.slider_0.is_visible = true;
                    self.slider_0_text.is_visible = true;
                    self.slider_0.value = self.player_0.search_depth as f32;
                    self.slider_0.max_value = 9.;
                    self.slider_0.tick_divisions = 7;
                    self.slider_0.snap_to_tick = true;
                },
                AIMonteCarlo => {
                    self.slider_0.is_visible = true;
                    self.slider_0_text.is_visible = true;
                    self.slider_0.value = self.player_0.search_rounds as f32;
                    self.slider_0.max_value = 1_000.;
                    self.slider_0.tick_divisions = 0;
                    self.slider_0.snap_to_tick = false;
                },
                _ => {},
            }
        }

        if player_id == 1 {
            let button_id = match self.player_1.kind {
                Human => HUMAN_1_ID,
                AIMinimax => MINIMAX_1_ID,
                AIMonteCarlo => MONTE_1_ID,
                AIRandom | AIMonteCarloTree => panic!(),
            };
            self.select_button(1, button_id);
    
            match self.player_1.kind {
                Human => {
                    self.slider_1.is_visible = false;
                    self.slider_1_text.is_visible = false;
                },
                AIMinimax => {
                    self.slider_1.is_visible = true;
                    self.slider_1_text.is_visible = true;
                    self.slider_1.value = self.player_1.search_depth as f32;
                    self.slider_1.max_value = 9.;
                    self.slider_1.tick_divisions = 7;
                    self.slider_1.snap_to_tick = true;
                },
                AIMonteCarlo => {
                    self.slider_1.is_visible = true;
                    self.slider_1_text.is_visible = true;
                    self.slider_1.value = self.player_1.search_rounds as f32;
                    self.slider_1.max_value = 1_000.;
                    self.slider_1.tick_divisions = 0;
                    self.slider_1.snap_to_tick = false;
                },
                _ => {},
            }
        }
    }

    pub fn process_events(&mut self) {
        // Key presses.
        if is_key_released(KeyCode::Escape) {
            self.tx.send(ViewIntroMessage::ShouldExit).expect("Intro message send error.");
        }

        // Track which player controls to update, if any.
        let mut player = None;

        // Buttons. They return Option<ButtonEvent>.
        for button in self.buttons.values_mut() {
            if let Some(event) = button.process_events() {
                match event {
                    ButtonEvent::Hovering(_id) => {},
                    ButtonEvent::Pushed(id) => {
                        // Start and Exit buttons
                        match id {
                            START_ID => {
                                let players = vec![self.player_0, self.player_1];
                                self.tx.send(ViewIntroMessage::ShouldStart(players)).expect("Intro message send error.");
                            },
                            EXIT_ID => {
                                self.tx.send(ViewIntroMessage::ShouldExit).expect("Intro message send error.");
                            }
                            RULES_ID => {
                                self.tx.send(ViewIntroMessage::ShowRules).expect("Intro message send error.");
                            }
                            _ => {},
                        }
                    },
                    ButtonEvent::Toggled(_id) => {},
                    ButtonEvent::Selected(id) => {
                        
                        let kind = match id {
                            HUMAN_0_ID => {
                                player = Some(0);
                                Human
                            },
                            MINIMAX_0_ID => {
                                player = Some(0);
                                AIMinimax
                            },
                            MONTE_0_ID => {
                                player = Some(0);
                                AIMonteCarlo
                            },
                            HUMAN_1_ID => {
                                player = Some(1);
                                Human
                            },
                            MINIMAX_1_ID => {
                                player = Some(1);
                                AIMinimax
                            },
                            MONTE_1_ID => {
                                player = Some(1);
                                AIMonteCarlo
                            }
                            _ => {panic!()},
                        };
                        if player.is_some() && player.unwrap() == 0 {
                            self.player_0.kind = kind;
                        } else if player.is_some() && player.unwrap() == 1 {
                            self.player_1.kind = kind;
                        }
                    },
                }
            }
        }
        if let Some(player) = player {
            self.set_player_controls(player);
        }
                
        // Slider 0. Sliders return Option<SliderEvent>.
        if let Some(event) = self.slider_0.process_events() {
            match event {
                SliderEvent::Hovering(_id) => {},
                SliderEvent::ValueChanged(_id, val) => {
                    if self.player_0.kind == AIMinimax {
                        self.player_0.search_depth = val as usize;
                    }
                    if self.player_0.kind == AIMonteCarlo {
                        self.player_0.search_rounds = val as usize;
                    }
                },
            }
        }
        // Slider 1
        if let Some(event) = self.slider_1.process_events() {
            match event {
                SliderEvent::Hovering(_id) => {},
                SliderEvent::ValueChanged(_id, val) => {
                    if self.player_1.kind == AIMinimax {
                        self.player_1.search_depth = val as usize;
                    }
                    if self.player_1.kind == AIMonteCarlo {
                        self.player_1.search_rounds = val as usize;
                    }
                },
            }
        }
    }

    pub fn draw(&mut self) {
        // Background
        clear_background(Color::from_rgba(222, 222, 193, 255));
        
        // Draw background. Could use a Sprite, but keeping this 
        let mut params = DrawTextureParams::default();
        let size_x = self.background_tex.width() * View::adj_scale();
        let size_y = self.background_tex.height() * View::adj_scale();
        params.dest_size = Some(Vec2::new(size_x, size_y));
        draw_texture_ex(self.background_tex, 0., 0., WHITE, params);

        // Widgets
        for button in self.buttons.values_mut() {
            button.draw();
        }

        self.slider_0.draw();
        // Use live values here so user sees the values change when dragging.
        self.slider_0_text.text = match self.player_0.kind {
            Human => "".to_string(),
            AIMinimax => format!("{} move look-ahead", self.slider_0.nearest_snap_value()),
            //AIMinimax => format!("{} move look-ahead", self.slider_0.value as usize),
            AIMonteCarlo => format!("{} play outs", self.slider_0.value as usize),
            _ => "".to_string(),
        };
        self.slider_0_text.draw();

        self.slider_1.draw();
        // Use live values here so user sees the values change when dragging.
        self.slider_1_text.text = match self.player_1.kind {
            Human => "".to_string(),
            AIMinimax => format!("{} move look-ahead", self.slider_1.nearest_snap_value() as usize),
            AIMonteCarlo => format!("{} play outs", self.slider_1.value as usize),
            _ => "".to_string(),
        };
        self.slider_1_text.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }
}