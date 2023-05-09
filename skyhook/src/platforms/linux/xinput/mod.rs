use std::{
    collections::HashSet,
    ffi::{c_int, CString},
    ptr::null,
    thread,
};

use chrono::Local;
use x11::{
    xlib::{self, _XDisplay},
    xrecord,
};

use crate::{
    keycodes::VK,
    platforms::linux::xinput::keycode::raw_xinput_keysym_to_vk,
    types::{Error, Event},
};

mod keycode;

static mut RECORDER: Option<Recorder> = None;
static mut START_ERROR: Option<Error> = None;
static mut STARTED: bool = false;

struct Recorder {
    pub dpy_control: *mut _XDisplay,
    pub dpy_data: *mut _XDisplay,
    pub callback: fn(Event),
    pub context: u64,
}

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        if let Some(_) = &RECORDER {
            return Err(Error {
                message: "recorder is already running".into(),
            });
        }

        START_ERROR = None;
        STARTED = false;

        thread::spawn(move || {
            if let Err(error) = run_recorder(callback) {
                START_ERROR = Some(error);
            }
        });

        loop {
            if START_ERROR.is_some() {
                break;
            }
            if RECORDER.is_some() {
                break;
            }
            thread::yield_now();
        }

        if let Some(err) = START_ERROR.clone() {
            return Err(err);
        }

        return Ok(());
    }
}

fn run_recorder(callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        let dpy_control = xlib::XOpenDisplay(null());
        let dpy_data = xlib::XOpenDisplay(null());
        if dpy_control.is_null() || dpy_data.is_null() {
            return Err(Error {
                message: "Unable to open X11 display".into(),
            });
        }

        xlib::XSynchronize(dpy_control, 1);

        let extension_name = CString::new("RECORD").map_err(|_| Error {
            message: "Unable to allocate record extension name".into(),
        })?;

        let extension = xlib::XInitExtension(dpy_control, extension_name.as_ptr());

        if extension.is_null() {
            return Err(Error {
                message: "Unable to initialize X Record Extension".into(),
            });
        }

        let mut version_major: c_int = 0;
        let mut version_minor: c_int = 0;

        xrecord::XRecordQueryVersion(dpy_control, &mut version_major, &mut version_minor);

        let mut record_range: xrecord::XRecordRange = *xrecord::XRecordAllocRange();
        record_range.device_events.first = xlib::KeyPress as u8;
        record_range.device_events.last = xlib::ButtonRelease as u8;

        let context = xrecord::XRecordCreateContext(
            dpy_control,
            0,
            &mut xrecord::XRecordAllClients.clone(),
            1,
            std::mem::transmute(&mut &mut record_range),
            1,
        );

        if context == 0 {
            return Err(Error {
                message: "Unable to create record context".into(),
            });
        }

        RECORDER = Some(Recorder {
            callback: callback,
            dpy_control: dpy_control,
            dpy_data: dpy_data,
            context: context,
        });

        let result = dbg!(xrecord::XRecordEnableContextAsync(
            dpy_data,
            context,
            Some(record_callback),
            &mut 0
        ));

        if result == 0 {
            return Err(Error {
                message: "Unable to enable record context".into(),
            });
        }

        STARTED = true;

        loop {
            if !STARTED {
                break;
            }
            xrecord::XRecordProcessReplies(dpy_data);
        }

        Ok(())
    }
}

pub fn stop() -> Result<(), Error> {
    unsafe {
        if let Some(recorder) = &RECORDER {
            xrecord::XRecordDisableContext(recorder.dpy_data, recorder.context);
            xrecord::XRecordFreeContext(recorder.dpy_control, recorder.context);
            xlib::XSynchronize(recorder.dpy_control, 0);
        }
        STARTED = false;
    }

    Ok(())
}

unsafe extern "C" fn record_callback(_: *mut i8, raw_data: *mut xrecord::XRecordInterceptData) {
    let data = &*raw_data;

    if data.category != xrecord::XRecordFromServer {
        return;
    }

    let xdatum = &*(data.data as *mut XRecordDatum);

    if let Some(rec) = &RECORDER {
        let cb = rec.callback;

        match xdatum.xtype as i32 {
            xlib::KeyPress => {
                let keysym = xlib::XKeycodeToKeysym(rec.dpy_control, xdatum.code, 0);

                if add_key(xdatum.code) {
                    cb(Event {
                        time: Local::now().naive_local(),
                        data: crate::types::EventData::KeyPress(
                            raw_xinput_keysym_to_vk(keysym),
                            keysym as u16,
                        ),
                    })
                }
            }
            xlib::KeyRelease => {
                let keysym = xlib::XKeycodeToKeysym(rec.dpy_control, xdatum.code, 0);

                if remove_key(xdatum.code) {
                    cb(Event {
                        time: Local::now().naive_local(),
                        data: crate::types::EventData::KeyRelease(
                            raw_xinput_keysym_to_vk(keysym),
                            keysym as u16,
                        ),
                    })
                }
            }
            xlib::ButtonPress => {
                let key = get_mouse_key(xdatum.code);

                if let Some(key) = key {
                    cb(Event {
                        time: Local::now().naive_local(),
                        data: crate::types::EventData::KeyPress(key, xdatum.code.into()),
                    })
                }
            }
            xlib::ButtonRelease => {
                let key = get_mouse_key(xdatum.code);

                if let Some(key) = key {
                    cb(Event {
                        time: Local::now().naive_local(),
                        data: crate::types::EventData::KeyRelease(key, xdatum.code.into()),
                    })
                }
            }
            _ => (),
        }
    }
}

fn get_mouse_key(code: u8) -> Option<VK> {
    match code {
        1 => Some(VK::MouseLeft),
        2 => Some(VK::MouseMiddle),
        3 => Some(VK::MouseRight),
        _ => None,
    }
}

static mut PRESSED_KEYS: Option<HashSet<u8>> = None;

unsafe fn add_key(key: u8) -> bool {
    match PRESSED_KEYS.as_mut() {
        None => {
            let mut hs = HashSet::<u8>::new();

            hs.insert(key);

            PRESSED_KEYS = Some(hs);

            return true;
        }
        Some(keys) => {
            return keys.insert(key);
        }
    }
}

unsafe fn remove_key(key: u8) -> bool {
    if let Some(keys) = PRESSED_KEYS.as_mut() {
        return keys.remove(&key);
    }
    false
}

#[repr(C)]
#[derive(Debug)]
struct XRecordDatum {
    xtype: u8,
    code: u8,
    unknown1: u8,
    unknown2: u8,
}
