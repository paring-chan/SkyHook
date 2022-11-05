use std::{fs::File, io::Read, time::SystemTime};

use crate::types::{Error, Event, EventData};

use super::{ARC_READY_COUNT, CANCELLATION_TOKEN};

fn convert_bit(bits: &[u8]) -> u16 {
    let mut result: u16 = 0;
    bits.iter().for_each(|&bit| {
        result <<= 1;
        result ^= bit as u16;
    });
    result
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

        let event_type = convert_bit(&vec![buffer[16], buffer[17]]);
        let code = convert_bit(&vec![buffer[18], buffer[19]]);
        let value = convert_bit(&vec![buffer[20], buffer[21]]);

        if event_type == 2 {
            match value {
                0 => callback(Event {
                    time: SystemTime::now(),
                    data: EventData::KeyRelease(code),
                }),
                2 => callback(Event {
                    time: SystemTime::now(),
                    data: EventData::KeyPress(code),
                }),
                _ => continue,
            };
        }
    }
}
