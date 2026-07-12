use crate::input::Button;

pub enum WindowEvent {
    Close,
    CursorMove { x: i32, y: i32 },
    CursorLeave,
    Button { button: Button, down: bool },
}
