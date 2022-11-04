use std::ffi::{c_char, CStr};

use crate::types::{Error, Event};

extern "C" {
    fn start_macos_hook() -> *const c_char;
    fn stop_macos_hook() -> *const c_char;
}

pub fn start(#[allow(unused_variables)] callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        let result = start_macos_hook();

        if result.is_null() {
            return Ok(());
        }

        Err(Error {
            message: CStr::from_ptr(result)
                .to_str()
                .expect("Unable to convert pointer to string")
                .into(),
        })
    }
}

pub fn stop() -> Result<(), Error> {
    unsafe {
        let result = stop_macos_hook();

        if result.is_null() {
            return Ok(());
        }

        Err(Error {
            message: CStr::from_ptr(result)
                .to_str()
                .expect("Unable to convert pointer to string")
                .into(),
        })
    }
}
