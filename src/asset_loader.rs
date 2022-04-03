// Asset Loader

use macroquad::audio::load_sound_from_bytes;
use macroquad::audio::Sound;
use macroquad::prelude::Font;
use macroquad::prelude::load_ttf_font_from_bytes;
use macroquad::texture::Texture2D;

pub struct AssetLoader {}

impl AssetLoader {

    /// Returns the texture associated with the given name.
    pub fn get_texture(name: &str) -> Texture2D {
        match name {
            // view_intro
            "title" => Texture2D::from_file_with_format(include_bytes!("../assets/title.png"),None,),
            "start" => Texture2D::from_file_with_format(include_bytes!("../assets/start.png"),None,),
            "rules" => Texture2D::from_file_with_format(include_bytes!("../assets/rules.png"),None,),
            "exit" => Texture2D::from_file_with_format(include_bytes!("../assets/exit.png"),None,),
            // view board
            "square" => Texture2D::from_file_with_format(include_bytes!("../assets/square.png"),None,),
            "line" => Texture2D::from_file_with_format(include_bytes!("../assets/line.png"),None,),
            "reserve" => Texture2D::from_file_with_format(include_bytes!("../assets/reserve.png"),None,),
            "king" => Texture2D::from_file_with_format(include_bytes!("../assets/king.png"),None,),
            "gold" => Texture2D::from_file_with_format(include_bytes!("../assets/gold.png"),None,),
            "silver" => Texture2D::from_file_with_format(include_bytes!("../assets/silver.png"),None,),
            "silver_pro" => Texture2D::from_file_with_format(include_bytes!("../assets/silver_pro.png"),None,),
            "pawn" => Texture2D::from_file_with_format(include_bytes!("../assets/pawn.png"),None,),
            "pawn_pro" => Texture2D::from_file_with_format(include_bytes!("../assets/pawn_pro.png"),None,),
            _ => panic!("No texture by that name."),
        }
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
            _ => panic!("No font by that name."),
        };
        match res {
            Ok(font) => font,
            _ => panic!("Font could not be loaded."),
        }
    }
}