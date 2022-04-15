// Button
// The code is similar to Sprite, but with button-specific features.
// Also, the position refers to the top-left corner of the texture instead
// of the center.

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::View;
use crate::widget_message::WidgetMessage;
use crate::widget_message::WidgetMessage::*;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum ButtonMode {
    Push,
    Toggle,
    Radio,
}

pub struct Button {
    pub position: (f32, f32), // top-left corner

    pub texture: Texture2D,
    pub disabled_texture: Option<Texture2D>,
    pub selected_texture: Option<Texture2D>,

    pub color: Color,
    pub disabled_color: Option<Color>,
    pub selected_color: Option<Color>,

    pub draw_params: DrawTextureParams,
    pub z_order: usize, // default 0
    pub mode: ButtonMode,

    pub is_visible: bool,
    pub is_enabled: bool,
    pub is_mouse_over: bool,
    pub is_selected: bool,

    pub id: usize,
    pub group_id: usize, // for radio-style groups
    pub tx: Option<Sender<WidgetMessage>>,
}

impl Button {
    pub fn new(
        position: (f32, f32), 
        texture: Texture2D, 
        mode: ButtonMode,
        id: usize) -> Self {

        // Adjust texture draw size based on the dpi scale.
        let mut params = DrawTextureParams::default();
        let size_x = texture.width() * View::adj_scale();
        let size_y = texture.height() * View::adj_scale();
        params.dest_size = Some(Vec2::new(size_x, size_y));

        Self {
            position, texture, mode, id,
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
            tx: None,
        }
    }

    // #[allow(dead_code)]
    // pub fn set_scale(&mut self, scale: f32) {
    //     let dest_size = Vec2::new(
    //         self.texture.width() * scale, 
    //         self.texture.height() * scale);
    //     self.draw_params.dest_size = Some(dest_size);
    // }

    #[allow(dead_code)]
    /// Test whether the given point lies in the texture rectangle, considering rotation.
    pub fn contains(&self, mut point: (f32, f32)) -> bool {
        // Convert point to logical units.
        point = View::logi_pos(point);

        let (w, h) = self.logical_size();

        // Get the net test point relative to the sprite's position.
        let net_x = point.0 - self.position.0 - w / 2.0;
        let net_y = point.1 - self.position.1 - h / 2.0;
        // Rotate the point clockwise (the same direction as Macroquad's rotation).
        let theta = self.draw_params.rotation;
        let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
        let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);
        // See if the rotated point is in the unrotated sprite rectangle.
        f32::abs(rot_x) < w / 2.0 && f32::abs(rot_y) < h / 2.0
    }

     /// Returns the size of button in logical units.
     fn logical_size(&self) -> (f32, f32) {
        let mut width = self.texture.width() / View::dpi_scale();
        let mut height = self.texture.height() / View::dpi_scale();
        if let Some(dest_size) = self.draw_params.dest_size {
            width = dest_size.x / View::dpi_scale();
            height = dest_size.y / View::dpi_scale();
        }
        (width, height)
    }

    pub fn process_events(&mut self) {
        if !self.is_visible || !self.is_enabled { return; }
        self.is_mouse_over = self.contains(mouse_position());
        let button_pressed = is_mouse_button_down(MouseButton::Left);
        // See if button was released *this frame*.
        let button_released = is_mouse_button_released(MouseButton::Left);

        match &self.mode {
            ButtonMode::Push => {
                self.is_selected = self.is_mouse_over && button_pressed;
                if self.is_mouse_over && button_released {
                    if let Some(sender) = &self.tx {
                        sender.send(Pushed(self.id)).expect("Button message send error.");
                    }
                    self.is_selected = false;
                }
            },
            ButtonMode::Toggle => {
                if self.is_mouse_over && button_released {
                    self.is_selected = !self.is_selected;
                    if let Some(sender) = &self.tx {
                        sender.send(Toggled(self.id)).expect("Button message send error.");
                    }
                }
            },
            ButtonMode::Radio => {
                if self.is_mouse_over && button_released {
                    if !self.is_selected {
                        if let Some(sender) = &self.tx {
                            sender.send(Selected(self.id)).expect("Button message send error.");
                        }
                    }
                    self.is_selected = true;
                }
            },
        }
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
        let (x, y) = View::phys_pos(self.position);
        draw_texture_ex(draw_texture, x, y, draw_color, self.draw_params.clone());
    }
}