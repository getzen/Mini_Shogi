// Mod.rs

pub mod animators;
pub mod button;
pub mod button3;
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

use macroquad::prelude::*;

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


/// Draw to a texture using the given function with drawing commands. Width and height
/// are passed in as examples of passing arguments to the draw function.
pub fn draw_to_texture(draw_fn: fn(u32, u32), phys_width: u32, phys_height: u32) -> Texture2D {
    let render_target = render_target(phys_width, phys_height);
    // For pixel art, use:
    // render_target.texture.set_filter(FilterMode::Nearest);

    // The zoom x:y ratio must match the phys_width:phys_height ratio, with 0.01 as the nominal setting.
    let (mut zoom_x, mut zoom_y) = (0.01, 0.01);
    if phys_width > phys_height {
        zoom_y = 0.01 * phys_width as f32 / phys_height as f32;
    } else {
        zoom_x = 0.01 * phys_height as f32 / phys_width as f32;
    }

    set_camera(&Camera2D {
        // It seems that, when rendering to a texture, 0.01 means "no zoom", a 1:1 pixel ratio.
        zoom: vec2(zoom_x, zoom_y),
        // Look at the center of the texture.
        target: vec2(phys_width as f32 / 2.0, phys_height as f32 / 2.0),
        render_target: Some(render_target),
        ..Default::default()
    });

    draw_fn(phys_width, phys_height);

    // All done. Restore default camera.
    set_default_camera();
    render_target.texture
}

/// Sample draw function.
fn draw_fn(phys_width: u32, phys_height: u32) {
    clear_background(Color::from_rgba(255, 255, 255, 255));
    draw_rectangle(0., 0., 100., 100., BLACK);
}