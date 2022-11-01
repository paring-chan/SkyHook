use std::{fs, sync::Arc, thread};

use cancellation::CancellationTokenSource;

use crate::{platforms::linux::reader::InputReader, types::Error};

mod reader;

pub static mut CANCELLATION_TOKEN: Option<Arc<CancellationTokenSource>> = None;

pub fn start() -> Result<(), Error> {
    let dir = fs::read_dir("/dev/input").expect("Failed to read /dev/input");

    for path in dir {
        let filename = path.expect("Failed to get dir entry").file_name();
        let filename = match filename.to_str() {
            Some(v) => v,
            None => continue,
        };

        let cts = CancellationTokenSource::new();

        unsafe {
            CANCELLATION_TOKEN = Some(Arc::new(cts));
        }

        if filename.starts_with("event") {
            let reader = InputReader::new(format!("/dev/input/{}", filename));

            thread::spawn(move || reader.run());
        }
    }

    Ok(())
}

// TODO
#[allow(dead_code)]
pub fn stop() {
    let token = unsafe { &CANCELLATION_TOKEN };

    if let Some(token) = token {
        token.cancel();

        unsafe {
            CANCELLATION_TOKEN = None;
        }
    }
}
