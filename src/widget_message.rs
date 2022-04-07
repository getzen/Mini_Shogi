/// WidgetMessage
/// The messages sent by Buttons, Sliders, etc
/// The first argument is always the widget id.

pub enum WidgetMessage {
    Pushed(usize),
    Toggled(usize),
    Selected(usize),
    ValueChanged(usize, f32),
}