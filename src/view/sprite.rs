// Sprite
// A basic sprite implementation.
//
// An array of sprites can be sorted by z_order like so:
// my_sprites.sort_by(|a, b| a.z_order.cmp(&b.z_order));

use std::time::Duration;

use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::view::drawable::Drawable;
use crate::view::eventable::Event;
use crate::view::eventable::Eventable;
use crate::view::transform::Transform;

use crate::view::*;

pub struct Sprite {
    pub id: Option<usize>,
    pub transform: Transform,
    pub drawable: Drawable,
    pub eventable: Eventable,

    // This app
    pub alt_texture: Option<Texture2D>,
    pub use_alt_texture: bool,

    pub alt_color: Option<Color>,
    pub use_alt_color: bool,
}

impl Sprite {
    /// Creates a new Sprite with the given logical position, texture, and optonal id.
    pub fn new(logi_position: (f32, f32), texture: Texture2D, id: Option<usize>) -> Self {
        let phys_position = phys_pos(logi_position);

        Self {
            id,
            transform: Transform::new(phys_position, 0.0),
            drawable: Drawable::new(texture, true),
            eventable: Eventable::new(),
            alt_texture: None,
            use_alt_texture: false,
            alt_color: None,
            use_alt_color: false,
        }
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        let mut updated = false;
        if self.transform.update(time_delta) {
            updated = true;
        }
        if self.drawable.update(time_delta) {
            updated = true;
        }
        updated
    }

    pub fn process_events(&self) -> Option<Event> {
        self.eventable.process_events(&self.transform, &self.drawable)
    }

    pub fn draw(&self) {
        self.drawable.draw(&self.transform);
    }

}



// pub struct Sprite2 {
//     /// Position in physical pixels of the center.
//     /// Use set_logi_position for logical pixel positioning.
//     pub phys_position: (f32, f32),
//     /// Rotation in radians clockwise
//     pub rotation: f32,

//     /// The displayed size of the texture in physical pixels. The initial size is automatically
//     /// scaled by the window dpi scale.
//     /// Use set_logi_size for logical pixel sizing.
//     pub size: (f32, f32),

//     //pub pivot: Option<(f32, f32)>, // Implementing this would require changes to 'contains' logic.

//     pub texture: Texture2D,
//     pub alt_texture: Option<Texture2D>,
//     pub use_alt_texture: bool,

//     pub color: Color,
//     pub alt_color: Option<Color>,
//     pub use_alt_color: bool,

//     pub z_order: usize, // view can use this to sort
//     pub is_visible: bool,
//     pub id: Option<usize>,

//     // Private
//     params: DrawTextureParams,
//     position_lerp: Option<Lerp>, // created when needed
//     fade_lerp: Option<Lerp>, // created when needed
//     // rotation_lerp...
// }

// impl Sprite2 {
//     #[allow(dead_code)]
//     // Use AssetLoader instead.
//     async fn load_texture(name: &str) -> Texture2D {
//         let mut path = "./assets".to_owned();
//         path.push_str(name);
//         load_texture(&path).await.unwrap()
//     }

//     /// Creates a new Sprite with the given logical position and texture. The texture will
//     /// be automatically scaled, if needed, for the dpi scale. In view.rs, see
//     /// IMAGE_ASSETS_SCALE.
//     pub fn new(logi_position: (f32, f32), texture: Texture2D) -> Self {
//         Self {
//             phys_position: View::phys_pos(logi_position),
//             rotation: 0.0,
//             size: (texture.width() * View::adj_scale(), texture.height() * View::adj_scale()),

//             /// See note above.
//             /// Rotate around this point.
//             /// When `None`, rotate around the texture's center.
//             /// When `Some`, the coordinates are in screen-space.
//             /// E.g. pivot (0,0) rotates around the top left corner of the screen, not of the
//             //pivot: None,

//             texture,
//             alt_texture: None,
//             use_alt_texture: false,

//             color: WHITE,
//             alt_color: None,
//             use_alt_color: false,

//             z_order: 0,
//             is_visible: true,
//             id: None,
//             params: DrawTextureParams::default(),
//             position_lerp: None,
//             fade_lerp: None,
//         }
//     }

//     #[allow(dead_code)]
//     /// Get the logical position of the sprite.
//     pub fn get_logi_position(&self) -> (f32, f32) {
//         View::logi_pos(self.phys_position)
//     }

//     #[allow(dead_code)]
//     /// Set the logical position of the sprite.
//     pub fn set_logi_position(&mut self, logi_position: (f32, f32)) {
//         self.phys_position = View::phys_pos(logi_position);
//     }

//     #[allow(dead_code)]
//     /// Get the size of the diplayed texture in logical pixels.
//     pub fn get_logi_size(&self) -> (f32, f32) {
//         (self.size.0 / View::dpi_scale(), self.size.1 / View::dpi_scale())
//     }

//     #[allow(dead_code)]
//     /// Set the size of the displayed texture using logical pixel size.
//     pub fn set_logi_size(&mut self, logi_size: (f32, f32)) {
//         self.size.0 = logi_size.0 * View::adj_scale();
//         self.size.1 = logi_size.1 * View::adj_scale();
//     }

//     #[allow(dead_code)]
//     /// Set the texture size using the given scale. Considers dpi scale.
//     pub fn scale_by(&mut self, scale: (f32, f32)) {
//         self.size.0 = self.texture.width() * View::adj_scale() * scale.0;
//         self.size.1 = self.texture.height() * View::adj_scale() * scale.1;
//     }

//     #[allow(dead_code)]
//     /// Test whether the logical position lies in texture rectange, considering rotation.
//     pub fn contains_logi_position(&self, logi_pos: (f32, f32)) -> bool {
//         self.contains_phys_position(View::phys_pos(logi_pos))
//     }

//     #[allow(dead_code)]
//     /// Test whether the physical point lies in the texture rectangle, considering rotation.
//     /// Note: Macroquad's mouse_position() gives the physical location of the mouse.
//     pub fn contains_phys_position(&self, phys_position: (f32, f32)) -> bool {
//         // Get the net test point relative to the sprite's position.
//         let net_x = phys_position.0 - self.phys_position.0;
//         let net_y = phys_position.1 - self.phys_position.1;
//         // Rotate the point clockwise (the same direction as Macroquad's rotation). This is a
//         // little different than the standard rotation formulas.
//         let theta = self.rotation;
//         let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
//         let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
//         // See if the rotated point is in the unrotated sprite rectangle.
//         let (w, h) = self.size;
//         f32::abs(rot_x) < w / 2.0 && f32::abs(rot_y) < h / 2.0
//     }

//     /// Returns the position at which the texture should be drawn, effectively centering
//     /// at self.position.
//     fn centered_position(&self) -> (f32, f32) {
//         let (x, y) = self.phys_position;
//         let (w, h) = self.size;
//         (x - w / 2.0, y - h / 2.0)
//     }

//     /// Perform animation updates and the like with the time_delta.
//     /// If update did something, return true, otherwise false.
//     pub fn update(&mut self, time_delta: Duration) -> bool {
//         // Position animation
//         if let Some(lerp) = &mut self.position_lerp {
//             let results = lerp.update(time_delta);
//             self.phys_position = (results.0, results.1);
//             if !results.2 {
//                 self.position_lerp = None;
//             }
//             return true;
//         }

//         // Fade animation
//         if let Some(lerp) = &mut self.fade_lerp {
//             let results = lerp.update(time_delta);
//             self.color.a = results.0;
//             if !results.2 {
//                 self.fade_lerp = None;
//             }
//             return true;
//         }
//         false
//     }

//     #[allow(dead_code)]
//     /// Use the Lerp struct to move the sprite.
//     pub fn animate_move(&mut self, to: (f32, f32), duration: Duration) {
//         let end = View::phys_pos(to);
//         self.position_lerp = Some(Lerp::new(self.phys_position, end, duration));
//     }

//     #[allow(dead_code)]
//     /// Use the Lerp struct to fade out the sprite.
//     pub fn animate_fade_out(&mut self, duration: Duration) {
//         self.fade_lerp = Some(Lerp::new((1.0, 0.0), (0.0, 0.0), duration));
//     }

//     /// Draw the sprite.
//     pub fn draw(&mut self) {
//         if !self.is_visible { return; }

//         let (x, y) = self.centered_position();

//         let mut draw_color = self.color;
//         if self.use_alt_color && self.alt_color.is_some() {
//             draw_color = self.alt_color.unwrap();
//         }

//         self.params.dest_size = Some(Vec2::new(self.size.0, self.size.1));
//         self.params.rotation = self.rotation;

//         // if let Some(piv) = self.pivot {
//         //     params.pivot = Some(Vec2::new(piv.0, piv.1));
//         // }
//         // params source, flip_x, etc. =

//         if !self.use_alt_texture {
//             draw_texture_ex(self.texture, x, y, draw_color, self.params.clone());
//         } else {
//             draw_texture_ex(self.alt_texture.unwrap(), x, y, draw_color, self.params.clone());
//         }
//     }
// }