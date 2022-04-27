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
    /// Normal push-button behavior.
    Pushed(Option<usize>),
    /// Toggle on or off.
    Toggled(Option<usize>),
}

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
    pub parent_transform: Transform,
    pub drawable: DrawTexture,
    pub eventable: Eventable,

    /// Default is Push.
    pub mode: ButtonMode,

    pub normal_color: Color,
    pub disabled_color: Color,
    pub mouse_over_color: Color,
    pub selected_color: Color,
    // Private
    selected: bool,
    draw_color: Color,
}

impl Button {
    pub fn new(logi_position: (f32, f32), texture: Texture2D, id: Option<usize>) -> Self {
        let phys_position = phys_pos(logi_position);

        let mut button = Self {
            id,
            group_id: None,
            transform: Transform::new(phys_position, 0.0),
            parent_transform: Transform::new((0., 0.), 0.),
            drawable: DrawTexture::new(texture, false),
            eventable: Eventable::new(),
            mode: ButtonMode::Push,
            normal_color: Color::from_rgba(235, 235, 235, 255),
            disabled_color: Color::from_rgba(255, 255, 255, 150),
            mouse_over_color: Color::from_rgba(245,245, 245, 255),
            selected: false,
            selected_color: Color::from_rgba(255, 255, 255, 255),
            draw_color: WHITE,
        };
        button.draw_color = button.normal_color;
        button
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.eventable.enabled = enabled;
        self.draw_color = if enabled { self.normal_color } else { self.disabled_color }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
        self.draw_color = if selected { self.selected_color } else { self.normal_color };
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    // Convenience method
    pub fn contains_phys_position(&self, phy_position: (f32, f32)) -> bool {
        let transform = self.transform.add(&self.parent_transform);
        self.eventable.contains_phys_position(phy_position, &transform, &self.drawable)
    }

    pub fn process_events(&mut self) -> Option<ButtonEvent> {
        if !self.drawable.visible { return None }

        let transform = self.transform.add(&self.parent_transform);
        let event = self.eventable.process_events(&transform, &self.drawable);

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
                        self.set_selected(!self.selected);
                        Some(ButtonEvent::Toggled(self.id))
                    },
                }
            },
        }
    }

    pub fn draw(&mut self) {
        let transform = self.transform.add(&self.parent_transform);
        self.drawable.draw(&transform, Some(self.draw_color));
    }
}
