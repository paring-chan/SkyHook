use std::time::SystemTime;

use winsafe::{co::WM, prelude::user_Hhook, HHOOK, POINT};

use crate::types::{Event, EventData};

use super::{CALLBACK, MOUSE_HOOK_ID, keycodes::raw_keycode_to_vk};

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub dummy: u16,
    pub mouse_data: u16
}

unsafe fn get_code(ev: usize, lpdata: isize) -> u16 {
    match (ev as u32).into() { 
        WM::LBUTTONDOWN | WM::LBUTTONUP => 0,
        WM::RBUTTONDOWN | WM::RBUTTONUP => 1,
        WM::MBUTTONDOWN | WM::MBUTTONUP => 2,
        WM::XBUTTONDOWN | WM::XBUTTONUP => {
            let ms = *(lpdata as *const MSLLHOOKSTRUCT); 
            if ms.mouse_data == 1 { 3 } else { 4 } 
        },
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
        WM::LBUTTONDOWN | WM::RBUTTONDOWN | WM::MBUTTONDOWN | WM::XBUTTONDOWN => unsafe {
            CALLBACK.unwrap()(Event {
                time: SystemTime::now(),
                data: EventData::KeyPress(raw_keycode_to_vk(get_code(wparam, lparam) as u16)),
            });
        },
        WM::LBUTTONUP | WM::RBUTTONUP | WM::MBUTTONUP | WM::XBUTTONUP => unsafe {
            CALLBACK.unwrap()(Event {
                time: SystemTime::now(),
                data: EventData::KeyRelease(raw_keycode_to_vk(get_code(wparam, lparam) as u16)),
            });
        },
        _ => (),
    }

    return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
}
