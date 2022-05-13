/// Button
/// A button that uses a background texture and adds centered text to it.

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum ButtonMode {
    Push,
    Toggle,
}

#[derive(Debug, PartialEq)]
pub enum ButtonEvent {
    /// Normal push-button behavior.
    Pushed(Option<usize>),
    /// Toggle on or off.
    Toggled(Option<usize>),
}

use macroquad::prelude::{Color, WHITE};

use crate::asset_loader::AssetLoader;
use crate::view::*;
use crate::view::draw_text::DrawText;
use crate::view::draw_texture::DrawTexture;
use crate::view::eventable::Event;
use crate::view::eventable::Eventable;
use crate::view::transform::Transform;

// These are AssetLoader names for the button background images for size 0, size 1...
const BUTTON_FILES: [&str; 2] = ["button_0", "button_1"];

pub struct Button {
    pub id: Option<usize>,
    pub group_id: Option<usize>,

    pub transform: Transform,
    pub texture_drawable: DrawTexture,
    pub text_drawable: DrawText,
    pub eventable: Eventable,

    /// Default is Push.
    pub mode: ButtonMode,

    pub button_disabled_color: Color,
    pub button_normal_color: Color,
    pub button_mouse_over_color: Color,
    pub button_selected_color: Color,

    pub text_disabled_color: Color,
    pub text_normal_color: Color,
    pub text_mouse_over_color: Color,
    pub text_selected_color: Color,

    // Private
    selected: bool,
    button_draw_color: Color,
    text_transform: Transform,
    text_draw_color: Color,
}

impl Button {
    pub fn new(logi_position: (f32, f32), size: usize, text: &str, id: Option<usize>) -> Self {
        if size >= BUTTON_FILES.len() {
            panic!("Button size for {} must be less than {}", text, BUTTON_FILES.len());
        }

        let phys_position = phys_pos(logi_position);
        let texture = AssetLoader::get_texture(BUTTON_FILES[size]);

        let mut button = Self {
            id,
            group_id: None,
            transform: Transform::new(phys_position, 0.0),
            texture_drawable: DrawTexture::new(texture, false),
            text_transform: Transform::new((texture.width() / 2.0, texture.height() / 2.0), 0.),
            text_drawable: DrawText::new(
                true, 
                true,
                text, 
                14,
                 Some("Menlo Bold")),
            eventable: Eventable::new(),
            mode: ButtonMode::Push,
            selected: false,

            button_disabled_color: Color::from_rgba(0, 0, 0, 255),
            button_normal_color: Color::from_rgba(70, 70, 70, 255),
            button_mouse_over_color: Color::from_rgba(100, 100, 100, 255),
            button_selected_color: Color::from_rgba(140, 140, 140, 255),
            button_draw_color: WHITE, // set below to normal_color

            text_disabled_color: Color::from_rgba(155, 155, 155, 255),
            text_normal_color: Color::from_rgba(225, 225, 225, 255),
            text_mouse_over_color: Color::from_rgba(240, 240, 240, 255),
            text_selected_color: Color::from_rgba(255, 255, 255, 255),
            text_draw_color: WHITE, // set below to normal_color
        };
        button.button_draw_color = button.button_normal_color;
        button.text_draw_color = button.text_normal_color;
        button
    }

    #[allow(dead_code)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.eventable.enabled = enabled;
        self.button_draw_color = if enabled { self.button_normal_color } else { self.button_disabled_color };
        self.text_draw_color = if enabled { self.text_normal_color } else { self.text_disabled_color };
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
        self.button_draw_color = if selected { self.button_selected_color } else { self.button_normal_color };
        self.text_draw_color = if selected { self.text_selected_color } else { self.text_normal_color };
    }

    #[allow(dead_code)]
    pub fn selected(&self) -> bool {
        self.selected
    }

    // Convenience method
    #[allow(dead_code)]
    pub fn contains_phys_position(&self, phy_position: (f32, f32)) -> bool {
        self.eventable.contains_phys_position(phy_position, &self.transform, &self.texture_drawable)
    }

    pub fn process_events(&mut self) -> Option<ButtonEvent> {
        if !self.texture_drawable.visible { return None }

        let event_opt = self.eventable.process_events(&self.transform, &self.texture_drawable);
        let event = event_opt?;

        match event {
            Event::MouseEntered => {
                if !self.selected {
                    self.button_draw_color = self.button_mouse_over_color;
                    self.text_draw_color = self.text_mouse_over_color;
                }
                None
            },
            Event::MouseExited => {
                if !self.selected {
                    self.button_draw_color = self.button_normal_color;
                    self.text_draw_color = self.text_normal_color;
                }
                None
            },
            Event::LeftMousePressed => {
                self.button_draw_color = self.button_selected_color;
                self.text_draw_color = self.text_selected_color;
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
        self.texture_drawable.draw(&self.transform, Some(self.button_draw_color));

        self.text_transform.set_parent(self.transform.combined());
        self.text_drawable.draw(&self.text_transform, Some(self.text_draw_color));
    }
}
