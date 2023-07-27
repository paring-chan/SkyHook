use std::collections::HashMap;

use core_foundation::{
    base::kCFAllocatorDefault,
    runloop::{kCFRunLoopDefaultMode, CFRunLoop},
};
use core_graphics::event::{CGEvent, CGEventTap, CGEventTapProxy, CGEventType};

use crate::{debug, Hook, KeyCode};

mod keycode;

static mut STATE_MAP: Option<HashMap<usize, OSXHookState>> = None;

fn get_state_map() -> &'static mut HashMap<usize, OSXHookState> {
    unsafe {
        if STATE_MAP.is_none() {
            STATE_MAP = Some(HashMap::new());
        }

        STATE_MAP.as_mut().unwrap()
    }
}

struct OSXHookState {
    run_loop: CFRunLoop,
}

impl Hook {
    fn down(&mut self, code: KeyCode, key: i64) {
        if self.key_mask.insert(key as i32) {
            (self.callback)(
                self.id,
                crate::Event::KeyDown(crate::event::EventData {
                    code,
                    key: key as i32,
                    time: chrono::Local::now().naive_local(),
                }),
            );
        }
    }
    fn up(&mut self, code: KeyCode, key: i64) {
        if self.key_mask.remove(&(key as i32)) {
            (self.callback)(
                self.id,
                crate::Event::KeyUp(crate::event::EventData {
                    code,
                    key: key as i32,
                    time: chrono::Local::now().naive_local(),
                }),
            );
        }
    }

    fn modifier(&mut self, code: KeyCode, key: i64, pressed: bool) {
        if !pressed {
            self.down(code, key);
        } else {
            self.up(code, key);
        }
    }

    pub(crate) fn initialize(&mut self) -> Result<(), String> {
        unsafe {
            let state_map = get_state_map();
            if let Some(_) = state_map.get(&self.id) {
                return Err("Hook already initialized".to_string());
            }

            let _self = self as *mut Hook;

            let callback = |_: CGEventTapProxy, event_type: CGEventType, event: &CGEvent| {
                match event_type {
                    CGEventType::KeyDown => {
                        let key = event.get_integer_value_field(9);
                        (*_self).down(KeyCode::from_virtual(key), key)
                    }
                    CGEventType::KeyUp => {
                        let key = event.get_integer_value_field(9);

                        (*_self).up(KeyCode::from_virtual(key), key)
                    }
                    CGEventType::LeftMouseDown => (*_self).down(KeyCode::MouseLeft, -1),
                    CGEventType::LeftMouseUp => (*_self).up(KeyCode::MouseLeft, -1),
                    CGEventType::RightMouseDown => (*_self).down(KeyCode::MouseRight, -2),
                    CGEventType::RightMouseUp => (*_self).up(KeyCode::MouseRight, -2),
                    CGEventType::OtherMouseDown => (*_self).down(KeyCode::MouseMiddle, -3),
                    CGEventType::OtherMouseUp => (*_self).up(KeyCode::MouseMiddle, -3),
                    CGEventType::FlagsChanged => {
                        let key = debug!(event.get_integer_value_field(9));
                        let code = debug!(KeyCode::from_virtual(key));
                        (*_self).modifier(code, key, self.key_mask.contains(&(key as i32)))
                    }
                    _ => {}
                }
                return Some(event.clone());
            };

            let event_tap = CGEventTap::new(
                core_graphics::event::CGEventTapLocation::Session,
                core_graphics::event::CGEventTapPlacement::HeadInsertEventTap,
                core_graphics::event::CGEventTapOptions::Default,
                vec![
                    CGEventType::KeyDown,
                    CGEventType::KeyUp,
                    CGEventType::LeftMouseDown,
                    CGEventType::LeftMouseUp,
                    CGEventType::RightMouseDown,
                    CGEventType::RightMouseUp,
                    CGEventType::OtherMouseDown,
                    CGEventType::OtherMouseUp,
                    CGEventType::FlagsChanged,
                ],
                callback,
            )
            .map_err(|_| "Failed to create event tap")?;

            let loop_source = event_tap
                .mach_port
                .create_runloop_source(kCFAllocatorDefault as isize)
                .map_err(|_| "Failed to create runloop source")?;

            event_tap.enable();

            let run_loop = CFRunLoop::get_current();

            run_loop.add_source(&loop_source, kCFRunLoopDefaultMode);

            let state = OSXHookState { run_loop };

            state_map.insert(self.id, state);

            self.running
                .store(true, std::sync::atomic::Ordering::SeqCst);

            CFRunLoop::run_current();
            Ok(())
        }
    }

    pub(crate) fn finalize(&mut self) {}

    pub(crate) fn pre_stop(&mut self) {
        let state_map = get_state_map();
        if let Some(state) = state_map.get(&self.id) {
            state.run_loop.stop();
        }
    }
}
