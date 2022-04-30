/// Asset Loader
///
/// *** See view.rs to set image scale factor *** 

use macroquad::audio::load_sound_from_bytes;
use macroquad::audio::Sound;
use macroquad::prelude::Font;
use macroquad::prelude::load_ttf_font_from_bytes;
use macroquad::texture::Texture2D;

pub struct AssetLoader {}

impl AssetLoader {

    /// Returns the texture associated with the given name.
    // pub fn get_texture(name: &str) -> Texture2D {
        // match name {
        //     // view_intro
        //     "title" => Texture2D::from_file_with_format(include_bytes!("../assets/title.png"),None,),
        //     "start" => Texture2D::from_file_with_format(include_bytes!("../assets/start.png"),None,),
        //     "rules" => Texture2D::from_file_with_format(include_bytes!("../assets/rules.png"),None,),
        //     "exit" => Texture2D::from_file_with_format(include_bytes!("../assets/exit.png"),None,),
        //     "human" => Texture2D::from_file_with_format(include_bytes!("../assets/human.png"),None,),
        //     "minimax" => Texture2D::from_file_with_format(include_bytes!("../assets/minimax.png"),None,),
        //     "monte_carlo" => Texture2D::from_file_with_format(include_bytes!("../assets/monte_carlo.png"),None,),
        //     // view board
        //     "square" => Texture2D::from_file_with_format(include_bytes!("../assets/square.png"),None,),
        //     "line" => Texture2D::from_file_with_format(include_bytes!("../assets/line.png"),None,),
        //     "reserve" => Texture2D::from_file_with_format(include_bytes!("../assets/reserve.png"),None,),
        //     "king" => Texture2D::from_file_with_format(include_bytes!("../assets/king.png"),None,),
        //     "gold" => Texture2D::from_file_with_format(include_bytes!("../assets/gold.png"),None,),
        //     "silver" => Texture2D::from_file_with_format(include_bytes!("../assets/silver.png"),None,),
        //     "silver_pro" => Texture2D::from_file_with_format(include_bytes!("../assets/silver_pro.png"),None,),
        //     "pawn" => Texture2D::from_file_with_format(include_bytes!("../assets/pawn.png"),None,),
        //     "pawn_pro" => Texture2D::from_file_with_format(include_bytes!("../assets/pawn_pro.png"),None,),
        //     _ => panic!("No texture by that name."),
        // }
    // }

    pub fn get_texture(name: &str) -> Texture2D {
        match name {
            // button
            "button_0" => Texture2D::from_file_with_format(include_bytes!("../assets/button_60x24_2x.png"),None,),
            "button_1" => Texture2D::from_file_with_format(include_bytes!("../assets/button_80x24_2x.png"),None,),

            // controller
            "bar_about" => Texture2D::from_file_with_format(include_bytes!("../assets/bar_about_2x.png"),None,),
            "bar_rules" => Texture2D::from_file_with_format(include_bytes!("../assets/bar_rules_2x.png"),None,),
            "bar_settings" => Texture2D::from_file_with_format(include_bytes!("../assets/bar_settings_2x.png"),None,),
            "bar_quit" => Texture2D::from_file_with_format(include_bytes!("../assets/bar_quit_2x.png"),None,),

            // view_intro
            "view_intro" => Texture2D::from_file_with_format(include_bytes!("../assets/view_intro_2x.png"),None,),

            // view_settings
            "view_settings" => Texture2D::from_file_with_format(include_bytes!("../assets/view_settings_2x.png"),None,),
            "okay" => Texture2D::from_file_with_format(include_bytes!("../assets/okay_2x.png"),None,),
            "button_human" => Texture2D::from_file_with_format(include_bytes!("../assets/button_human_2x.png"),None,),
            "button_ai" => Texture2D::from_file_with_format(include_bytes!("../assets/button_ai_2x.png"),None,),
            //"monte_carlo" => Texture2D::from_file_with_format(include_bytes!("../assets/monte_carlo_2x.png"),None,),

            // view board
            "square" => Texture2D::from_file_with_format(include_bytes!("../assets/square_2x.png"),None,),
            "line" => Texture2D::from_file_with_format(include_bytes!("../assets/line_2x.png"),None,),
            "reserve" => Texture2D::from_file_with_format(include_bytes!("../assets/reserve_2x.png"),None,),
            "king" => Texture2D::from_file_with_format(include_bytes!("../assets/king_2x.png"),None,),
            "gold" => Texture2D::from_file_with_format(include_bytes!("../assets/gold_2x.png"),None,),
            "silver" => Texture2D::from_file_with_format(include_bytes!("../assets/silver_2x.png"),None,),
            "silver_pro" => Texture2D::from_file_with_format(include_bytes!("../assets/silver_pro_2x.png"),None,),
            "pawn" => Texture2D::from_file_with_format(include_bytes!("../assets/pawn_2x.png"),None,),
            "pawn_pro" => Texture2D::from_file_with_format(include_bytes!("../assets/pawn_pro_2x.png"),None,),
            // view rules
            "close" => Texture2D::from_file_with_format(include_bytes!("../assets/close_2x.png"),None,),
            "rules_view" => Texture2D::from_file_with_format(include_bytes!("../assets/rules_view_2x.png"),None,),
            _ => panic!("No texture by that name: {}", name),
        }
    }

    #[allow(dead_code)]
    /// Given the corner position, returns the center of the given texture.
    pub fn center_position(texture: &Texture2D, corner: (f32, f32)) -> (f32, f32) {
        (corner.0 + texture.width() / 2.0, corner.1 + texture.height() / 2.0)
    }

    /// Returns the sound associated with the given name. Async function.
    pub async fn get_sound(name: &str) -> Sound {
        let res = match name {
            "piece_move" => load_sound_from_bytes(include_bytes!("../assets/piece_move.wav")).await,
            "piece_capture" => load_sound_from_bytes(include_bytes!("../assets/piece_capture.wav")).await,
            _ => panic!("No sound by that name."),
        };
        match res {
            Ok(snd) => snd,
            _ => panic!("Sound could not be loaded."),
        }
    }

    /// Returns the font associated with the given name.
    pub fn get_font(name: &str) -> Font {
        let res = match name {
            "Menlo" => load_ttf_font_from_bytes(include_bytes!("../assets/Menlo.ttc")),
            "Menlo Bold" => load_ttf_font_from_bytes(include_bytes!("../assets/Menlo-Bold.ttf")),
            _ => panic!("No font by that name."),
        };
        match res {
            Ok(font) => font,
            _ => panic!("Font could not be loaded."),
        }
    }
}