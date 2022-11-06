use std::{collections::HashSet, fs::File, io::Read, time::SystemTime};

use crate::types::{Error, Event, EventData};

use super::{ARC_READY_COUNT, CANCELLATION_TOKEN};

static mut PRESSED_KEYS: Option<HashSet<u16>> = None;

unsafe fn add_key(key: u16) -> bool {
    match PRESSED_KEYS.as_mut() {
        None => {
            let mut hs = HashSet::<u16>::new();

            hs.insert(key);

            PRESSED_KEYS = Some(hs);

            return true;
        }
        Some(keys) => {
            return keys.insert(key);
        }
    }
}

unsafe fn remove_key(key: u16) -> bool {
    if let Some(keys) = PRESSED_KEYS.as_mut() {
        return keys.remove(&key);
    }
    false
}

pub fn start_reader(file_path: String, callback: fn(Event)) -> Result<(), Error> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(Error {
                message: format!("Failed to open file: {:?}", e.kind()),
            });
        }
    };

    unsafe {
        if let Some(arc) = &ARC_READY_COUNT {
            arc.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    loop {
        let mut buffer = [0; 24];

        if let Err(err) = file.read(&mut buffer) {
            return Err(Error {
                message: format!("Failed to read stream: {:?}", err),
            });
        }

        let token = match unsafe { &CANCELLATION_TOKEN } {
            Some(token) => token,
            None => {
                return Ok(());
            }
        };

        if token.is_canceled() {
            return Ok(());
        }

        let event_type: [u8; 2] = [buffer[16], buffer[17]];
        let code: [u8; 2] = [buffer[18], buffer[19]];
        let value: [u8; 2] = [buffer[20], buffer[21]];

        let event_type = u16::from_le_bytes(event_type);
        let code = u16::from_le_bytes(code);
        let value = u16::from_le_bytes(value);

        unsafe {
            if event_type == 1 {
                match value {
                    0 => {
                        if !remove_key(code) {
                            continue;
                        }

                        callback(Event {
                            time: SystemTime::now(),
                            data: EventData::KeyRelease(code),
                        });
                    }
                    1 => {
                        if !add_key(code) {
                            continue;
                        }

                        callback(Event {
                            time: SystemTime::now(),
                            data: EventData::KeyPress(code),
                        });
                    }
                    _ => continue,
                };
            }
        }
    }
}
