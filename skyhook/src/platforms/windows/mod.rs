use cancellation::CancellationTokenSource;

use crate::types::Event;

mod hook;
mod message;

pub static mut CANCELLATION_TOKEN: Option<CancellationTokenSource> = None;

pub fn start(callback: fn(Event)) -> Result<(), crate::types::Error> {
    unsafe {
        CANCELLATION_TOKEN = Some(CancellationTokenSource::new());
    }

    hook::start(callback)
}

pub fn stop() -> Result<(), crate::types::Error> {
    hook::stop()?;
    unsafe {
        if let Some(token) = &CANCELLATION_TOKEN {
            token.cancel();
        }

        CANCELLATION_TOKEN = None;
    }

    Ok(())
}
