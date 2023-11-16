use std::collections::HashSet;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::thread;
use std::thread::Builder;

use cacao::defaults::UserDefaults;
use core_foundation::{
    base::kCFAllocatorDefault,
    runloop::{CFRunLoop, kCFRunLoopDefaultMode},
};
use core_foundation::base::ToVoid;
use core_foundation::dictionary::{CFDictionaryCreateMutable, CFDictionarySetValue, kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks};
use core_foundation::number::{CFNumberCreate, kCFNumberIntType};
use core_foundation::runloop::CFRunLoopGetCurrent;
use io_kit_sys::CFSTR;
use io_kit_sys::hid::base::{IOHIDValueCallback, IOHIDValueRef};
use io_kit_sys::hid::element::IOHIDElementGetUsage;
use io_kit_sys::hid::keys::{kIOHIDOptionsTypeNone, kIOHIDPrimaryUsageKey};
use io_kit_sys::hid::manager::{IOHIDManagerCreate, IOHIDManagerOpen, IOHIDManagerRegisterInputValueCallback, IOHIDManagerScheduleWithRunLoop, IOHIDManagerSetDeviceMatching};
use io_kit_sys::hid::usage_tables::*;
use io_kit_sys::hid::value::{IOHIDValueGetElement, IOHIDValueGetIntegerValue};
use io_kit_sys::ret::{IOReturn, kIOReturnNotPermitted};

use crate::keycodes::VK;
use crate::keycodes::VK::*;
use crate::platforms::macos::keycode::raw_keycode_to_vk;
use crate::types::{Error, Event, EventData};

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

unsafe extern "C" fn keyboard_callback(
    _context: *mut c_void,
    _result: IOReturn,
    _sender: *mut c_void,
    value: IOHIDValueRef,
) {
    let element = IOHIDValueGetElement(value);
    // let device = IOHIDElementGetDevice(element);
    let key_code = IOHIDElementGetUsage(element);
    let element_value = IOHIDValueGetIntegerValue(value);

    if key_code == u32::MAX || key_code == 1 {
        return;
    }

    let vk = raw_keycode_to_vk(key_code);

    match element_value {
        0 => up(vk, key_code as i64),
        1 => down(vk, key_code as i64),
        _ => return
    }
}

fn swap_button(left: bool) -> VK {
    let swapped = match UserDefaults::standard().get("com.apple.mouse.swapLeftRightButton") {
        Some(v) => match v.as_bool() {
            Some(v) => v,
            None => false
        }
        None => false
    };
    let mut left = left;
    if swapped {
        left = !left;
    }
    return match left {
        true => MouseLeft,
        false => MouseRight
    };
}

#[allow(non_upper_case_globals)]
unsafe extern "C" fn mouse_callback(
    _context: *mut c_void,
    _result: IOReturn,
    _sender: *mut c_void,
    value: IOHIDValueRef,
) {
    let element = IOHIDValueGetElement(value);
    // let device = IOHIDElementGetDevice(element);
    let button_code = IOHIDElementGetUsage(element);
    let element_value = IOHIDValueGetIntegerValue(value);

    if button_code == u32::MAX {
        return;
    }

    let vk = match button_code {
        kHIDUsage_Button_1 => swap_button(true),
        kHIDUsage_Button_2 => swap_button(false),
        kHIDUsage_Button_3 => MouseMiddle,
        kHIDUsage_Button_4 => MouseX1,
        0x05 => MouseX2,
        _ => return
    };

    match element_value {
        0 => up(vk, button_code as i64),
        1 => down(vk, button_code as i64),
        _ => return
    }
}

unsafe fn watch(device_type: *const c_void, callback: IOHIDValueCallback) {
    let hid_manager = IOHIDManagerCreate(kCFAllocatorDefault, kIOHIDOptionsTypeNone);

    let match_dict = CFDictionaryCreateMutable(
        kCFAllocatorDefault,
        2,
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );

    CFDictionarySetValue(
        match_dict,
        CFSTR(kIOHIDPrimaryUsageKey).to_void(),
        CFNumberCreate(
            kCFAllocatorDefault,
            kCFNumberIntType,
            device_type,
        ).to_void(),
    );

    IOHIDManagerSetDeviceMatching(
        hid_manager,
        match_dict,
    );
    IOHIDManagerScheduleWithRunLoop(
        hid_manager,
        CFRunLoopGetCurrent(),
        kCFRunLoopDefaultMode,
    );


    let result = IOHIDManagerOpen(hid_manager, kIOHIDOptionsTypeNone);

    if result == kIOReturnNotPermitted {
        RESULT = Some(Err("NOT_PERMITTED".to_string()));
        return;
    }

    IOHIDManagerRegisterInputValueCallback(hid_manager, callback, null_mut());
}

pub fn start(#[allow(unused_variables)] callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        CURRENT_CALLBACK = Some(callback);

        Builder::new().spawn(|| {
            if IS_RUNNING {
                RESULT = Some(Err("Hook already initialized".to_string()));
                return;
            }

            watch((&kHIDUsage_GD_Keyboard as *const u32) as *const c_void, keyboard_callback);
            watch((&kHIDUsage_GD_Mouse as *const u32) as *const c_void, mouse_callback);

            LOOP = Some(CFRunLoop::get_current());

            RESULT = Some(Ok(()));
            IS_RUNNING = true;

            CFRunLoop::run_current();
        }).expect("Unable to create thread");

        loop {
            if let Some(result) = RESULT.as_ref() {
                let result = result.clone();
                RESULT = None;
                return match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error { message: e.clone() })
                };
            }
            thread::yield_now();
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
