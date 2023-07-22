use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Instant,
};

use chrono::Local;

use crate::{debug, Event};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Hook {
    pub id: usize,
    pub running: Arc<AtomicBool>,
    pub cancelled: Arc<AtomicBool>,
    pub polling_rate: Arc<AtomicUsize>,
    pub key_mask: HashSet<i32>,
    pub(crate) callback: Box<dyn Fn(usize, Event) + Send + Sync>,
    pub(crate) error: Option<String>,
}

impl Hook {
    pub fn new(callback: Box<dyn Fn(usize, Event) + Send + Sync>) -> Hook {
        let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let hook = Hook {
            id,
            key_mask: HashSet::new(),
            running: Arc::new(AtomicBool::new(false)),
            cancelled: Arc::new(AtomicBool::new(false)),
            callback: callback,
            polling_rate: Arc::new(AtomicUsize::new(250)), // defaults by 250hz
            error: None,
        };

        hook
    }

    pub fn start_polling(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            self.error = Some("already running".to_string());
            return;
        }

        if let Err(message) = self.initialize() {
            self.error = Some(message);
            return;
        }

        debug!(self.running.store(true, Ordering::SeqCst));

        loop {
            if self.cancelled.load(Ordering::SeqCst) {
                break;
            }

            let instant_at_frame_start = Instant::now();
            let time = Local::now().naive_local();

            self.poll(time);

            hertz::sleep_for_constant_rate(
                self.polling_rate.load(Ordering::SeqCst),
                instant_at_frame_start,
            );
        }

        #[cfg(target_os = "linux")]
        self.finalize();

        self.running.store(false, Ordering::SeqCst);
        self.cancelled.store(false, Ordering::SeqCst);
    }

    pub fn wait_for_start(&mut self) -> Result<(), String> {
        while !self.running.load(Ordering::SeqCst) {
            thread::yield_now();
            if self.error.is_some() {
                return Err(self.error.clone().unwrap());
            }
        }

        Ok(())
    }

    pub fn stop_polling(&mut self) {
        self.cancelled.store(true, Ordering::SeqCst);

        while self.running.load(Ordering::SeqCst) {
            thread::yield_now();
        }
    }
}
