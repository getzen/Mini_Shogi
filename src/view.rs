// View
// Utility methods related to high-resolution image display.

use macroquad::prelude::get_internal_gl;

/// The ratio of the image actual size versus the intended display size.
/// For example, it should be 2.0 if images are twice their intended display
/// size for sharpness with Retina displays. Consider ending assets names with
/// "_2x" as a reminder.
const IMAGE_ASSETS_SCALE: f32 = 2.0;

pub struct View {}

impl View {
    /// Returns the number of physical pixels per logical pixels.
    pub fn dpi_scale() -> f32 {
        unsafe {
            get_internal_gl().quad_context.dpi_scale()
        }
    }

    /// Returns the scaling factor that should be used for textures and images,
    /// given the dpi scale and the asset scale.
    pub fn adj_scale() -> f32 {
        1.0 / IMAGE_ASSETS_SCALE * View::dpi_scale()
    }

    /// Returns the physical pixels position, considering the dpi scale,
    /// given the logical positions. Use to place sprites and other elements.
    pub fn phys_pos(logi_position: (f32, f32)) -> (f32, f32) {
        (logi_position.0 * View::dpi_scale(), logi_position.1 * View::dpi_scale())
    }

    /// Returns the logical position, considering the dpi scale, given the 
    /// physical pixel position. Use to scale mouse coordinates, for instance.
    pub fn logi_pos(phys_position: (f32, f32)) -> (f32, f32) {
        (phys_position.0 / View::dpi_scale(), phys_position.1 / View::dpi_scale())
    }
}

