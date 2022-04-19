use macroquad::prelude::Color;

/// ButtonBar
/// Displays a row of buttons side-by-side, similar to a menu bar,
/// but without drop-down, sub-menu capability. Only one button may
/// be active at a time.

use crate::View;
use crate::widget_button::{Button, ButtonEvent};

// pub enum ButtonBarMessage {
//     Pushed(usize), // button index/id
// }

pub struct ButtonBar {
    /// Position in physical pixels of the top-left corner.
    /// Use set_logi_position for logical pixel positioning.
    pub phys_position: (f32, f32),
    pub buttons: Vec<Button>,
    pub margin: f32,
    pub selected_id: Option<usize>,
    /// If true, then at least one button must be selected, like a radio
    /// button grouping. If false, then all buttons may be unselected and
    /// behavior is like a command bar. Default is false.
    pub radio_behavior: bool,
    pub visible: bool,

    // Private
    enabled: bool,
}

impl ButtonBar {

    pub fn new(logi_position: (f32, f32), radio_behavior: bool) -> Self {       
        Self {
            phys_position: View::phys_pos(logi_position),
            buttons: Vec::new(),
            margin: 0.0,
            selected_id: None,
            radio_behavior: false,
            visible: true,
            enabled: true,
        }
    }

    /// Convenience method to set the color of all buttons at once.
    pub fn set_color(&mut self, color: Color) {
        for button in &mut self.buttons {
            button.color = color;
        }
    }

    /// Convenience method to set the selected color of all buttons at once.
    pub fn set_selected_color(&mut self, color: Color) {
        for button in &mut self.buttons {
            button.selected_color = Some(color);
        }
    }

    pub fn add_button(&mut self, mut button: Button) -> usize {
        let index = self.buttons.len();
        button.id = index;
        self.buttons.push(button);
        index
    }

    pub fn unselect_all(&mut self) {
        for button in &mut self.buttons {
            button.is_selected = false;
        }
        self.selected_id = None;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        for button in &mut self.buttons {
            button.is_enabled = enabled;
        }
        if enabled && self.selected_id.is_some() {
            self.buttons[self.selected_id.unwrap()].is_selected = true;
        }
    }

    pub fn process_events(&mut self) -> Option<usize> {
        if !self.visible || !self.enabled { return None }

        for button in &mut self.buttons {
            if let Some(event) = button.process_events() {
                match event {
                    ButtonEvent::Pushed(id) => {
                        if let Some(old_id) = self.selected_id {
                            if old_id != id {
                                self.buttons[old_id].is_selected = false;
                                self.selected_id = Some(id);
                                return Some(id);
                            }
                        }
                        //
                        self.selected_id = Some(id);
                        return Some(id);
                    },
                    ButtonEvent::Hovering(_) => {},
                    ButtonEvent::Toggled(_) => {},
                    ButtonEvent::Selected(_) => {},
                }
            }
        }
        None
    }

    pub fn draw(&mut self) {
        if !self.visible { return }

        let (mut x, y) = self.phys_position;

        for button in &mut self.buttons {
            button.set_logi_position((x, y));
            button.draw();

            let tex_width = button.texture.width() / View::dpi_scale();
            x += tex_width + self.margin;
        }
    }
}