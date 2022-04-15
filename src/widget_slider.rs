// Slider

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::View;
use crate::widget_message::WidgetMessage;
use crate::widget_message::WidgetMessage::*;

pub struct Slider {
    pub position: (f32, f32), // left side
    pub width: f32, // the graphic width
    pub value: f32, // the chosen value
    pub min_value: f32,
    pub max_value: f32,
    pub color: Color,
    pub line_thickness: f32,
    pub value_marker_radius: f32, // the current value marker
    pub is_value_marker_solid: bool,
    pub show_ticks: bool, // the min, max, and division ticks
    pub snap_to_tick: bool,
    pub tick_divisions: usize, // # ticks between min and max
    pub tick_height: f32,
    pub is_visible: bool,
    pub id: usize,
    pub tx: Option<Sender<WidgetMessage>>,
    // Private
    is_tracking: bool, // the mouse position
}

impl Slider {
    pub fn new(
        position: (f32, f32), 
        width: f32, 
        value: f32,
        min_value: f32, 
        max_value: f32, 
        id: usize) -> Self {

        Self {
            position, width, value, min_value, max_value, id,
            color: BLACK,
            line_thickness: 2.0,
            value_marker_radius: 8.0,
            is_value_marker_solid: true,
            show_ticks: true,
            tick_divisions: 0,
            tick_height: 12.0,
            snap_to_tick: false,
            is_visible: true,
            tx: None,
            is_tracking: false,
        }
    }

    fn division_width(&self) -> f32 {
        self.width / ((self.tick_divisions + 1) as f32)
    }

    fn mouse_pos_to_value(&self, mouse_pos: (f32, f32)) -> f32 {
        // Convert point to logical units.
        let point = View::logi_pos(mouse_pos);

        let rel_x = point.0 - self.position.0;
        let val = rel_x / self.width * (self.max_value - self.min_value) + self.min_value;
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

    fn contains(&self, point: (f32, f32)) -> bool {
        // Convert point to logical units.
        let pt = View::logi_pos(point);

        pt.0 >= self.position.0 
        && pt.0 <= self.position.0 + self.width
        && pt.1 >= self.position.1 - self.value_marker_radius 
        && pt.1 <= self.position.1 + self.value_marker_radius
    }

    pub fn process_events(&mut self) {
        let mouse_pos = mouse_position();
        let old_value = self.value;
        let mut send_message = false;
        if is_mouse_button_down(MouseButton::Left) {
            if !self.is_tracking && self.contains(mouse_pos) {
                self.is_tracking = true;
            }
            if self.is_tracking {
                self.value = self.mouse_pos_to_value(mouse_pos);
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

        // Convert logical position to physical pixel position.
        let (x, y) = View::phys_pos(self.position);
        let width = self.width * View::dpi_scale();
        let tick_height = self.tick_height * View::dpi_scale();
        let thickness = self.line_thickness * View::dpi_scale();

        // Slider line
        draw_line(x, y, x + width, y, thickness, self.color);

        if self.show_ticks {
            // Min value tick
            draw_line(x, y - tick_height * 0.5, x, y + tick_height * 0.5, thickness, self.color);

            // Max value tick
            draw_line(x + width, y - tick_height * 0.5, x + width, y + tick_height * 0.5, thickness, self.color);

            // Division ticks in between
            if self.tick_divisions > 0 {
                for d in 0..self.tick_divisions {
                    let x = x + self.division_width() * View::dpi_scale() * ((d + 1) as f32);
                    draw_line(x, y - tick_height * 0.5, x, y + tick_height * 0.5, thickness, self.color);
                }
            }
        }

        // Value marker
        let x_ratio = (self.value - self.min_value) / (self.max_value - self.min_value);
        let pt_x = x_ratio * self.width * View::dpi_scale();
        let radius = self.value_marker_radius * View::dpi_scale();
        if self.is_value_marker_solid {
            draw_circle(x + pt_x, y, radius, self.color);
        } else {
            draw_circle_lines(x + pt_x, y, radius, thickness, self.color);
        }
    }
}