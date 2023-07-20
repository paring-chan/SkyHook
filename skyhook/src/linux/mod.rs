use chrono::NaiveDateTime;

use crate::Hook;

impl Hook {
    pub(crate) fn initialize(&mut self) {}

    pub(crate) fn poll(&mut self, _time: NaiveDateTime) {}
}
