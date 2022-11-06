use std::{collections::HashSet, time::SystemTime};

use winsafe::{co::WM, prelude::user_Hhook, HHOOK};

use crate::{
    breakable_unsafe,
    types::{Event, EventData},
};

use super::{CALLBACK, KBD_HOOK_ID};

#[derive(Clone, Copy)]
struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
}

unsafe fn get_code(lpdata: isize) -> u32 {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);

    kb.vk_code + 3 // 0~2 is for mouse
}

static mut PRESSED_KEYS: Option<HashSet<u16>> = None;

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

unsafe fn remove_key(key: u16) -> bool {
    if let Some(keys) = PRESSED_KEYS.as_mut() {
        return keys.remove(&key);
    }
    false
}

pub extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    let processed_hook_id: HHOOK = unsafe { KBD_HOOK_ID.expect("HOOK_ID is None") };

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
                    time: SystemTime::now(),
                    data: EventData::KeyPress(vkcode),
                });
            }
            WM::KEYUP | WM::SYSKEYUP => {
                let vkcode = get_code(lparam) as u16;

                // Do not ignore lifted keys upon next down event
                if !remove_key(vkcode) {
                    break;
                }

                CALLBACK.unwrap()(Event {
                    time: SystemTime::now(),
                    data: EventData::KeyRelease(vkcode),
                });
            }
            _ => (),
        }
    });

    // ALWAYS call CallNextHookEx
    return processed_hook_id.CallNextHookEx(code.into(), wparam, lparam);
}
