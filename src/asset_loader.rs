// Asset Loader

use macroquad::audio::load_sound_from_bytes;
use macroquad::audio::Sound;
use macroquad::texture::Texture2D;

pub struct AssetLoader {}

impl AssetLoader {

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

    pub async fn get_sound(name: &str) -> Sound {
        let res;
        match name {
            "piece_move" => {res = load_sound_from_bytes(include_bytes!("../assets/piece_move.wav")).await},
            "piece_capture" => {res = load_sound_from_bytes(include_bytes!("../assets/piece_capture.wav")).await},
            _ => panic!("No sound by that name."),
        };
        match res {
            Ok(snd) => snd,
            _ => panic!("Sound could not be loaded."),
        }
    }

    // fn resource_path(file_name: &str) -> PathBuf {
    //     match current_exe() {
    //         Ok(current) => {
    //             let parent = current.parent().unwrap();
    //             let path = parent.join(RESOURCE_PATH);
    //             return path.join(file_name);
    //         },
    //         Err(_) => todo!(),
    //     }
    // }

    // pub async fn load_textures2(names: &[&'static str]) -> HashMap<&'static str, Texture2D> {
        
    //     let mut hash = HashMap::new();
    //     for name in names {
    //         let path_buf = ResourceLoader::resource_path(name);
    //         let path = path_buf.to_str().unwrap();
            
    //         let texture = load_texture(&path).await.unwrap();
    //         hash.insert(name.to_owned(), texture);
    //     }
    //     hash
    // }

    // pub async fn load_textures(names: &[&'static str]) -> HashMap<&'static str, Texture2D> {
    //     let mut hash = HashMap::new();
    //     for name in names {
    //         let path_buf = ResourceLoader::resource_path(name);
    //         let path = path_buf.to_str().unwrap();
    //         let texture = load_texture(&path).await.unwrap();
    //         hash.insert(name.to_owned(), texture);
    //     }
    //     hash
    // }

    // pub async fn load_sounds(names: &[&'static str]) -> HashMap<&'static str, Sound> {
    //     let mut hash = HashMap::new();
    //     for name in names {
    //         let path_buf = ResourceLoader::resource_path(name);
    //         let path = path_buf.to_str().unwrap();
    //         let sound = load_sound(&path).await.unwrap();
    //         hash.insert(name.to_owned(), sound);
    //     }
    //     hash
    // }
}