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

use macroquad::prelude::{Color, GRAY, WHITE, YELLOW};
use macroquad::prelude::Texture2D;

use crate::view::draw_texture::DrawTexture;
use crate::view::eventable::Event;
use crate::view::eventable::Eventable;
use crate::view::transform::Transform;

use crate::view::*;

pub struct Button {
    pub id: Option<usize>,
    pub transform: Transform,
    pub drawable: DrawTexture,
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
            drawable: DrawTexture::new(texture, false),
            eventable: Eventable::new(),
            mode: ButtonMode::Push,

            color: WHITE,
            disabled: false,
            disabled_color: Some(GRAY),
            selected: false,
            selected_color: Some(YELLOW),
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
                    //ButtonMode::Radio => todo!(),
                }
            },
        }
    }

    pub fn draw(&mut self) {
        let draw_color = 
        if self.disabled {
            self.disabled_color
        } else if self.selected {
            self.selected_color
        } else {
            None
        };
        self.drawable.draw(&self.transform, draw_color);
    }
}
