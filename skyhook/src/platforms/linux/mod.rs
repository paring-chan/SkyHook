use std::{fs, sync::Arc, thread};

use cancellation::CancellationTokenSource;

use crate::types::{Error, Event};

use self::reader::start_reader;

mod reader;

pub static mut CANCELLATION_TOKEN: Option<Arc<CancellationTokenSource>> = None;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    let dir = fs::read_dir("/dev/input").expect("Failed to read /dev/input");

    for path in dir {
        let filename = path.expect("Failed to get dir entry").file_name();
        let filename = match filename.to_str() {
            Some(v) => v.into(),
            None => continue,
        };

        let cts = CancellationTokenSource::new();

        unsafe {
            CANCELLATION_TOKEN = Some(Arc::new(cts));
        }

        if filename.starts_with("event") {
            if let Err(err) = thread::Builder::new()
                .name("SkyHook".into())
                .spawn(move || start_reader(format!("/dev/input/{}", filename), callback))
            {
                return Err(Error {
                    message: format!("Failed to spawn thread: {:?}", err),
                });
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn stop() -> Result<(), Error> {
    let token = unsafe { &CANCELLATION_TOKEN };

    if let Some(token) = token {
        token.cancel();

        unsafe {
            CANCELLATION_TOKEN = None;
        }

        return Ok(());
    }

    Err(Error {
        message: "Hook cannot be stopped before starting.".into(),
    })
}
