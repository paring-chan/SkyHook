use std::time::SystemTime;

#[derive(Debug)]
pub enum EventData {
    KeyPress(u16),
    KeyRelease(u16),
}

#[derive(Debug)]
pub struct Event {
    pub time: SystemTime,
    pub data: EventData,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}
