use chrono::NaiveDateTime;

use crate::Hook;

impl Hook {
    pub(crate) fn initialize(&mut self) -> Result<(), String> {
        // TODO: Register EventTap
        Ok(())
    }

    pub(crate) fn poll(&mut self, _time: NaiveDateTime) {}

    pub(crate) fn finalize(&mut self) {
        // TODO: Unregister EventTap
    }
}
