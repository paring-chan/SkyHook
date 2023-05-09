use crate::types::{Error, Event};

mod inputdev;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    inputdev::start(callback)
}

#[allow(dead_code)]
pub fn stop() -> Result<(), Error> {
    inputdev::stop()
}
