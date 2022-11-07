use crate::keycodes::VK;

pub fn raw_keycode_to_vk(code: u16) -> VK {
    match code {
        0x100 => VK::MouseLeft,
        0x101 => VK::MouseRight,
        0x102 => VK::MouseMiddle,
        _ => VK::Unknown,
    }
}
