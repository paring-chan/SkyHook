use std::{fs::File, io::Read, time::SystemTime};

use crate::types::{Event, EventData};

use super::CANCELLATION_TOKEN;

fn convert_bit(bits: &[u8]) -> u16 {
    let mut result: u16 = 0;
    bits.iter().for_each(|&bit| {
        result <<= 1;
        result ^= bit as u16;
    });
    result
}

pub fn start_reader(file_path: String, callback: fn(Event)) {
    let token = match unsafe { &CANCELLATION_TOKEN } {
        Some(token) => token,
        None => return,
    };

    let mut file = File::open(file_path).expect("Failed to open file");

    loop {
        if token.is_canceled() {
            return;
        }

        let mut buffer = [0; 24];

        if let Err(_) = file.read(&mut buffer) {
            return;
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
