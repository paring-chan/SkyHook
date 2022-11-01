use std::{fs::File, io::Read};

use crate::types::Event;

use super::CANCELLATION_TOKEN;

#[derive(Debug)]
pub struct InputReader {
    file_path: String,
}

fn is_cancelled() -> bool {
    let token = unsafe { &CANCELLATION_TOKEN };

    if let Some(token) = token {
        token.is_canceled()
    } else {
        true
    }
}

fn convert_bit(bits: Vec<u8>) -> u16 {
    let mut result: u16 = 0;
    bits.iter().for_each(|&bit| {
        result <<= 1;
        result ^= bit as u16;
    });
    result
}

impl InputReader {
    pub fn new(file_path: String) -> InputReader {
        InputReader { file_path }
    }

    pub fn run(&self, callback: fn(Event)) {
        let mut file = File::open(&self.file_path).expect("Failed to open file");

        loop {
            if is_cancelled() {
                println!("Stop");
                return;
            }

            let mut buffer = [0; 24];

            if let Err(_) = file.read(&mut buffer) {
                return;
            }

            let event_type = convert_bit(vec![buffer[16], buffer[17]]);
            let code = convert_bit(vec![buffer[18], buffer[19]]);
            let value = convert_bit(vec![buffer[20], buffer[21]]);

            if event_type == 2 {
                match value {
                    0 => callback(Event::KeyRelease(code)),
                    2 => callback(Event::KeyPress(code)),
                    _ => continue,
                };
            }
        }
    }
}
