use types::{Error, Event};

#[cfg(target_os = "linux")]
extern crate cancellation;
extern crate winsafe;

mod platforms;
pub mod types;

pub fn run(callback: fn(Event)) -> Result<(), Error> {
    platforms::run(callback)
}

pub fn stop() {
    platforms::stop()
}
