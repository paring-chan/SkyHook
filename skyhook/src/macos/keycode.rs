use crate::KeyCode;

impl KeyCode {
    pub(crate) fn from_virtual(key: i64) -> KeyCode {
        match key {
            0x00 => Self::A,
            0x01 => Self::S,
            0x02 => Self::D,
            0x03 => Self::F,
            0x04 => Self::H,
            0x05 => Self::G,
            0x06 => Self::Z,
            0x07 => Self::X,
            0x08 => Self::C,
            0x09 => Self::V,
            // skip
            0x0b => Self::B,
            0x0c => Self::Q,
            0x0d => Self::W,
            0x0e => Self::E,
            0x0f => Self::R,
            0x10 => Self::Y,
            0x11 => Self::T,
            0x12 => Self::Alpha1,
            0x13 => Self::Alpha2,
            0x14 => Self::Alpha3,
            0x15 => Self::Alpha4,
            0x16 => Self::Alpha6,
            0x17 => Self::Alpha5,
            0x18 => Self::Equal,
            0x19 => Self::Alpha9,
            0x1a => Self::Alpha7,
            0x1b => Self::Minus,
            0x1c => Self::Alpha8,
            0x1d => Self::Alpha0,
            0x1e => Self::RightBrace,
            0x1f => Self::O,
            0x20 => Self::U,
            0x21 => Self::LeftBrace,
            0x22 => Self::I,
            0x23 => Self::P,
            0x24 => Self::Enter,
            0x25 => Self::L,
            0x26 => Self::J,
            0x27 => Self::Apostrophe,
            0x28 => Self::K,
            0x29 => Self::Semicolon,
            0x2a => Self::BackSlash,
            0x2b => Self::Comma,
            0x2c => Self::Slash,
            0x2d => Self::N,
            0x2e => Self::M,
            0x2f => Self::Dot,
            0x30 => Self::Tab,
            0x31 => Self::Space,
            0x32 => Self::Grave,
            0x33 => Self::Backspace,
            // skip
            0x35 => Self::Escape,
            // skip
            0x37 => Self::LMeta,
            0x38 => Self::LShift,
            0x39 => Self::CapsLock,
            0x3a => Self::LAlt,
            0x3b => Self::LControl,
            0x3c => Self::RShift,
            0x3d => Self::RAlt,
            0x3e => Self::RControl,
            // skip
            0x40 => Self::F17,
            // skip
            0x43 => Self::NumPadMultiply,
            // skip
            0x45 => Self::NumPadAdd,
            // skip
            0x47 => Self::NumLock,
            // skip
            0x4b => Self::NumPadDivide,
            0x4c => Self::NumPadEnter,
            // skip
            0x4e => Self::NumPadSubtract,
            0x4f => Self::F18,
            0x50 => Self::F19,
            // skip
            0x52 => Self::NumPad0,
            0x53 => Self::NumPad1,
            0x54 => Self::NumPad2,
            0x55 => Self::NumPad3,
            0x56 => Self::NumPad4,
            0x57 => Self::NumPad5,
            0x58 => Self::NumPad6,
            0x59 => Self::NumPad7,
            0x5a => Self::F20,
            0x5b => Self::NumPad8,
            0x5c => Self::NumPad9,
            // skip
            0x5e => Self::Minus,
            // skip
            0x60 => Self::F5,
            0x61 => Self::F6,
            0x62 => Self::F7,
            0x63 => Self::F3,
            0x64 => Self::F8,
            0x65 => Self::F9,
            // skip
            0x67 => Self::F11,
            // skip
            0x69 => Self::F13,
            0x6a => Self::F16,
            0x6b => Self::F14,
            // skip
            0x6d => Self::F10,
            // skip
            0x6f => Self::F12,
            // skip
            0x71 => Self::F15,
            0x72 => Self::Insert,
            0x73 => Self::Home,
            0x74 => Self::PageUp,
            0x75 => Self::Delete,
            0x76 => Self::F4,
            0x77 => Self::End,
            0x78 => Self::F2,
            0x79 => Self::PageDown,
            0x7a => Self::F1,
            0x7b => Self::ArrowLeft,
            0x7c => Self::ArrowRight,
            0x7d => Self::ArrowDown,
            0x7e => Self::ArrowUp,
            // skip
            0x100 => Self::MouseLeft,
            0x101 => Self::MouseRight,
            0x102 => Self::MouseMiddle,

            _ => Self::Unknown,
        }
    }
}
