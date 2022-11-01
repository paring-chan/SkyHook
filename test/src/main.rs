use std::thread;

extern crate skyhook;

fn main() {
    skyhook::run(|event| {
        println!("{:?}", event);
    })
    .unwrap();

    thread::park();
}
