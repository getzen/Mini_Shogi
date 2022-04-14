// View
// Utility methods

use macroquad::prelude::get_internal_gl;

// This should be 2.0 if images are twice their intended display size
// to handle Retina displays. End assets names with "-2x" as a reminder.
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
    pub fn phys_pos(logical_position: (f32, f32)) -> (f32, f32) {
        (logical_position.0 * View::dpi_scale(), logical_position.1 * View::dpi_scale())
    }

    /// Returns the logical position, considering the dpi scale, given the 
    /// physical pixel position. Use to scale mouse coordinates, for instance.
    pub fn logi_pos(physical_position: (f32, f32)) -> (f32, f32) {
        (physical_position.0 / View::dpi_scale(), physical_position.1 / View::dpi_scale())
    }
}

