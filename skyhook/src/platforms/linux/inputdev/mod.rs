use std::{
    ffi::CString,
    fs,
    os::unix::prelude::PermissionsExt,
    sync::{atomic::AtomicUsize, Arc},
    thread,
};

use cancellation::CancellationTokenSource;
use libc::{c_char, AT_EACCESS, AT_FDCWD, R_OK};

use crate::types::{Error, Event};

use self::reader::start_reader;

mod keycode;
mod reader;

pub static mut CANCELLATION_TOKEN: Option<Arc<CancellationTokenSource>> = None;

pub static mut STARTED: bool = false;
pub static mut ERROR: Option<Error> = None;
pub static mut ARC_READY_COUNT: Option<Arc<AtomicUsize>> = None;

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
        ARC_READY_COUNT = Some(Arc::new(AtomicUsize::new(0)));
    }

    let mut count = 0;

    for path in dir {
        let filename = path.expect("Failed to get dir entry").file_name();
        let filename: String = match filename.to_str() {
            Some(v) => v.into(),
            None => continue,
        };

        if filename.starts_with("event") {
            exists = true;

            let meta = match fs::metadata(format!("/dev/input/{}", filename)) {
                Ok(v) => v,
                Err(err) => {
                    return Err(Error {
                        message: format!("Unable to get permissions: {:?}", err),
                    });
                }
            };

            let c_path = CString::new(format!("/dev/input/{}", filename)).unwrap();

            unsafe {
                if libc::faccessat(
                    AT_FDCWD,
                    c_path.as_ptr() as *const c_char,
                    0 | R_OK,
                    AT_EACCESS,
                ) != 0
                {
                    return Err(Error {
                        message: format!("You have no permission to read /dev/input/{}", filename),
                    });
                }
            }

            let permissions = meta.permissions();

            println!("{:?}", permissions.mode());

            if let Err(err) = thread::Builder::new()
                .name("SkyHook".into())
                .spawn(move || {
                    if let Err(err) = start_reader(format!("/dev/input/{}", filename), callback) {
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

            count += 1;
        }
    }

    if !exists {
        return Err(Error {
            message: "Cannot find any event stream".into(),
        });
    }

    while unsafe {
        ERROR.is_none()
            && !{
                if let Some(arc) = &ARC_READY_COUNT {
                    arc.load(std::sync::atomic::Ordering::Relaxed) < count
                } else {
                    false
                }
            }
    } {
        thread::yield_now();
    }

    if let Some(err) = unsafe { &ERROR } {
        return Err(err.clone());
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

pub fn is_running() -> bool {
    unsafe { STARTED }
}
