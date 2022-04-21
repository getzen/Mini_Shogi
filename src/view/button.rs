// Button


#[allow(dead_code)]
#[derive(PartialEq)]
pub enum ButtonMode {
    Push,
    Toggle,
    //Radio,
}

#[derive(Debug, PartialEq)]
pub enum ButtonEvent {
    // Mouse is over button. (id)
    //Hovering(Option<usize>),
    /// Normal push-button behavior.
    Pushed(Option<usize>),
    /// Toggle on or off.
    Toggled(Option<usize>),
    // As with a radio button.
    //Selected(usize),
}

use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::view::drawable::Drawable;
use crate::view::eventable::Event;
use crate::view::eventable::Eventable;
use crate::view::transform::Transform;

use crate::view::*;

pub struct Button {
    pub id: Option<usize>,
    pub transform: Transform,
    pub drawable: Drawable,
    pub eventable: Eventable,

    /// Default is Push.
    pub mode: ButtonMode,

    pub color: Color,
    pub disabled: bool,
    pub disabled_color: Option<Color>,
    pub selected: bool,
    pub selected_color: Option<Color>,
}

impl Button {
    pub fn new(logi_position: (f32, f32), texture: Texture2D, id: Option<usize>) -> Self {
        let phys_position = phys_pos(logi_position);

        Self {
            id,
            transform: Transform::new(phys_position, 0.0),
            drawable: Drawable::new(texture, false),
            eventable: Eventable::new(),
            mode: ButtonMode::Push,
            disabled: false,
            disabled_color: None,
            selected: false,
            selected_color: None,
        }
    }

    // Convenience methods

    pub fn contains_phys_position(&self, phy_position: (f32, f32)) -> bool {
        self.eventable.contains_phys_position(phy_position, &self.transform, &self.drawable)
    }

    pub fn process_events(&self) -> Option<ButtonEvent> {
        if !self.drawable.visible { return None }
        let event = self.eventable.process_events(&self.transform, &self.drawable);
        if event.is_none() { return None }

        match event.unwrap() {
            Event::MouseEntered => todo!(),
            Event::MouseExited => todo!(),
            Event::LeftMouseDown => todo!(),
            Event::LeftMouseReleased => {
                match self.mode {
                    ButtonMode::Push => Some(ButtonEvent::Pushed(self.id)),
                    ButtonMode::Toggle => {
                        self.selected = !self.selected;
                        Some(ButtonEvent::Toggled(self.id))
                    },
                    ButtonMode::Radio => todo!(),
                }
            },
        }

    }

    // pub fn process_events(&mut self) -> Option<ButtonEvent> {
    //     let mut event = None;
    //     if !self.is_visible || !self.is_enabled { return event }

    //     self.is_mouse_over = self.contains_phys_position(mouse_position());
    //     if self.is_mouse_over {
    //         event = Some(ButtonEvent::Hovering(self.id));
    //     }
    //     let button_pressed = is_mouse_button_down(MouseButton::Left);
    //     // See if button was released *this frame*.
    //     let button_released = is_mouse_button_released(MouseButton::Left);

    //     match &self.mode {
    //         ButtonMode::Push => {
    //             self.is_selected = self.is_mouse_over && button_pressed;
    //             if self.is_mouse_over && button_released {
    //                 event = Some(ButtonEvent::Pushed(self.id));
    //                 self.is_selected = false;
    //             }
    //         },
    //         ButtonMode::Toggle => {
    //             if self.is_mouse_over && button_released {
    //                 self.is_selected = !self.is_selected;
    //                 event = Some(ButtonEvent::Toggled(self.id));
    //             }
    //         },
    //         ButtonMode::Radio => {
    //             if self.is_mouse_over && button_released {
    //                 if !self.is_selected {
    //                     event = Some(ButtonEvent::Selected(self.id));
    //                 }
    //                 self.is_selected = true;
    //             }
    //         },
    //     }
    //     event
    // }

    pub fn draw(&self) {
        
        self.drawable.draw(&self.transform);
    }

}

pub struct Button2 {
    /// Position in physical pixels of the top-left corner.
    /// Use set_logi_position for logical pixel positioning.
    pub phys_position: (f32, f32),
    /// Rotation in radians clockwise
    pub rotation: f32,

    pub texture: Texture2D,
    pub disabled_texture: Option<Texture2D>,
    pub selected_texture: Option<Texture2D>,

    pub color: Color,
    pub disabled_color: Option<Color>,
    pub selected_color: Option<Color>,

    pub z_order: usize, // default 0
    pub mode: ButtonMode,

    pub is_visible: bool,
    pub is_enabled: bool,
    pub is_mouse_over: bool,
    pub is_selected: bool,

    pub id: usize,
    pub group_id: usize, // for radio-style groups
    
    // Private
    draw_params: DrawTextureParams,

}

/// Creates a new Button with the given logical position, texture, mode, and id.
/// The texture will be automatically scaled, if needed, for the dpi scale.
/// In view.rs, see IMAGE_ASSETS_SCALE.
impl Button2 {
    pub fn new(
        logi_position: (f32, f32), 
        texture: Texture2D, 
        mode: ButtonMode,
        id: usize) -> Self {

        // Adjust texture draw size based on the dpi scale.
        let mut params = DrawTextureParams::default();
        let size_x = texture.width() * View::adj_scale();
        let size_y = texture.height() * View::adj_scale();
        params.dest_size = Some(Vec2::new(size_x, size_y));

        Self {
            phys_position: View::phys_pos(logi_position),
            rotation: 0.,
            texture, mode, id,
            disabled_texture: None,
            selected_texture: None,
            color: WHITE,
            disabled_color: Some(GRAY),
            selected_color: Some(YELLOW),
            draw_params: params,
            z_order: 0,
            is_visible: true,
            is_enabled: true,
            is_mouse_over: false,
            is_selected: false,
            group_id: 0,
        }
    }

    #[allow(dead_code)]
    /// Get the logical position of the sprite.
    pub fn get_logi_position(&self) -> (f32, f32) {
        View::logi_pos(self.phys_position)
    }

    #[allow(dead_code)]
    /// Set the logical position of the sprite.
    pub fn set_logi_position(&mut self, logi_position: (f32, f32)) {
        self.phys_position = View::phys_pos(logi_position);
    }

    #[allow(dead_code)]
    /// Test whether the logical position lies in texture rectange, considering rotation.
    pub fn contains_logi_position(&self, logi_pos: (f32, f32)) -> bool {
        self.contains_phys_position(View::phys_pos(logi_pos))
    }

    #[allow(dead_code)]
    /// Test whether the physical point lies in the texture rectangle, considering rotation.
    /// Note: Macroquad's mouse_position() gives the physical location of the mouse.
    pub fn contains_phys_position(&self, phys_position: (f32, f32)) -> bool {
        let size = self.draw_params.dest_size.unwrap();
        let net_x = phys_position.0 - self.phys_position.0 - size.x / 2.0;
        let net_y = phys_position.1 - self.phys_position.1 - size.y / 2.0;
        // Rotate the point clockwise (the same direction as Macroquad's rotation).
        let theta = self.draw_params.rotation;
        let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
        let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
        // See if the rotated point is in the unrotated sprite rectangle.
        f32::abs(rot_x) < size.x / 2.0 && f32::abs(rot_y) < size.y / 2.0
    }

    #[allow(dead_code)]
    /// Returns the size of button in logical units.
    fn logical_size(&self) -> (f32, f32) {
        let size = self.draw_params.dest_size.unwrap();
        (size.x / View::dpi_scale(), size.y / View::dpi_scale())
    }

    pub fn process_events(&mut self) -> Option<ButtonEvent> {
        let mut event = None;
        if !self.is_visible || !self.is_enabled { return event }

        self.is_mouse_over = self.contains_phys_position(mouse_position());
        if self.is_mouse_over {
            event = Some(ButtonEvent::Hovering(self.id));
        }
        let button_pressed = is_mouse_button_down(MouseButton::Left);
        // See if button was released *this frame*.
        let button_released = is_mouse_button_released(MouseButton::Left);

        match &self.mode {
            ButtonMode::Push => {
                self.is_selected = self.is_mouse_over && button_pressed;
                if self.is_mouse_over && button_released {
                    event = Some(ButtonEvent::Pushed(self.id));
                    self.is_selected = false;
                }
            },
            ButtonMode::Toggle => {
                if self.is_mouse_over && button_released {
                    self.is_selected = !self.is_selected;
                    event = Some(ButtonEvent::Toggled(self.id));
                }
            },
            ButtonMode::Radio => {
                if self.is_mouse_over && button_released {
                    if !self.is_selected {
                        event = Some(ButtonEvent::Selected(self.id));
                    }
                    self.is_selected = true;
                }
            },
        }
        event
    }

    pub fn draw(&mut self) {
        if !self.is_visible { return ;}
        
        let mut draw_texture = self.texture;
        let mut draw_color = self.color;

        if self.is_enabled {
            if self.is_selected {
                if self.selected_texture.is_some() {
                    draw_texture = self.selected_texture.unwrap();
                }
                if self.selected_color.is_some() {
                    draw_color = self.selected_color.unwrap();
                }
            }
        } else { // disabled
            if self.disabled_texture.is_some() {
                draw_texture = self.disabled_texture.unwrap();
            }
            if self.disabled_color.is_some() {
                draw_color = self.disabled_color.unwrap();
            }
        }
        // Convert logical position to physical pixel position.
        let (x, y) = self.phys_position;
        draw_texture_ex(draw_texture, x, y, draw_color, self.draw_params.clone());
    }
}