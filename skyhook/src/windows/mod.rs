use std::collections::HashSet;

use once_cell::sync::Lazy;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_MENU, VK_SHIFT,
};

use crate::{windows::keycode::get_keycode, Event, Hook};

mod keycode;

fn vk(key: VIRTUAL_KEY) -> i32 {
    key.0 as i32
}

static IGNORED_KEYS: Lazy<HashSet<i32>> = Lazy::new(|| {
    let mut set = HashSet::new();

    set.insert(vk(VK_CONTROL));
    set.insert(vk(VK_SHIFT));
    set.insert(vk(VK_MENU));

    set
});

impl Hook {
    pub(crate) fn initialize(&mut self) {
        unsafe {
            for i in 0x01..0xfe {
                if IGNORED_KEYS.contains(&i) {
                    continue;
                }

                let state = GetAsyncKeyState(i);

                if state < 0 {
                    self.key_mask.insert(i);
                } else {
                    self.key_mask.remove(&i);
                }
            }
        }
    }

    pub(crate) fn poll(&mut self) {
        unsafe {
            for i in 0x01..0xfe {
                if IGNORED_KEYS.contains(&i) {
                    continue;
                }

                let state = GetAsyncKeyState(i);
                let cb = self.callback;

                if state < 0 {
                    if self.key_mask.insert(i) {
                        let (key, i) = get_keycode(i as u16);
                        cb(Event::KeyDown(key, i));
                    }
                } else {
                    if self.key_mask.remove(&i) {
                        let (key, i) = get_keycode(i as u16);
                        cb(Event::KeyUp(key, i));
                    }
                }
            }
        }
    }
}