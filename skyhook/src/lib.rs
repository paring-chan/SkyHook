mod event;
mod hook;
mod keycode;
mod macros;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub use event::Event;
pub use hook::Hook;
pub use keycode::KeyCode;
