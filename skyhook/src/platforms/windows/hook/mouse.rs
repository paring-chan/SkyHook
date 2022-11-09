use std::time::SystemTime;

use winsafe::{co::WM, prelude::user_Hhook, HHOOK, POINT};

use crate::{types::{Event, EventData}, breakable_unsafe};

use super::{CALLBACK, MOUSE_HOOK_ID, keycode::raw_keycode_to_vk};

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub dummy: u16,
    pub mouse_data: u16
}

// getting an xbutton's code requires a pointer
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

    // we can use a break statement like a jump here to
    // avoid putting multiple complex return statements
    breakable_unsafe!({
        if code < 0 {
            // Don't do anything, just return
            break;
        }
    
        match (wparam as u32).into() {
            // we could use any of the flags, but supporting xbutton still requires a pointer
            WM::LBUTTONDOWN | WM::RBUTTONDOWN | WM::MBUTTONDOWN | WM::XBUTTONDOWN => {
                let code = get_code(wparam, lparam) as u16;
    
                CALLBACK.unwrap()(Event {
                    time: SystemTime::now(),
                    data: EventData::KeyPress(raw_keycode_to_vk(code), code),
                });
            },
            WM::LBUTTONUP | WM::RBUTTONUP | WM::MBUTTONUP | WM::XBUTTONUP => {
                let code = get_code(wparam, lparam) as u16;
    
                CALLBACK.unwrap()(Event {
                    time: SystemTime::now(),
                    data: EventData::KeyRelease(raw_keycode_to_vk(code), code),
                });
            },
            _ => (),
        }
    });

    // ALWAYS call CallNextHookEx
    return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
}
