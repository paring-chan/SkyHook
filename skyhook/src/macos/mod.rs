use std::collections::HashMap;

use core_foundation::{
    base::kCFAllocatorDefault,
    runloop::{kCFRunLoopDefaultMode, CFRunLoop},
};
use core_graphics::event::{CGEvent, CGEventTap, CGEventTapProxy, CGEventType};

use crate::Hook;

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
    pub(crate) fn initialize(&mut self) -> Result<(), String> {
        unsafe {
            // TODO: Register EventTap
            let state_map = get_state_map();
            if let Some(_) = state_map.get(&self.id) {
                return Err("Hook already initialized".to_string());
            }

            let callback = |_: CGEventTapProxy, event_type: CGEventType, event: &CGEvent| {
                dbg!(event_type);
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
