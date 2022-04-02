// Resource loader

use std::collections::HashMap;
use std::env::current_exe;
use std::path::PathBuf;

use macroquad::audio::load_sound;
use macroquad::audio::Sound;
use macroquad::prelude::load_texture;
use macroquad::prelude::Texture2D;

const RESOURCE_PATH: &str = "assets/";

pub struct ResourceLoader {}

impl ResourceLoader {
    
    fn resource_path(file_name: &str) -> PathBuf {
        match current_exe() {
            Ok(current) => {
                let parent = current.parent().unwrap();
                let path = parent.join(RESOURCE_PATH);
                return path.join(file_name);
            },
            Err(_) => todo!(),
        }
    }

    pub async fn load_textures(names: &[&'static str]) -> HashMap<&'static str, Texture2D> {
        let mut hash = HashMap::new();
        for name in names {
            let path_buf = ResourceLoader::resource_path(name);
            let path = path_buf.to_str().unwrap();
            let texture = load_texture(&path).await.unwrap();
            hash.insert(name.to_owned(), texture);
        }
        hash
    }

    pub async fn load_sounds(names: &[&'static str]) -> HashMap<&'static str, Sound> {
        let mut hash = HashMap::new();
        for name in names {
            let path_buf = ResourceLoader::resource_path(name);
            let path = path_buf.to_str().unwrap();
            let sound = load_sound(&path).await.unwrap();
            hash.insert(name.to_owned(), sound);
        }
        hash
    }
}