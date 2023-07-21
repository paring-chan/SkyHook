use std::{
    collections::HashMap,
    ffi::{c_char, CString},
    ptr::null,
    sync::atomic::AtomicUsize,
    thread,
};

use chrono::{Local, NaiveDateTime};
use skyhook::{Event, Hook, KeyCode};

#[repr(C)]
#[derive(Debug, Clone)]
pub enum NativeEventType {
    KeyPress,
    KeyRelease,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct NativeEvent {
    pub code: KeyCode,
    pub event_type: NativeEventType,
    pub key: i32,
    pub time: NativeTime,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct NativeTime {
    pub time_sec: i64,
    pub time_nsec: u32,
}

static mut HOOKS: Option<HashMap<usize, Hook>> = None;
static mut HOOK_QUEUES: Option<HashMap<usize, Vec<NativeEvent>>> = None;
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn skyhook_new_hook() -> usize {
    let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let hooks = get_hook_map();
    let queues = get_hook_queue_map();
    let callback = Box::new(make_callback(id));

    queues.insert(id, Vec::new());
    hooks.insert(id, Hook::new(callback));

    id
}

#[no_mangle]
pub extern "C" fn skyhook_drop_hook(id: usize) {
    let hooks = get_hook_map();
    if let Some(hook) = hooks.remove(&id) {
        let mut hook = hook;
        hook.stop_polling();
    }

    let queues = get_hook_queue_map();
    queues.remove(&id);
}

#[no_mangle]
pub extern "C" fn skyhook_set_polling_frequency(id: usize, frequency: usize) -> *const c_char {
    let result = set_polling_frequency(id, frequency);

    if let Err(err) = result {
        let cstr = CString::new(err).unwrap();
        return cstr.into_raw();
    }

    null()
}

#[no_mangle]
pub extern "C" fn skyhook_get_polling_frequency(id: usize) -> usize {
    let result = get_polling_frequency(id);

    if let Err(_) = result {
        return 0;
    }

    result.unwrap()
}

#[no_mangle]
pub extern "C" fn skyhook_start_hook(id: usize) -> *const c_char {
    let result = start_hook(id);

    if let Err(err) = result {
        let cstr = CString::new(err).unwrap();
        return cstr.into_raw();
    }

    null()
}

#[no_mangle]
pub extern "C" fn skyhook_stop_hook(id: usize) -> *const c_char {
    let result = stop_hook(id);

    if let Err(err) = result {
        let cstr = CString::new(err).unwrap();
        return cstr.into_raw();
    }

    null()
}

#[no_mangle]
pub extern "C" fn skyhook_read_queue(id: usize, cb: extern "C" fn(NativeEvent)) {
    let queue = match get_hook_queue(id) {
        Ok(v) => v,
        Err(_) => return,
    };

    let copied_queue = queue.clone();

    for i in copied_queue {
        cb(i);
    }

    queue.clear();
}

#[no_mangle]
pub extern "C" fn skyhook_get_time() -> NativeTime {
    let now = Local::now().naive_local();

    get_time(now)
}

fn start_hook(id: usize) -> Result<(), String> {
    let hook = get_hook(id)?;

    thread::spawn(move || {
        hook.start_polling();
    });

    let hook = get_hook(id)?;

    hook.wait_for_start()?;

    Ok(())
}

fn get_polling_frequency(id: usize) -> Result<usize, String> {
    let hook = get_hook(id)?;

    Ok(hook.polling_rate.load(std::sync::atomic::Ordering::SeqCst))
}

fn set_polling_frequency(id: usize, frequency: usize) -> Result<(), String> {
    let hook = get_hook(id)?;
    hook.polling_rate
        .store(frequency, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

fn stop_hook(id: usize) -> Result<(), String> {
    let hook = get_hook(id)?;

    hook.stop_polling();

    Ok(())
}

fn get_hook_map() -> &'static mut HashMap<usize, Hook> {
    unsafe {
        match HOOKS {
            Some(ref mut hooks) => hooks,
            None => {
                let hooks = HashMap::new();
                HOOKS = Some(hooks);
                HOOKS.as_mut().unwrap()
            }
        }
    }
}

fn get_hook(id: usize) -> Result<&'static mut Hook, String> {
    let map = get_hook_map();

    if let Some(hook) = map.get_mut(&id) {
        Ok(hook)
    } else {
        Err(format!("hook not found: {}", id))
    }
}

fn get_hook_queue_map() -> &'static mut HashMap<usize, Vec<NativeEvent>> {
    unsafe {
        match HOOK_QUEUES {
            Some(ref mut queues) => queues,
            None => {
                let queues = HashMap::new();
                HOOK_QUEUES = Some(queues);
                HOOK_QUEUES.as_mut().unwrap()
            }
        }
    }
}

fn get_hook_queue(id: usize) -> Result<&'static mut Vec<NativeEvent>, String> {
    let queues = get_hook_queue_map();
    if let Some(queue) = queues.get_mut(&id) {
        Ok(queue)
    } else {
        Err(format!("queue not found: {}", id))
    }
}

fn make_callback(id: usize) -> impl Fn(Event) {
    move |ev| {
        let queue = match get_hook_queue(id) {
            Ok(v) => v,
            Err(_) => return,
        };

        let native_event = match ev {
            Event::KeyDown(ev) => {
                let time = ev.time;
                NativeEvent {
                    code: ev.code,
                    key: ev.key,
                    event_type: NativeEventType::KeyPress,
                    time: get_time(time),
                }
            }
            Event::KeyUp(ev) => {
                let time = ev.time;

                NativeEvent {
                    code: ev.code,
                    key: ev.key,
                    event_type: NativeEventType::KeyRelease,
                    time: get_time(time),
                }
            }
        };

        queue.append(&mut vec![native_event]);
    }
}

fn get_time(time: NaiveDateTime) -> NativeTime {
    NativeTime {
        time_sec: time.timestamp(),
        time_nsec: time.timestamp_subsec_nanos(),
    }
}
