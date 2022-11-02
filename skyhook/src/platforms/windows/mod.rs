use crate::types::Event;

mod hook;

pub fn start(callback: fn(Event)) -> Result<(), crate::types::Error> {
    hook::start(callback)
}

pub fn stop() -> Result<(), crate::types::Error> {
    hook::stop()
}
