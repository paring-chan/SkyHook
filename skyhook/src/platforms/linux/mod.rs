use std::{fs, sync::Arc, thread};

use cancellation::CancellationTokenSource;

use crate::types::{Error, Event};

use self::reader::start_reader;

mod reader;

pub static mut CANCELLATION_TOKEN: Option<Arc<CancellationTokenSource>> = None;

pub static mut STARTED: bool = false;
pub static mut ERROR: Option<Error> = None;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    if unsafe { STARTED } {
        return Err(Error {
            message: "Hook is already started".into(),
        });
    }

    let dir = match fs::read_dir("/dev/input") {
        Ok(dir) => dir,
        Err(err) => {
            return Err(Error {
                message: format!("Failed to read input dir: {:?}", err),
            });
        }
    };

    let mut exists = false;

    let cts = CancellationTokenSource::new();

    unsafe {
        CANCELLATION_TOKEN = Some(Arc::new(cts));
    }

    for path in dir {
        let filename = path.expect("Failed to get dir entry").file_name();
        let filename: String = match filename.to_str() {
            Some(v) => v.into(),
            None => continue,
        };

        if filename.starts_with("event") {
            exists = true;
            if let Err(err) = thread::Builder::new()
                .name("SkyHook".into())
                .spawn(move || {
                    if let Err(err) = start_reader(format!("/dev/input/{}", filename), callback) {
                        println!("{:?}", err);
                        unsafe {
                            ERROR = Some(err);
                        }

                        if let Err(_) = stop() {}
                    }
                })
            {
                return Err(Error {
                    message: format!("Failed to spawn thread: {:?}", err),
                });
            }
        }
    }

    if !exists {
        return Err(Error {
            message: "Cannot find any event stream".into(),
        });
    }

    unsafe {
        STARTED = true;
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
            STARTED = false;
        }

        return Ok(());
    }

    Err(Error {
        message: "Hook cannot be stopped before starting.".into(),
    })
}
