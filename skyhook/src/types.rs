use std::time::SystemTime;

use crate::keycodes::VK;

#[derive(Debug)]
pub enum EventData {
    KeyPress(VK, u16),
    KeyRelease(VK, u16),
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
