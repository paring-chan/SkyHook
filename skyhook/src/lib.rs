extern crate cancellation;

mod platforms;
pub mod types;

pub fn run() {
    platforms::linux::start().unwrap();
}
