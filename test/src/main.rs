use std::thread;

extern crate skyhook;

fn main() {
    skyhook::run(|event| {
        println!("{:?}", event);
    })
    .unwrap();

    println!("hook started!");

    // thread::park_timeout(Duration::from_secs(5));

    thread::park();

    skyhook::stop().expect("Failed to stop hook");
}
