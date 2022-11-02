use std::time::SystemTime;

use winsafe::{co::WM, prelude::user_Hhook, HHOOK};

use crate::types::{Event, EventData};

use super::{CALLBACK, HOOK_ID};

#[derive(Clone, Copy)]
struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
}

unsafe fn get_code(lpdata: isize) -> u32 {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);

    kb.vk_code
}

pub extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    let processed_hook_id: HHOOK = unsafe { HOOK_ID.expect("HOOK_ID is None") };

    if code < 0 {
        // Don't do anything, just return
        return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
    }

    match (wparam as u32).into() {
        WM::KEYDOWN | WM::SYSKEYDOWN => unsafe {
            CALLBACK.unwrap()(Event {
                time: SystemTime::now(),
                data: EventData::KeyPress(get_code(lparam) as u16),
            });
        },
        WM::KEYUP | WM::SYSKEYUP => unsafe {
            CALLBACK.unwrap()(Event {
                time: SystemTime::now(),
                data: EventData::KeyRelease(get_code(lparam) as u16),
            });
        },
        _ => (),
    }

    // ALWAYS call CallNextHookEx
    return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
}
