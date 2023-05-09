use std::{
    ffi::{c_int, CString},
    ptr::null,
};

use chrono::Local;
use x11::{xlib, xrecord};

use crate::{
    keycodes::VK,
    types::{Error, Event},
};

static mut CALLBACK: Option<fn(Event)> = None;

pub fn start(callback: fn(Event)) -> Result<(), Error> {
    unsafe {
        CALLBACK = Some(callback);

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

        let result =
            xrecord::XRecordEnableContext(dpy_data, context, Some(record_callback), &mut 0);
        if result == 0 {
            return Err(Error {
                message: "Unable to enable record context".into(),
            });
        }
    }

    Ok(())
}

pub fn stop() -> Result<(), Error> {
    Ok(())
}

unsafe extern "C" fn record_callback(_: *mut i8, raw_data: *mut xrecord::XRecordInterceptData) {
    let data = &*raw_data;

    if data.category != xrecord::XRecordFromServer {
        return;
    }

    let xdatum = &*(data.data as *mut XRecordDatum);

    if let Some(cb) = CALLBACK {
        cb(Event {
            time: Local::now().naive_local(),
            data: crate::types::EventData::KeyPress(VK::Unknown, xdatum.code.into()),
        })
    }
}

#[repr(C)]
#[derive(Debug)]
struct XRecordDatum {
    xtype: u8,
    code: u8,
    unknown1: u8,
    unknown2: u8,
}
