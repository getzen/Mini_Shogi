/// ButtonBar
/// Displays a row of buttons side-by-side, similar to a menu bar,
/// but without drop-down, sub-menu capability. Only one button may
/// be active at a time.

use macroquad::prelude::Color;

use crate::view::*;
use crate::view::button::Button;
use crate::view::button::ButtonEvent;

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
            phys_position: phys_pos(logi_position),
            buttons: Vec::new(),
            margin: 50.0,
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
        button.id = Some(index);
        self.buttons.push(button);
        index
    }

    pub fn unselect_all(&mut self) {
        for button in &mut self.buttons {
            button.selected = false;
        }
        self.selected_id = None;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        for button in &mut self.buttons {
            button.eventable.enabled = enabled;
        }
        if enabled && self.selected_id.is_some() {
            self.buttons[self.selected_id.unwrap()].selected = true;
        }
    }

    pub fn process_events(&mut self) -> Option<usize> {
        if !self.visible || !self.enabled { return None }

        if let Some(old_id) = self.selected_id {
            self.buttons[old_id].selected = false;
        }
        
        let mut button_selected_opt = None;

        for button in &mut self.buttons {
            if let Some(event) = button.process_events() {
                match event {
                    ButtonEvent::Hovering(id) => {
                        button_selected_opt = Some(button);
                        break;
                    },

                    ButtonEvent::Pushed(id) => {
                        // if let Some(old_id) = self.selected_id {
                        //     if old_id != id.unwrap() {
                        //         self.buttons[old_id].selected = false;
                        //         self.selected_id = id;
                        //         return id;
                        //     }
                        // }
                        // //
                        // self.selected_id = id;
                        // button.selected = true;
                        //return id;
                    },
                    ButtonEvent::Toggled(_) => {},
                    //ButtonEvent::Selected(_) => {},
                }
            }
        }
        if let Some(button_selected) = button_selected_opt {           
            button_selected.selected = true;
            self.selected_id = button_selected.id;
            return None;
        }
        None
    }

    pub fn draw(&mut self) {
        if !self.visible { return }

        let (mut x, y) = self.phys_position;

        for button in &mut self.buttons {
            button.transform.phys_position = (x, y);
            button.draw();

            let tex_width = button.drawable.texture.width();
            x += tex_width + self.margin;
        }
    }
}