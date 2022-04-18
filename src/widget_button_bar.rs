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
    pub selected_index: Option<usize>,
    /// If true, then at least one button must be selected, like a radio
    /// button grouping. If false, then all buttons may be unselected and
    /// behavior is like a command bar. Default is false.
    pub radio_behavior: bool,
    pub visible: bool,
}

impl ButtonBar {

    pub fn new(logi_position: (f32, f32), radio_behavior: bool) -> Self {       
        Self {
            phys_position: View::phys_pos(logi_position),
            buttons: Vec::new(),
            margin: 0.0,
            selected_index: None,
            radio_behavior: false,
            visible: true,
        }
    }

    pub fn add_button(&mut self, button: Button) -> usize {
        self.buttons.push(button);
        self.buttons.len()
    }

    pub fn process_events(&mut self) {
        for button in &mut self.buttons {
            if let Some(event) = button.process_events() {
                match event {
                    ButtonEvent::Hovering(_) => todo!(),
                    ButtonEvent::Pushed(_) => todo!(),
                    ButtonEvent::Toggled(_) => todo!(),
                    ButtonEvent::Selected(_) => todo!(),
                }
            }
        }
    }

    pub fn draw(&mut self) {

        let (mut x, y) = self.phys_position;

        for button in &mut self.buttons {
            button.set_logi_position((x,y));
            button.draw();

            x += button.texture.width() + self.margin;
        }
    }
}