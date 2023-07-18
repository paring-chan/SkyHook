use std::{thread, time::Duration};

use skyhook::{Event, Hook};

static mut HOOK: Option<Hook> = None;

fn get_hook() -> &'static mut Hook {
    unsafe {
        match HOOK {
            Some(ref mut hook) => hook,
            None => {
                let hook = Hook::new(Box::new(callback));
                HOOK = Some(hook);
                HOOK.as_mut().unwrap()
            }
        }
    }
}

fn callback(ev: Event) {
    dbg!(ev);
}

fn main() {
    let hook = get_hook();

    thread::spawn(|| {
        get_hook().start_polling();
    });

    hook.wait_for_start().unwrap();

    thread::park_timeout(Duration::from_secs(5));

    hook.stop_polling();
}
