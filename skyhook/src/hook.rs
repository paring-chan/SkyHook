use std::{collections::HashSet, thread, time::Instant};

use crate::{debug, Event};

pub struct Hook {
    pub running: bool,
    pub cancelled: bool,
    pub polling_rate: usize,
    pub key_mask: HashSet<i32>,
    pub(crate) callback: Box<dyn Fn(Event) + Send + Sync>,
    pub(crate) error: Option<String>,
}

impl Hook {
    pub fn new(callback: Box<dyn Fn(Event) + Send + Sync>) -> Hook {
        let hook = Hook {
            key_mask: HashSet::new(),
            running: false,
            cancelled: false,
            callback: callback,
            polling_rate: 250, // defaults by 250hz
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

            self.poll(instant_at_frame_start);

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
