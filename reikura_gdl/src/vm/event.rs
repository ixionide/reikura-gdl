use crate::input::Button;

pub enum WindowEvent {
    Button { button: Button, down: bool },
    Close,
    CursorMove { x: i32, y: i32 },
    CursorLeave,
    Focused(bool),
}
