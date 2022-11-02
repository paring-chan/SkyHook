use std::{thread::Builder, time::SystemTime};

use winsafe::{co::{WH, WM}, HHOOK, HINSTANCE};

use crate::types::{Error, Event, EventData};

//#region Commons
static mut HOOK_ID: Option<HHOOK> = None;
static mut CALLBACK: Option<fn(Event)> = None;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        // return if hook is already set
        if HOOK_ID.is_some() {
            return Err(Error {
                message: "Hook cannot be started if the hook is already running.".into(),
            });
        }

        // assign callback
        CALLBACK = Some(callback);
    }

    let thread = Builder::new().spawn(|| {
        let registered_hook = HHOOK::SetWindowsHookEx(
            WH::KEYBOARD_LL,
            hook_callback,
            Some(HINSTANCE::NULL),
            Some(0));

        unsafe {
            HOOK_ID = match registered_hook {
                Ok(h) => Some(h),
                Err(err) => return Err(err)
            };
        }

        return Ok(());
    });

    match thread {
        Ok(processed_thread) => {
            match processed_thread.join() {
                Ok(result) => {
                    match result {
                        Err(err) => Err(Error {message: format!("Could not start the hook. {:?}", err)}),
                        _ => Ok(())
                    }
                },
                Err(err) => {
                    Err(Error {message: format!("A panic occured in the thread. {:?}", err)})
                }
            }
        },
        Err(err) => {
            Err(Error {message: format!("Could not create a thread for the hook. {:?}", err)})
        }
    }
}

pub fn stop() -> Result<(), Error> {
    unsafe {
        if let Some(hook_id) = HOOK_ID {
            match HHOOK::UnhookWindowsHookEx(hook_id) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(Error {message: format!("Could not stop the hook. {:?}", err)})
            };
        }
    }

    Err(Error {message: "Hook cannot be stopped before starting.".to_string()})
}
//#endregion

// This is executed in another thread!
extern "system" fn hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    let processed_hook_id: HHOOK;

    unsafe {
        processed_hook_id = HOOK_ID.unwrap();
    }

    if code < 0 {
        // Don't do anything, just return
        return processed_hook_id.CallNextHookEx(WH::from(code), w_param, l_param);
    }

    match WM::from(w_param as u32) {
        WM::KEYDOWN | WM::SYSKEYDOWN => {
            unsafe {
                CALLBACK.unwrap() (Event {
                    time: SystemTime::now(),
                    data: EventData::KeyPress(l_param as u16),
                });
            }
        },
        WM::KEYUP | WM::SYSKEYUP => {
            unsafe {
                CALLBACK.unwrap() (Event {
                    time: SystemTime::now(),
                    data: EventData::KeyRelease(l_param as u16),
                });
            }
        },
        _ => ()
    }

    // ALWAYS call CallNextHookEx
    return processed_hook_id.CallNextHookEx(WH::from(code), w_param, l_param);
}
