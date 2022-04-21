// Main

use macroquad::prelude::*;

mod ai;
mod ai_minimax;
mod ai_monte_carlo;
mod ai_monte_carlo_tree;
mod ai_random;
mod ai_sender;
mod asset_loader;
mod controller;
mod game;
mod image;
mod lerp;
mod piece;
mod sprite;
mod text;
mod view;
mod widget_button_bar;
// mod widget_components;
// mod view_game;
// mod view_intro;
// mod view_settings;
// mod view_rules;
mod widget_button;
mod widget_container;
mod widget_slider;

use crate::controller::Controller;
//use crate::view::View;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Yōkaï No Mori"),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        //fullscreen: bool,
        //sample_count: 0,
        //window_resizable: bool,
        //icon: Option<Icon>,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    println!("dpi_scale: {}", crate::view::dpi_scale());
    let mut controller = Controller::new().await;
    controller.prepare().await;
    controller.go().await;
}
