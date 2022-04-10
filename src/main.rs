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
mod lerp;
mod piece;
mod sprite;
mod text;
mod view_game;
mod view_intro;
mod widget_button;
mod widget_message;
mod widget_slider;

use crate::controller::Controller;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Yokai"),
        window_width: 800,
        window_height: 800,
        high_dpi: false,
        //fullscreen: bool,
        //sample_count: 0,
        //window_resizable: bool,
        //icon: Option<Icon>,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    unsafe {
        let gl = get_internal_gl();
        println!("dpi_scale: {}", gl.quad_context.dpi_scale());
        
    }
    println!("dpi_scale: {}", screen_width() / 800.0);
    let mut controller = Controller::new().await;
    controller.prepare().await;
    controller.go().await;
}
