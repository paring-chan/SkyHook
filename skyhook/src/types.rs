#[derive(Debug)]
pub enum Event {
    KeyPress(u16),
    KeyRelease(u16),
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
}
