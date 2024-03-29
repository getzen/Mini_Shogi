/// ButtonBar
/// Displays a row of buttons side-by-side, similar to a menu bar,
/// but without drop-down, sub-menu capability. Only one button may
/// be active at a time.

use crate::view::*;
use crate::view::button::ButtonEvent;
use crate::view::button::Button;
use crate::view::transform::Transform;

#[allow(dead_code)]
pub enum ButtonBarOrientation {
    Horizontal,
    Vertical,
}

pub struct ButtonBar {
    /// Use to set physical position of top-left corner.
    pub transform: Transform,
    pub orientation: ButtonBarOrientation,
    pub spacing: f32,
    pub buttons: Vec<Button>,
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

    pub fn new(position: (f32, f32), orientation: ButtonBarOrientation, spacing: f32) -> Self {       
        Self {
            transform: Transform::new(position, 0.0),
            orientation,
            spacing,
            buttons: Vec::new(),
            selected_id: None,
            radio_behavior: false,
            visible: true,
            enabled: true,
        }
    }

    // /// Convenience method to set the color of all buttons at once.
    // pub fn set_color(&mut self, color: Color) {
    //     for button in &mut self.buttons {
    //         button.normal_color = color;
    //     }
    // }

    // /// Convenience method to set the selected color of all buttons at once.
    // pub fn set_selected_color(&mut self, color: Color) {
    //     for button in &mut self.buttons {
    //         button.selected_color = color;
    //     }
    // }

    pub fn add_button(&mut self, mut button: Button) -> usize {
        let mut id = self.buttons.len();
        if button.id.is_none() {
            button.id = Some(id);
        } else {
            id = button.id.unwrap();
        }
        self.buttons.push(button);
        id
    }

    pub fn select_only(&mut self, id: usize) {
        for button in &mut self.buttons {
            button.set_selected(id == button.id.unwrap());
        }
    }

    #[allow(dead_code)]
    pub fn unselect_all(&mut self) {
        for button in &mut self.buttons {
            button.set_selected(false);
        }
        self.selected_id = None;
    }

    #[allow(dead_code)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        for button in &mut self.buttons {
            button.set_enabled(enabled);
        }
        if enabled && self.selected_id.is_some() {
            self.buttons[self.selected_id.unwrap()].set_selected(true);
        }
    }

    pub fn process_events(&mut self) -> Option<usize> {
        if !self.visible || !self.enabled { return None }

        for button in &mut self.buttons {
            if let Some(event) = button.process_events() {
                
                if self.radio_behavior {
                    todo!("add this behavior");
                }

                match event {
                    ButtonEvent::Pushed(id) => {
                        return id;
                    },
                    ButtonEvent::Toggled(id) => {
                        return id;
                    },
                }
            }
        }
        None
    }

    pub fn draw(&mut self) {
        if !self.visible { return }

        let (mut x, mut y) = (0.0, 0.0);

        for button in &mut self.buttons {
            button.transform.set_parent(self.transform.combined());
            button.transform.position.0 = x;
            button.transform.position.1 = y;
            button.draw();

            match self.orientation {
                ButtonBarOrientation::Horizontal => x += button.texture_drawable.size.0 + self.spacing,
                ButtonBarOrientation::Vertical => y += button.texture_drawable.size.1 + self.spacing,
            }
        }
    }
}