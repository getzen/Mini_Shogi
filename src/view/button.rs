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
    Hovering(Option<usize>),
    /// Normal push-button behavior.
    Pushed(Option<usize>),
    /// Toggle on or off.
    Toggled(Option<usize>),
    // As with a radio button.
    //Selected(usize),
}

// #[derive(PartialEq)]
// pub enum ButtonState {
//     Normal,
//     Disabled,
//     MouseOver,
//     Selected,
// }

use macroquad::prelude::{Color, WHITE};
use macroquad::prelude::Texture2D;

use crate::view::draw_texture::DrawTexture;
use crate::view::eventable::Event;
use crate::view::eventable::Eventable;
use crate::view::transform::Transform;

use crate::view::*;

pub struct Button {
    pub id: Option<usize>,
    pub group_id: Option<usize>,
    pub transform: Transform,
    pub drawable: DrawTexture,
    pub eventable: Eventable,

    /// Default is Push.
    pub mode: ButtonMode,
    //pub state: ButtonState,

    pub normal_color: Color,
    //pub disabled: bool,
    pub disabled_color: Color,
    pub mouse_over_color: Color,
    pub selected: bool,
    pub selected_color: Color,
    draw_color: Color,
}

impl Button {
    pub fn new(logi_position: (f32, f32), texture: Texture2D, id: Option<usize>) -> Self {
        let phys_position = phys_pos(logi_position);

        Self {
            id,
            group_id: None,
            transform: Transform::new(phys_position, 0.0),
            drawable: DrawTexture::new(texture, false),
            eventable: Eventable::new(),
            mode: ButtonMode::Push,
            //state: ButtonState::Normal,
            normal_color: WHITE,
            //disabled: false,
            disabled_color: Color::from_rgba(150, 150, 150, 255),
            mouse_over_color: Color::from_rgba(235, 235, 235, 255),
            selected: false,
            selected_color: Color::from_rgba(200, 225, 255, 255),
            draw_color: WHITE,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.eventable.enabled = enabled;
        self.draw_color = if enabled { self.normal_color } else { self.disabled_color }
    }
    // Convenience methods

    pub fn contains_phys_position(&self, phy_position: (f32, f32)) -> bool {
        self.eventable.contains_phys_position(phy_position, &self.transform, &self.drawable)
    }

    pub fn process_events(&mut self) -> Option<ButtonEvent> {
        if !self.drawable.visible { return None }

        let event = self.eventable.process_events(&self.transform, &self.drawable);

        if event.is_none() { return None }

        match event.unwrap() {
            Event::MouseEntered => {
                if !self.selected {
                    self.draw_color = self.mouse_over_color;
                }
                None
            },
            Event::MouseExited => {
                if !self.selected {
                    self.draw_color = self.normal_color;
                }
                None
            },
            Event::LeftMousePressed => {
                self.draw_color = self.selected_color;
                None
            },
            Event::LeftMouseReleased => {
                match self.mode {
                    ButtonMode::Push => Some(ButtonEvent::Pushed(self.id)),
                    ButtonMode::Toggle => {
                        self.selected = !self.selected;
                        if self.selected {
                            self.draw_color = self.selected_color;
                        } else {
                            self.draw_color = self.mouse_over_color;
                        }
                        Some(ButtonEvent::Toggled(self.id))
                    },
                    //ButtonMode::Radio => todo!(),
                }
            },
        }
    }

    pub fn draw(&mut self) {
        self.drawable.draw(&self.transform, Some(self.draw_color));
    }
}
