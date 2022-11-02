use std::thread::Builder;

use winsafe::{
    co::WH,
    msg::wm,
    prelude::kernel_Hthread,
    prelude::{user_Hhook, Handle},
    HHOOK, HINSTANCE,
};

mod keyboard;

use crate::types::{Error, Event};

use super::{message::process_message, CANCELLATION_TOKEN};

//#region Commons
pub(crate) static mut HOOK_ID: Option<HHOOK> = None;
static mut THREAD_ID: Option<u32> = None;
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

    let cancellation_token = match unsafe { &CANCELLATION_TOKEN } {
        Some(v) => v,
        None => {
            return Err(Error {
                message: "Cancellation token is None.".into(),
            })
        }
    };

    let thread = Builder::new().spawn(|| {
        let registered_hook = HHOOK::SetWindowsHookEx(
            WH::KEYBOARD_LL,
            keyboard::hook_callback,
            Some(HINSTANCE::NULL),
            Some(0),
        );

        unsafe {
            HOOK_ID = match registered_hook {
                Ok(h) => Some(h),
                Err(err) => return Err(err),
            };

            let thread_id = winsafe::HTHREAD::GetCurrentThreadId();

            THREAD_ID = Some(thread_id);
        }

        process_message(cancellation_token);

        Ok(())
    });

    while let None = unsafe { HOOK_ID } {}

    if let Err(e) = thread {
        return Err(Error {
            message: format!("Failed to start hook thread: {:?}", e),
        });
    }

    Ok(())
}

pub fn stop() -> Result<(), Error> {
    unsafe {
        if let Some(hook_id) = HOOK_ID {
            match HHOOK::UnhookWindowsHookEx(hook_id) {
                Ok(_) => (),
                Err(err) => {
                    return Err(Error {
                        message: format!("Could not stop the hook. {:?}", err),
                    })
                }
            };
        }

        if let Some(thread_id) = THREAD_ID {
            if let Err(e) = winsafe::PostThreadMessage(thread_id, wm::Close {}) {
                return Err(Error {
                    message: format!("Failed to send close message: {:?}", e),
                });
            }

            return Ok(());
        }
    }

    Err(Error {
        message: "Hook cannot be stopped before starting.".into(),
    })
}
//#endregion
