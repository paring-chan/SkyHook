use std::{
    collections::{HashMap, HashSet},
    ffi::{c_char, c_ulong, c_void, CString},
    ptr::null,
};

use chrono::NaiveDateTime;
use x11::{
    xlib::{
        self, Display, XCloseDisplay, XExtCodes, XFree, XInitExtension, XOpenDisplay, XSynchronize,
    },
    xrecord::{
        XRecordAllClients, XRecordAllocRange, XRecordCreateContext, XRecordEnableContextAsync,
        XRecordFromServer, XRecordInterceptData, XRecordProcessReplies, XRecordRange,
    },
};

use crate::{Event, Hook, KeyCode};

mod keycode;

static mut HOOK_STATE_ID_MAP: Option<HashMap<usize, *mut i8>> = None;
static mut X11_STATES_MAP: Option<HashMap<*mut i8, X11State>> = None;
// static mut X11_STATE_QUEUE_MAP: Option<HashMap<*mut i8, Vec<NativeEvent>>> = None;

fn get_x11_state_map() -> &'static mut HashMap<*mut i8, X11State> {
    unsafe {
        if X11_STATES_MAP.is_none() {
            X11_STATES_MAP = Some(HashMap::new());
        }

        X11_STATES_MAP.as_mut().unwrap()
    }
}

fn get_x11_state_id_map() -> &'static mut HashMap<usize, *mut i8> {
    unsafe {
        if HOOK_STATE_ID_MAP.is_none() {
            HOOK_STATE_ID_MAP = Some(HashMap::new());
        }

        HOOK_STATE_ID_MAP.as_mut().unwrap()
    }
}

fn get_x11_state_id(hook_id: usize) -> Option<&'static *mut i8> {
    let state_id_map = get_x11_state_id_map();
    state_id_map.get(&hook_id)
}

fn get_x11_state_ptr(hook_id: *mut i8) -> Option<&'static mut X11State> {
    let state_map = get_x11_state_map();
    state_map.get_mut(&hook_id)
}

fn get_x11_state(hook_id: usize) -> Option<&'static mut X11State> {
    let state_id = get_x11_state_id(hook_id)?;

    get_x11_state_ptr(*state_id)
}

pub struct X11State {
    pub dpy_control: *mut Display,
    pub dpy_data: *mut Display,
    pub context: u64,
    pub extension: *mut XExtCodes,
    pub queue: Vec<QueueItem>,
}

impl Hook {
    pub(crate) fn initialize(&mut self) -> Result<(), String> {
        unsafe {
            let dpy_control = XOpenDisplay(null());
            let dpy_data = XOpenDisplay(null());

            XSynchronize(dpy_control, 1);

            let extension_name =
                CString::new("RECORD").map_err(|_| "Unable to allocate X11 extension name")?;

            let extension = XInitExtension(dpy_control, extension_name.as_ptr());

            if extension.is_null() {
                return Err("Unable to initialize SkyHook XRecord extension".to_string());
            }

            let mut record_range: XRecordRange = *XRecordAllocRange();
            record_range.device_events.first = xlib::KeyPress as u8;
            record_range.device_events.last = xlib::ButtonRelease as u8;

            let context = XRecordCreateContext(
                dpy_control,
                0,
                &mut XRecordAllClients.clone(),
                1,
                std::mem::transmute(&mut &mut record_range),
                1,
            );

            if context == 0 {
                return Err("Unable to create record context".to_string());
            }

            let mut id_i8 = self.id.clone() as i8 + 10;
            let id_ptr = &mut id_i8 as *mut i8;

            XRecordEnableContextAsync(dpy_data, context, Some(record_callback), id_ptr);

            let state = X11State {
                dpy_control,
                dpy_data,
                context,
                extension,
                queue: vec![],
            };

            let state_map = get_x11_state_map();
            let state_id_map = get_x11_state_id_map();

            state_id_map.insert(self.id, id_ptr);
            state_map.insert(id_ptr, state);
        }

        Ok(())
    }

    pub(crate) fn poll(&mut self, time: NaiveDateTime) {
        let cb = &self.callback;
        unsafe {
            let state = match get_x11_state(self.id) {
                Some(state) => state,
                None => return,
            };

            XRecordProcessReplies(state.dpy_data);

            for item in &state.queue {
                match item {
                    QueueItem::KeyPress(keysym) => {
                        let keysym_i32 = *keysym as i32;
                        if self.key_mask.insert(keysym_i32) {
                            cb(
                                self.id,
                                Event::KeyDown(crate::event::EventData {
                                    code: KeyCode::from_keysym(*keysym),
                                    key: *keysym as i32,
                                    time,
                                }),
                            );
                        }
                    }
                    QueueItem::KeyRelease(keysym) => {
                        let keysym_i32 = *keysym as i32;
                        if self.key_mask.remove(&keysym_i32) {
                            cb(
                                self.id,
                                Event::KeyUp(crate::event::EventData {
                                    code: KeyCode::from_keysym(*keysym),
                                    key: *keysym as i32,
                                    time,
                                }),
                            );
                        }
                    }
                }
            }
            state.queue.clear();
        }
    }

    pub(crate) fn finalize(&mut self) {
        if let Some(ptr) = get_x11_state_id(self.id) {
            let state_map = get_x11_state_map();
            let state_id_map = get_x11_state_id_map();

            state_map.remove(ptr);
            state_id_map.remove(&self.id);
        }
    }
}

unsafe extern "C" fn record_callback(id: *mut c_char, data: *mut XRecordInterceptData) {
    let data = &*data;

    if data.category != XRecordFromServer {
        return;
    }

    let state = match get_x11_state_ptr(id) {
        Some(state) => state,
        None => return,
    };

    let xdatum = &*(data.data as *mut XRecordDatum);

    match xdatum.xtype as i32 {
        xlib::KeyPress => {
            let keysym = xlib::XKeycodeToKeysym(state.dpy_control, xdatum.code, 0);
            state.queue.append(&mut vec![QueueItem::KeyPress(keysym)]);
        }
        xlib::KeyRelease => {
            let keysym = xlib::XKeycodeToKeysym(state.dpy_control, xdatum.code, 0);
            state.queue.append(&mut vec![QueueItem::KeyRelease(keysym)]);
        }
        _ => {}
    }
}

impl Drop for Hook {
    fn drop(&mut self) {}
}

impl Drop for X11State {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.dpy_control);
            XCloseDisplay(self.dpy_data);
        }
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

#[derive(Debug)]
pub enum QueueItem {
    KeyPress(u64),
    KeyRelease(u64),
}
