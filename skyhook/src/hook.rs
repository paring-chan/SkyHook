use std::{collections::HashSet, thread, time::Instant};

use crate::{debug, Event};

#[derive(Debug)]
pub struct Hook {
    pub cancelled: bool,
    pub running: bool,
    pub key_mask: HashSet<i32>,
    pub polling_rate: usize,
    pub(crate) callback: fn(Event),

    pub(crate) error: Option<String>,
}

impl Hook {
    pub fn new(callback: fn(Event)) -> Hook {
        let hook = Hook {
            cancelled: false,
            running: false,
            key_mask: HashSet::new(),
            callback,
            polling_rate: 60, // defaults by 250hz
            error: None,
        };

        hook
    }

    pub fn start_polling(&mut self) {
        if self.running {
            self.error = Some("already running".to_string());
            return;
        }

        debug!(self.running = true);

        self.initialize();

        loop {
            if self.cancelled {
                break;
            }

            let instant_at_frame_start = Instant::now();

            self.poll();

            hertz::sleep_for_constant_rate(self.polling_rate, instant_at_frame_start);
        }

        self.running = false;
        self.cancelled = false;
    }

    pub fn wait_for_start(&self) -> Result<(), String> {
        while !self.running {
            if self.error.is_some() {
                return Err(self.error.clone().unwrap());
            }

            thread::yield_now();
        }

        Ok(())
    }

    pub fn stop_polling(&mut self) {
        self.cancelled = true;

        while self.running {
            thread::yield_now();
        }
    }
}
