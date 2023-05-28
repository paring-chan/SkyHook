use std::thread::{self, Builder};

use winsafe::{
    co::{ERROR, WH},
    msg::wm,
    prelude::kernel_Hthread,
    prelude::{user_Hhook, Handle},
    HHOOK, HINSTANCE,
};

mod keyboard;
mod keycode;
mod mouse;

use crate::types::{Error, Event};

use super::{message::process_message, CANCELLATION_TOKEN};

//#region Commons
pub(crate) static mut KBD_HOOK_ID: Option<HHOOK> = None;
pub(crate) static mut MOUSE_HOOK_ID: Option<HHOOK> = None;
static mut THREAD_ID: Option<u32> = None;
static mut LISTEN_ERROR: Option<ERROR> = None;
static mut CALLBACK: Option<fn(Event)> = None;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        // return if hook is already set
        if KBD_HOOK_ID.is_some() {
            return Err(Error {
                message: "Hook cannot be started if the hook is already running.".into(),
            });
        }

        // assign callback
        CALLBACK = Some(callback);
    }

    let cancellation_token = match unsafe { &CANCELLATION_TOKEN } {
        Some(v) => v,
        None => {
            return Err(Error {
                message: "Cancellation token is None.".into(),
            })
        }
    };

    unsafe { LISTEN_ERROR = None }

    let thread = Builder::new().spawn(|| {
        let registered_keyboard_hook = HHOOK::SetWindowsHookEx(
            WH::KEYBOARD_LL,
            keyboard::hook_callback,
            Some(HINSTANCE::NULL),
            Some(0),
        );

        let registered_mouse_hook = HHOOK::SetWindowsHookEx(
            WH::MOUSE_LL,
            mouse::hook_callback,
            Some(HINSTANCE::NULL),
            Some(0),
        );

        unsafe {
            KBD_HOOK_ID = match registered_keyboard_hook {
                Ok(h) => Some(h),
                Err(err) => {
                    LISTEN_ERROR = Some(err);
                    return;
                }
            };
            MOUSE_HOOK_ID = match registered_mouse_hook {
                Ok(h) => Some(h),
                Err(err) => {
                    LISTEN_ERROR = Some(err);
                    return;
                }
            };

            let thread_id = winsafe::HTHREAD::GetCurrentThreadId();

            THREAD_ID = Some(thread_id);
        }

        process_message(cancellation_token);

        unsafe {
            THREAD_ID = None;
        }
    });

    if let Err(e) = thread {
        return Err(Error {
            message: format!("Failed to start hook thread: {:?}", e),
        });
    }

    while unsafe { THREAD_ID.is_none() } {
        if let Some(err) = unsafe { LISTEN_ERROR } {
            return Err(Error {
                message: format!("Unable to set hook: {:?}", err),
            });
        }
        thread::yield_now();
    }

    Ok(())
}

pub fn stop() -> Result<(), Error> {
    let mut exists = false;

    unsafe {
        if let Some(cancellation_token) = &CANCELLATION_TOKEN {
            cancellation_token.cancel();
        } else {
            return Err(Error {
                message: "Cancellation token not found".into(),
            });
        }

        if let Some(hook_id) = KBD_HOOK_ID {
            exists = true;
            match HHOOK::UnhookWindowsHookEx(hook_id) {
                Ok(_) => (),
                Err(err) => {
                    return Err(Error {
                        message: format!("Could not stop the keyboard hook. {:?}", err),
                    })
                }
            };
            KBD_HOOK_ID = None;
        }

        if let Some(hook_id) = MOUSE_HOOK_ID {
            exists = true;
            match HHOOK::UnhookWindowsHookEx(hook_id) {
                Ok(_) => (),
                Err(err) => {
                    return Err(Error {
                        message: format!("Could not stop the mouse hook. {:?}", err),
                    })
                }
            };
            MOUSE_HOOK_ID = None;
        }

        if let Some(thread_id) = THREAD_ID {
            if let Err(e) = winsafe::PostThreadMessage(thread_id, wm::Close {}) {
                return Err(Error {
                    message: format!("Failed to send close message: {:?}", e),
                });
            }

            exists = true;

            while THREAD_ID.is_some() {
                thread::yield_now();
            }
        }
    }

    if !exists {
        Err(Error {
            message: "Hook cannot be stopped before starting.".into(),
        })
    } else {
        Ok(())
    }
}

pub fn is_running() -> bool {
    unsafe {
        if let Some(_) = &CANCELLATION_TOKEN {
            true
        } else {
            false
        }
    }
}

//#endregion
