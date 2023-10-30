use std::collections::HashSet;
use std::thread::Builder;

use crate::types::{Error, Event, EventData};
use core_foundation::{
    base::kCFAllocatorDefault,
    runloop::{kCFRunLoopDefaultMode, CFRunLoop},
};

use core_graphics::event::{CGEvent, CGEventTap, CGEventTapProxy, CGEventType};
use crate::debug;
use crate::keycodes::VK;
use crate::platforms::macos::keycode::raw_keycode_to_vk;

mod keycode;

static mut CURRENT_CALLBACK: Option<fn(Event)> = None;

static mut PRESSED_KEYS: Option<HashSet<i64>> = None;

static mut LOOP: Option<CFRunLoop> = None;
static mut IS_RUNNING: bool = false;
static mut RESULT: Option<Result<(), String>> = None;

fn get_keys() -> &'static mut HashSet<i64> {
    unsafe {
        match PRESSED_KEYS.as_mut() {
            None => {
                let hs = HashSet::<i64>::new();

                PRESSED_KEYS = Some(hs);
                return PRESSED_KEYS.as_mut().unwrap();
            }
            Some(keys) => keys,
        }
    }
}

unsafe fn down(code: VK, key: i64) {
    let keys = get_keys();

    if keys.insert(key) {
        let callback = CURRENT_CALLBACK.unwrap();
        callback(Event { time: chrono::Local::now().naive_local(), data: EventData::KeyPress(code, key as u16) });
    }
}

unsafe fn up(code: VK, key: i64) {
    if get_keys().remove(&key) {
        let callback = CURRENT_CALLBACK.unwrap();
        callback(Event { time: chrono::Local::now().naive_local(), data: EventData::KeyRelease(code, key as u16) });
    }
}

unsafe fn modifier(code: VK, key: i64) {
    let pressed = get_keys().contains(&key);

    if pressed {
        up(code, key);
    } else {
        down(code, key);
    }
}

pub fn start(#[allow(unused_variables)] callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        CURRENT_CALLBACK = Some(callback);

        Builder::new().spawn(|| {
            if IS_RUNNING {
                RESULT = Some(Err("Hook already initialized".to_string()));
                return;
            }

            let callback = |_: CGEventTapProxy, event_type: CGEventType, event: &CGEvent| {
                match event_type {
                    CGEventType::KeyDown => {
                        let key = event.get_integer_value_field(9);
                        down(raw_keycode_to_vk(key as u16), key)
                    }
                    CGEventType::KeyUp => {
                        let key = event.get_integer_value_field(9);

                        up(raw_keycode_to_vk(key as u16), key)
                    }
                    CGEventType::LeftMouseDown => down(VK::MouseLeft, -1),
                    CGEventType::LeftMouseUp => up(VK::MouseLeft, -1),
                    CGEventType::RightMouseDown => down(VK::MouseRight, -2),
                    CGEventType::RightMouseUp => up(VK::MouseRight, -2),
                    CGEventType::OtherMouseDown => down(VK::MouseMiddle, -3),
                    CGEventType::OtherMouseUp => up(VK::MouseMiddle, -3),
                    CGEventType::FlagsChanged => {
                        let key = debug!(event.get_integer_value_field(9));
                        let code = debug!(raw_keycode_to_vk(key as u16));
                        modifier(code, key)
                    }
                    _ => {}
                }
                return Some(event.clone());
            };

            let event_tap = match CGEventTap::new(
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
            ) {
                Ok(v) => v,
                Err(_) => {
                    RESULT = Some(Err("Failed to create event tap".to_string()));

                    return;
                }
            };


            let loop_source = match event_tap
                .mach_port
                .create_runloop_source(kCFAllocatorDefault as isize)
                .map_err(|_| "Failed to create runloop source".to_string()) {
                Ok(v) => v,
                Err(e) => {
                    RESULT = Some(Err(e));
                    return;
                }
            };

            event_tap.enable();

            let run_loop = CFRunLoop::get_current();

            run_loop.add_source(&loop_source, kCFRunLoopDefaultMode);

            LOOP = Some(run_loop);
            IS_RUNNING = true;

            let run_loop = match &LOOP {
                Some(v) => v,
                _ => return
            };

            CFRunLoop::run_current();

            IS_RUNNING = false;

            run_loop.remove_source(&loop_source, kCFRunLoopDefaultMode);
        }).expect("Unable to create thread");
        loop {
            if let Some(result) = &RESULT {
                let result = result.clone();
                RESULT = None;
                return match result {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        println!("err: {:?}", e);
                        Err(Error { message: e.clone() })
                    }
                };
            }
        }
    }
}

pub fn stop() -> Result<(), Error> {
    unsafe {
        if !&IS_RUNNING {
            return Err(Error { message: "Hook is not running.".to_string() });
        }

        if let Some(run_loop) = &LOOP {
            run_loop.stop();
        }
    }

    Ok(())
}

pub fn is_running() -> bool {
    unsafe { IS_RUNNING }
}
