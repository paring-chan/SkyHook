use std::{
    collections::HashMap,
    ffi::{c_char, CString},
    ptr::null,
    sync::atomic::AtomicUsize,
    thread,
};

use skyhook::{Event, Hook, KeyCode};

#[repr(C)]
pub enum NativeEventType {
    KeyPress,
    KeyRelease,
}
#[derive(Debug)]
pub struct NativeEvent {
    pub code: KeyCode,
    pub key: i32,
    pub time_nsec: u64,
}

static mut HOOKS: Option<HashMap<usize, Hook>> = None;
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn skyhook_new_hook() -> usize {
    let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let hooks = get_hook_map();
    let callback = Box::new(make_callback(id));

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

fn start_hook(id: usize) -> Result<(), String> {
    let hook = get_hook(id)?;

    thread::spawn(move || {
        hook.start_polling();
    });

    let hook = get_hook(id)?;

    hook.wait_for_start()?;

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

fn make_callback(id: usize) -> impl Fn(Event) {
    move |ev| {
        dbg!(ev);
    }
}
