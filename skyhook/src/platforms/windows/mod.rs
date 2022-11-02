mod hook;

pub fn start() -> Result<(), crate::types::Error> {
    hook::start()
}

pub fn stop() -> Result<(), crate::types::Error> {
    hook::stop()
}
