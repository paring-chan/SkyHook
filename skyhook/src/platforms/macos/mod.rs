use std::ffi::{c_char, CStr};

use crate::types::{Error, Event};

extern "C" {
    fn start_macos_hook(callback: extern "C" fn(u16, bool)) -> *const c_char;
    fn stop_macos_hook() -> *const c_char;
}

static mut CURRENT_CALLBACK: Option<fn(Event)> = None;

extern "C" fn native_callback(key: u16, down: bool) {
    println!("{:?} {:?}", key, down);
}

pub fn start(#[allow(unused_variables)] callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        CURRENT_CALLBACK = Some(callback);

        let result = start_macos_hook(native_callback);

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
