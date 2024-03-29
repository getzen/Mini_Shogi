// Slider

use macroquad::prelude::*;

use crate::view::transform::Transform;

#[derive(Debug)]
pub enum SliderEvent {
    /// Mouse is over slider. (id)
    Hovering(usize),
    /// Slider value changed. (id, value)
    ValueChanged(usize, f32), // i
}

pub struct Slider {
    /// Use to set position of top-left corner.
    pub transform: Transform,
    /// The widget width.
    pub width: f32,
    /// The thickness of the lines that make up the slider.
    pub line_thickness: f32,
    /// The tick height. Half the tick is above the line, half is below.
    pub tick_height: f32,
    /// The marker radius.
    pub value_marker_radius: f32, // the current value marker
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
    //pub tx: Option<Sender<WidgetMessage>>,

    // Private
    is_tracking: bool, // the mouse position
}

impl Slider {
    /// Creates a new Slider with the given position and width in logical pixels.
    pub fn new(
        position: (f32, f32), 
        width: f32, 
        value: f32,
        min_value: f32, 
        max_value: f32, 
        id: usize) -> Self {

        Self {
            transform: Transform::new(position, 0.0),
            width,
            line_thickness: 1.0,
            tick_height: 10.0,
            value_marker_radius: 10.0,
            is_value_marker_solid: true,
            tick_divisions: 0,
            show_ticks: true,
            snap_to_tick: false,
            color: WHITE,

            value, min_value, max_value,

            is_visible: true,
            id,
            //tx: None,
            is_tracking: false,
        }
    }

    fn division_width(&self) -> f32 {
        self.width / ((self.tick_divisions + 1) as f32)
    }

    fn mouse_position_to_value(&self, position: (f32, f32)) -> f32 {
        let (x, _, _) = self.transform.combined_x_y_rot();
        let rel_x = position.0 - x;
        let val = rel_x / self.width * (self.max_value - self.min_value) + self.min_value;
        val.clamp(self.min_value, self.max_value)
    }

    /// Returns the value the slider would snap to if the mouse is/were released.
    /// Useful for live value tracking to update labels, etc.
    pub fn nearest_snap_value(&mut self) -> f32 {
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
        nearest_value
    }

    pub fn snap_to_nearest_value(&mut self) {
        self.value = self.nearest_snap_value();
    }

    /// Test whether the physical point lies in the slider's area.
    fn contains_point(&self, point: (f32, f32)) -> bool {
        let (x, y, _) = self.transform.combined_x_y_rot();
        point.0 >= x 
        && point.0 <= x + self.width
        && point.1 >= y - self.value_marker_radius 
        && point.1 <= y + self.value_marker_radius
    }

    pub fn process_events(&mut self) -> Option<SliderEvent> {
        let mut event = None;
        if !self.is_visible { return event; };
        
        let mouse_pos = mouse_position();
        let old_value = self.value;
        let mut value_changed = false;

        let hovering = self.contains_point(mouse_pos);
        if hovering {
            event = Some(SliderEvent::Hovering(self.id));
        }

        if is_mouse_button_down(MouseButton::Left) {
            if !self.is_tracking && hovering {
                self.is_tracking = true;
            }
            if self.is_tracking {
                self.value = self.mouse_position_to_value(mouse_pos);
                value_changed = self.value != old_value && !self.snap_to_tick;
            }
        } else if self.is_tracking {
            self.is_tracking = false;
            if self.snap_to_tick {
                self.snap_to_nearest_value();
                value_changed = self.value != old_value;
            }
        }

        if value_changed {
            event = Some(SliderEvent::ValueChanged(self.id, self.value));
        }
        event
    }

    pub fn draw(&self) {
        if !self.is_visible { return; }

        let (x, y, _) = self.transform.combined_x_y_rot();

        // Slider line
        draw_line(x, y, x + self.width, y, self.line_thickness, self.color);

        if self.show_ticks {
            // Min value tick
            draw_line(
                x, 
                y - self.tick_height * 0.5, 
                x, y + self.tick_height * 0.5, 
                self.line_thickness, 
                self.color);

            // Max value tick
            draw_line(
                x + self.width, 
                y - self.tick_height * 0.5, 
                x + self.width, 
                y + self.tick_height * 0.5, 
                self.line_thickness, 
                self.color);

            // Division ticks in between
            if self.tick_divisions > 0 {
                for d in 0..self.tick_divisions {
                    let x = x + self.division_width() * ((d + 1) as f32);
                    draw_line(
                        x, 
                        y - self.tick_height * 0.5, 
                        x, 
                        y + self.tick_height * 0.5, 
                        self.line_thickness,
                        self.color);
                }
            }
        }

        // Value marker
        let x_ratio = (self.value - self.min_value) / (self.max_value - self.min_value);
        let pt_x = x_ratio * self.width;
        if self.is_value_marker_solid {
            draw_circle(x + pt_x, y, self.value_marker_radius, self.color);
        } else {
            draw_circle_lines(x + pt_x, y, self.value_marker_radius, self.line_thickness, self.color);
        }
    }
}