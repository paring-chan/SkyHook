use std::{collections::HashSet};

use chrono::Local;
use winsafe::{co::WM, prelude::user_Hhook, HHOOK};

use crate::{
    breakable_unsafe,
    types::{Event, EventData}, platforms::windows::hook::keycode::raw_keycode_to_vk,
};

use super::{CALLBACK, KBD_HOOK_ID};

#[derive(Clone, Copy)]
struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
}

// getting a vkcode requires a pointer
unsafe fn get_code(lpdata: isize) -> u32 {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);

    kb.vk_code
}

// static hashset
static mut PRESSED_KEYS: Option<HashSet<u16>> = None;

// add a key to hashset to filter consecutive key down events
unsafe fn add_key(key: u16) -> bool {
    match PRESSED_KEYS.as_mut() {
        None => {
            let mut hs = HashSet::<u16>::new();

            hs.insert(key);

            PRESSED_KEYS = Some(hs);

            return true;
        }
        Some(keys) => {
            return keys.insert(key);
        }
    }
}

// remove a key in the hashset
unsafe fn remove_key(key: u16) -> bool {
    if let Some(keys) = PRESSED_KEYS.as_mut() {
        return keys.remove(&key);
    }
    false
}

pub extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    let processed_hook_id: HHOOK = unsafe { KBD_HOOK_ID.expect("HOOK_ID is None") };

    // we can use a break statement like a jump here to
    // avoid putting multiple complex return statements
    breakable_unsafe!({
        if code < 0 {
            // Don't do anything, just return
            break;
        }

        match (wparam as u32).into() {
            WM::KEYDOWN | WM::SYSKEYDOWN => {
                let vkcode = get_code(lparam) as u16;

                // Ignore already down keys
                if !add_key(vkcode) {
                    break;
                }

                CALLBACK.unwrap()(Event {
                    time: Local::now().naive_local(),
                    data: EventData::KeyPress(raw_keycode_to_vk(vkcode), vkcode),
                });
            }
            WM::KEYUP | WM::SYSKEYUP => {
                let vkcode = get_code(lparam) as u16;

                // Do not ignore lifted keys upon next down event
                if !remove_key(vkcode) {
                    break;
                }

                CALLBACK.unwrap()(Event {
                    time: Local::now().naive_local(),
                    data: EventData::KeyRelease(raw_keycode_to_vk(vkcode), vkcode),
                });
            }
            _ => (),
        }
    });

    // ALWAYS call CallNextHookEx
    return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
}
