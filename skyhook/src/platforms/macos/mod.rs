use crate::types::{Error, Event};

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    Ok(())
}

pub fn stop() -> Result<(), Error> {
    Ok(())
}
