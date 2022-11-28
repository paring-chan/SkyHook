use types::{Error, Event};

#[cfg(target_os = "linux")]
extern crate cancellation;

#[cfg(target_os = "windows")]
extern crate winsafe;

extern crate chrono;

pub mod keycodes;
pub(crate) mod macros;
mod platforms;
pub mod types;

pub fn run(callback: fn(Event)) -> Result<(), Error> {
    platforms::run(callback)
}

pub fn stop() -> Result<(), Error> {
    platforms::stop()
}
