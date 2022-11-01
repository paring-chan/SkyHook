// ------- LINUX -------

#[cfg(target_os = "linux")]
pub mod linux;

use crate::types::{Error, Event};

#[cfg(target_os = "linux")]
pub fn run(callback: fn(Event)) -> Result<(), Error> {
    linux::start(callback)
}

#[cfg(target_os = "linux")]
pub fn stop() {
    linux::stop()
}

// ------- WINDOWS -------

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub fn run(callback: fn(Event)) {} // TODO

#[cfg(target_os = "windows")]
pub fn stop() {
    linux::stop()
}
