use std::{thread, time::Duration};

extern crate skyhook;

fn main() {
    skyhook::run(|event| {
        println!("{:?}", event);
    })
    .unwrap();

    // thread::park_timeout(Duration::from_secs(5));

    thread::park();

    skyhook::stop().expect("Failed to stop hook");
}
