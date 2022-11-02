// safe stuff

use winsafe::{co::{WH, WM}, HHOOK, HINSTANCE};

use crate::types::Error;


/*
//#region Constants
const KEY_PRESSED: i32 = 0x8000;
const WM_KEYDOWN: i32 = 0x0100;
const WM_KEYUP: i32 = 0x0101;
const WM_SYSKEYDOWN: i32 = 0x0104;
const WM_SYSKEYUP: i32 = 0x0105;
const WH_KEYBOARD_LL: i32 = 13;
const WH_MOUSE_LL: i32 = 14;
//#endregion
*/

//#region Commons
static mut HOOK_ID: Option<HHOOK> = None;

pub fn start() -> Result<(), Error> {
    unsafe {
        let registered_hook = HHOOK::SetWindowsHookEx(
            WH::KEYBOARD_LL,
            hook_callback,
            Some(HINSTANCE::NULL),
            Some(0));

        let processed_hook_id = match registered_hook {
            Ok(h) => h,
            Err(err) => {
                return Err(Error {message: format!("Could not start the hook. {:?}", err)}); 
            }
        };

        HOOK_ID = Some(processed_hook_id);

        return Ok(());
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

        Err(Error {message: "Hook cannot be stopped before starting.".to_string()})
    }
}
//#endregion

extern "system" fn hook_callback(nCode: i32, wParam: usize, lParam: isize) -> isize {
    unsafe {
        let processed_hook_id = HOOK_ID.unwrap();
        
        if nCode < 0 {
            // Don't do anything, just return
            return processed_hook_id.CallNextHookEx(WH::from(nCode), wParam, lParam);
        }

        match WM::from(wParam as u32) {
            WM::KEYDOWN | WM::SYSKEYDOWN => {
                // TODO: Send an event
            },
            WM::KEYUP | WM::SYSKEYUP => {
                // TODO: Send an event
            },
            _ => ()
        }
    }

    return 1;
}
