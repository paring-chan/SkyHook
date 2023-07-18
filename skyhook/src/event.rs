use crate::KeyCode;

#[derive(Debug)]
pub enum Event {
    KeyDown(KeyCode, i32),
    KeyUp(KeyCode, i32),
}
