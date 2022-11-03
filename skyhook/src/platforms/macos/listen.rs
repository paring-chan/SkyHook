#![allow(improper_ctypes_definitions)]
use crate::{
    platforms::macos::common::*,
    types::{Error, Event},
};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use core_foundation::runloop::CFRunLoopRef;
use core_graphics::event::{CGEventTapLocation, CGEventType};
use std::os::raw::c_void;

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event)>> = None;

pub static mut RUN_LOOP: Option<CFRunLoopRef> = None;

unsafe extern "C" fn raw_callback(
    _proxy: CGEventTapProxy,
    _type: CGEventType,
    cg_event: CGEventRef,
    _user_info: *mut c_void,
) -> CGEventRef {
    if let Some(event) = convert(_type, &cg_event) {
        if let Some(callback) = &mut GLOBAL_CALLBACK {
            callback(event);
        }
    }
    cg_event
}

pub fn listen<T>(callback: T) -> Result<(), Error>
where
    T: FnMut(Event) + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        let _pool = NSAutoreleasePool::new(nil);
        let tap = CGEventTapCreate(
            CGEventTapLocation::HID, // HID, Session, AnnotatedSession,
            kCGHeadInsertEventTap,
            CGEventTapOption::ListenOnly,
            kCGEventMaskForAllEvents,
            raw_callback,
            nil,
        );
        if tap.is_null() {
            return Err(Error {
                message: "Tap is null".into(),
            });
        }
        let _loop = CFMachPortCreateRunLoopSource(nil, tap, 0);
        if _loop.is_null() {
            return Err(Error {
                message: "Loop is null".into(),
            });
        }

        let current_loop = CFRunLoopGetCurrent();

        RUN_LOOP = Some(current_loop);

        CFRunLoopAddSource(current_loop, _loop, kCFRunLoopCommonModes);

        CGEventTapEnable(tap, true);

        CFRunLoopRun();
    }
    Ok(())
}
