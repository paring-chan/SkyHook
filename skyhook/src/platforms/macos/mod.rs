use std::thread;

use core_foundation::runloop::CFRunLoopStop;

use crate::types::{Error, Event};

use self::listen::RUN_LOOP;

mod common;
mod listen;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    if let Err(e) = thread::Builder::new()
        .name("SkyHook Listener Thread".into())
        .spawn(move || listen::listen(callback))
    {
        return Err(Error {
            message: format!("Failed to start listener thread: {:?}", e),
        });
    }

    while let None = unsafe { &RUN_LOOP } {}

    Ok(())
}

pub fn stop() -> Result<(), Error> {
    unsafe {
        if let Some(rl) = RUN_LOOP {
            CFRunLoopStop(rl);
        } else {
            return Err(Error {
                message: "Loop is not started".into(),
            });
        }
    }

    Ok(())
}
