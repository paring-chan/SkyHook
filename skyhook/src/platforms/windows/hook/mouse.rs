use std::time::SystemTime;

use winsafe::{co::WM, prelude::user_Hhook, HHOOK};

use crate::types::{Event, EventData};

use super::{CALLBACK, MOUSE_HOOK_ID};

fn get_code(ev: usize) -> u16 {
    match (ev as u32).into() {
        WM::LBUTTONDOWN | WM::LBUTTONUP => 0,
        WM::RBUTTONDOWN | WM::RBUTTONUP => 1,
        WM::MBUTTONDOWN | WM::MBUTTONUP => 2,
        _ => panic!("Unknown button"),
    }
}

pub extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    let processed_hook_id: HHOOK = unsafe { MOUSE_HOOK_ID.expect("HOOK_ID is None") };

    if code < 0 {
        // Don't do anything, just return
        return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
    }

    match (wparam as u32).into() {
        WM::LBUTTONDOWN | WM::RBUTTONDOWN | WM::MBUTTONDOWN => unsafe {
            CALLBACK.unwrap()(Event {
                time: SystemTime::now(),
                data: EventData::KeyPress(get_code(wparam) as u16),
            });
        },
        WM::LBUTTONUP | WM::RBUTTONUP | WM::MBUTTONUP => unsafe {
            CALLBACK.unwrap()(Event {
                time: SystemTime::now(),
                data: EventData::KeyRelease(get_code(wparam) as u16),
            });
        },
        _ => (),
    }

    return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
}
