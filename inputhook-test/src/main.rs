extern crate inputhook;

use std::fs::{self};

fn main() {
    // println!("{}", inputhook::test());

    // let mut file = File::open();

    let dir = fs::read_dir("/dev/input").expect("Failed to read /dev/input");

    for path in dir {
        let filename = path.expect("Failed to get dir entry").file_name();
        let filename = match filename.to_str() {
            Some(v) => v,
            None => continue,
        };

        if filename.starts_with("event") {
            println!("{}", filename);
        }
    }
}
