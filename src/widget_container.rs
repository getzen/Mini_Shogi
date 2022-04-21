/// Container
/// Holds other widgets and positions those widgets relative to itself.
/// During process_events(), the first event recieved is propogated up.

use crate::View;
use crate::widget_button::{Button, ButtonEvent};
use crate::widget_button_bar::ButtonBar;
use crate::widget_slider::{Slider, SliderEvent};

pub struct Container {
    /// Position in physical pixels of the top-left corner.
    /// Use set_logi_position for logical pixel positioning.
    pub phys_position: (f32, f32),
    pub visible: bool,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,

    // Private
    enabled: bool,
}

impl Container {
    pub fn new(logi_position: (f32, f32)) -> Self {       
        Self {
            phys_position: View::phys_pos(logi_position),
            visible: true,
            enabled: true,
            buttons: Vec::new(),
            sliders: Vec::new(),
        }
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        for button in &mut self.buttons {
            button.is_enabled = enabled;
        }

        for slider in &mut self.sliders {
            //
        }
    }

    pub fn process_button_events(&mut self) -> Option<ButtonEvent> {
        None
    }

    pub fn process_slider_events(&mut self) -> Option<SliderEvent> {
        None
    }

    pub fn draw(&mut self) {
        for button in &mut self.buttons {
            button.draw();
        }

        for slider in &mut self.sliders {
            slider.draw();
        }
    }
}
