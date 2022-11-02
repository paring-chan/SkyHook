use crate::types::{Error, Event};

// ------- LINUX -------

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub fn run(callback: fn(Event)) -> Result<(), Error> {
    linux::start(callback)
}

#[cfg(target_os = "linux")]
pub fn stop() -> Result<(), Error> {
    linux::stop()
}

// ------- WINDOWS -------

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub fn run(callback: fn(Event)) -> Result<(), Error> {
    windows::start(callback)
}

#[cfg(target_os = "windows")]
pub fn stop() -> Result<(), Error> {
    windows::stop()
}
