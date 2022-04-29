// Button2

use macroquad::prelude::*;

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

use macroquad::prelude::{Color, WHITE, TextDimensions};
use macroquad::prelude::Texture2D;

use crate::view::draw_texture::DrawTexture;
use crate::view::eventable::Event;
use crate::view::eventable::Eventable;
use crate::view::transform::Transform;
use crate::view::*;
use super::label::Label;

const FONT: &str = "Menlo";
const FONT_SIZE: u16 = 18;
const LABEL_MARGIN: u32 = 40;
const BUTTON_HEIGHT: u32 = 64;
const BUTTON_CORNER_RADIUS: f32 = 20.;
const BUTTON_COLOR: Color = WHITE;

pub struct Button2 {
    pub id: Option<usize>,
    pub group_id: Option<usize>,
    pub transform: Transform,
    pub drawable: DrawTexture,
    pub label: Label,
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

impl Button2 {
    pub fn new(logi_position: (f32, f32), text: &str, id: Option<usize>) -> Self {
        let phys_position = phys_pos(logi_position);

        let texture = Texture2D::empty();

        let mut button = Self {
            id,
            group_id: None,
            transform: Transform::new(phys_position, 0.0),
            drawable: DrawTexture::new(texture, false),
            label: Label::new((0., 0.), true, text.to_owned(), FONT_SIZE, Some(FONT)),
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

    pub fn draw_to_texture(&mut self, draw_fn: fn(u32, u32), phys_width: u32, phys_height: u32) -> Texture2D {
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

    // Convenience method
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
                        self.set_selected(!self.selected);
                        Some(ButtonEvent::Toggled(self.id))
                    },
                }
            },
        }
    }

    pub fn draw(&mut self) {
        let label_width = self.label.width();
        let button_width = label_width as u32 + LABEL_MARGIN * 2;
        let texture = self.draw_to_texture(draw_rounded_button, button_width,BUTTON_HEIGHT);
        self.drawable.set_texture(texture);
        self.drawable.draw(&self.transform, None);

        

        let (x, y) = self.label.center();
        self.label.transform.phys_position.0 = self.transform.phys_position.0 + self.drawable.texture.width() / 2.0;
        self.label.transform.phys_position.1 = self.transform.phys_position.1 + self.drawable.texture.height() / 2.0 + y;
        self.label.draw();
    }
}

fn draw_rounded_button(phys_width: u32, phys_height: u32) {
    let width = phys_width as f32;
    let height = phys_height as f32;
    
    clear_background(Color::from_rgba(255, 255, 255, 0));

    // Draw the four corner circles.
    let mut x = BUTTON_CORNER_RADIUS;
    let mut y = BUTTON_CORNER_RADIUS;
    draw_circle(x, y, BUTTON_CORNER_RADIUS, BUTTON_COLOR);
    x = width - BUTTON_CORNER_RADIUS;
    draw_circle(x, y, BUTTON_CORNER_RADIUS, BUTTON_COLOR);
    y = height - BUTTON_CORNER_RADIUS;
    draw_circle(x, y, BUTTON_CORNER_RADIUS, BUTTON_COLOR);
    x = BUTTON_CORNER_RADIUS;
    draw_circle(x, y, BUTTON_CORNER_RADIUS, BUTTON_COLOR);

    // Draw the top-to-bottom rect.
    //draw_rectangle(BUTTON_CORNER_RADIUS, 0., width - BUTTON_CORNER_RADIUS * 2., height, BUTTON_COLOR);

    // Draw the left-to-right rect.
    //draw_rectangle(0., BUTTON_CORNER_RADIUS, width, height- BUTTON_CORNER_RADIUS * 2., BUTTON_COLOR);
}

