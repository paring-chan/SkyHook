use crate::types::{Error, Event};

pub fn start(#[allow(unused_variables)] callback: fn(Event)) -> Result<(), Error> {
    Err(Error {
        message: "Not yet implemented".into(),
    })
}

pub fn stop() -> Result<(), Error> {
    Err(Error {
        message: "Not yet implemented".into(),
    })
}
