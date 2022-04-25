// Mod.rs

pub mod animators;
pub mod button;
pub mod button_bar;
pub mod draw_text;
pub mod draw_texture;
pub mod eventable;
pub mod image;
pub mod slider;
pub mod sprite;
pub mod label;
pub mod transform;

// mods for this app
pub mod view_game;
pub mod view_intro;
pub mod view_rules;
pub mod view_settings;


/// The ratio of the image actual size versus the intended display size.
/// For example, it should be 2.0 if images are twice their intended display
/// size for sharpness with Retina displays. Consider ending assets names with
/// "_2x" as a reminder.
const IMAGE_ASSETS_SCALE: f32 = 2.0;

// Utility methods related to high-resolution image display.

/// Returns the number of physical pixels per logical pixel.
pub fn dpi_scale() -> f32 {
    unsafe {
        macroquad::prelude::get_internal_gl().quad_context.dpi_scale()
    }
}

/// Returns the scaling factor that should be used for textures and images,
/// given the dpi scale and the asset scale.
pub fn adj_scale() -> f32 {
    1.0 / IMAGE_ASSETS_SCALE * dpi_scale()
}

/// Returns the physical pixels position, considering the dpi scale,
/// given the logical positions. Use to place sprites and other elements.
pub fn phys_pos(logi_position: (f32, f32)) -> (f32, f32) {
    (logi_position.0 * dpi_scale(), logi_position.1 * dpi_scale())
}

/// Returns the logical position, considering the dpi scale, given the 
/// physical pixel position. Use to scale mouse coordinates, for instance.
pub fn logi_pos(phys_position: (f32, f32)) -> (f32, f32) {
    (phys_position.0 / dpi_scale(), phys_position.1 / dpi_scale())
}