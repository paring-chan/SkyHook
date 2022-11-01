use types::{Error, Event};

extern crate cancellation;

mod platforms;
pub mod types;

pub fn run(callback: fn(Event)) -> Result<(), Error> {
    platforms::run(callback)
}

pub fn stop() {
    platforms::stop()
}
