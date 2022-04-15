// Slider

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::View;
use crate::widget_message::WidgetMessage;
use crate::widget_message::WidgetMessage::*;

pub struct Slider {
    /// Position in physical pixels of the left-center.
    /// Use set_logi_position for logical pixel positioning.
    pub phys_position: (f32, f32),
    /// The widget width in physical pixels.
    pub phys_width: f32,
    /// The thickness in physical pixels of the lines that make up the slider.
    pub phys_line_thickness: f32,
    /// The tick height in physical pixels. Half the tick is above
    /// the line, half is below.
    pub phys_tick_height: f32,
    /// The marker radius in physical pixels.
    pub phys_value_marker_radius: f32, // the current value marker
    /// If true, the marker is a solid circle, otherwise an outline.
    pub is_value_marker_solid: bool,
    /// The number of ticks betweeen mix and max values.
    pub tick_divisions: usize,
    /// If true, the min, max, and division tick marks will be drawn.
    pub show_ticks: bool,
    /// If true, the value will snap to the division ticks when the
    /// mouse is released.
    pub snap_to_tick: bool,
    pub color: Color,

    /// The current value.
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,

    pub is_visible: bool,
    pub id: usize,
    pub tx: Option<Sender<WidgetMessage>>,
    
    // Private
    is_tracking: bool, // the mouse position
}

impl Slider {
    /// Creates a new Slider with the given position and width in logical pixels.
    pub fn new(
        logi_position: (f32, f32), 
        width: f32, 
        value: f32,
        min_value: f32, 
        max_value: f32, 
        id: usize) -> Self {

        Self {
            phys_position: View::phys_pos(logi_position),
            phys_width: width * View::dpi_scale(),
            phys_line_thickness: 1.0 * View::dpi_scale(),
            phys_tick_height: 10.0 * View::dpi_scale(),
            phys_value_marker_radius: 10.0 * View::dpi_scale(),
            is_value_marker_solid: true,
            tick_divisions: 0,
            show_ticks: true,
            snap_to_tick: false,
            color: BLACK,

            value, min_value, max_value,

            is_visible: true,
            id,
            tx: None,
            is_tracking: false,
        }
    }

    #[allow(dead_code)]
    /// Get the logical position of the sprite.
    pub fn get_logi_position(&self) -> (f32, f32) {
        View::logi_pos(self.phys_position)
    }

    #[allow(dead_code)]
    /// Set the logical position of the sprite.
    pub fn set_logi_position(&mut self, logi_position: (f32, f32)) {
        self.phys_position = View::phys_pos(logi_position);
    }

    fn division_width(&self) -> f32 {
        self.phys_width / ((self.tick_divisions + 1) as f32)
    }

    fn mouse_position_to_value(&self, position: (f32, f32)) -> f32 {
        let rel_x = position.0 - self.phys_position.0;
        let val = rel_x / self.phys_width * (self.max_value - self.min_value) + self.min_value;
        val.clamp(self.min_value, self.max_value)
    }

    fn snap_to_nearest_value(&mut self,) {
        let mut nearest_distance = f32::MAX;
        let mut nearest_value = self.min_value;
        let mut test_value = self.min_value;

        while test_value <= self.max_value {
            let d = (self.value - test_value).abs();
            if d < nearest_distance {
                nearest_distance = d;
                nearest_value = test_value;
            }
            test_value += (self.max_value - self.min_value) / (self.tick_divisions + 1) as f32;
        }
        self.value = nearest_value;
    }

    /// Test whether the physical point lies in the slider's area.
    fn contains_phys_position(&self, phys_position: (f32, f32)) -> bool {
        phys_position.0 >= self.phys_position.0 
        && phys_position.0 <= self.phys_position.0 + self.phys_width
        && phys_position.1 >= self.phys_position.1 - self.phys_value_marker_radius 
        && phys_position.1 <= self.phys_position.1 + self.phys_value_marker_radius
    }

    pub fn process_events(&mut self) {
        let mouse_pos = mouse_position();
        let old_value = self.value;
        let mut send_message = false;
        if is_mouse_button_down(MouseButton::Left) {
            if !self.is_tracking && self.contains_phys_position(mouse_pos) {
                self.is_tracking = true;
            }
            if self.is_tracking {
                self.value = self.mouse_position_to_value(mouse_pos);
                send_message = self.value != old_value && !self.snap_to_tick;
            }
        } else if self.is_tracking {
            self.is_tracking = false;
            if self.snap_to_tick {
                self.snap_to_nearest_value();
                send_message = self.value != old_value;
            }
        }
        if send_message {
            if let Some(sender) = &self.tx {
                sender.send(ValueChanged(self.id, self.value)).expect("Button message send error.");
            }
        }
    }

    pub fn draw(&self) {
        if !self.is_visible { return; }

        let (x, y) = self.phys_position;

        // Slider line
        draw_line(x, y, x + self.phys_width, y, self.phys_line_thickness, self.color);

        if self.show_ticks {
            // Min value tick
            draw_line(
                x, 
                y - self.phys_tick_height * 0.5, 
                x, y + self.phys_tick_height * 0.5, 
                self.phys_line_thickness, 
                self.color);

            // Max value tick
            draw_line(
                x + self.phys_width, 
                y - self.phys_tick_height * 0.5, 
                x + self.phys_width, 
                y + self.phys_tick_height * 0.5, 
                self.phys_line_thickness, 
                self.color);

            // Division ticks in between
            if self.tick_divisions > 0 {
                for d in 0..self.tick_divisions {
                    let x = x + self.division_width() * ((d + 1) as f32);
                    draw_line(
                        x, 
                        y - self.phys_tick_height * 0.5, 
                        x, 
                        y + self.phys_tick_height * 0.5, 
                        self.phys_line_thickness,
                        self.color);
                }
            }
        }

        // Value marker
        let x_ratio = (self.value - self.min_value) / (self.max_value - self.min_value);
        let pt_x = x_ratio * self.phys_width;
        if self.is_value_marker_solid {
            draw_circle(x + pt_x, y, self.phys_value_marker_radius, self.color);
        } else {
            draw_circle_lines(x + pt_x, y, self.phys_value_marker_radius, self.phys_line_thickness, self.color);
        }
    }
}