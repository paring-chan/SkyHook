use std::fs;

use crate::platforms::linux::reader::InputReader;

mod reader;

static mut READERS: Vec<InputReader> = vec![];

pub fn start() {
    let dir = fs::read_dir("/dev/input").expect("Failed to read /dev/input");

    let mut readers: Vec<InputReader> = vec![];

    for path in dir {
        let filename = path.expect("Failed to get dir entry").file_name();
        let filename = match filename.to_str() {
            Some(v) => v,
            None => continue,
        };

        if filename.starts_with("event") {
            let reader = InputReader::new(format!("/dev/input/{}", filename));
            readers.append(&mut vec![reader]);
        }
    }

    println!("{:?}", readers);

    unsafe {
        READERS = readers;
    }
}
