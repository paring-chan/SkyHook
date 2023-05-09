use crate::keycodes::VK;

#[allow(non_upper_case_globals)]
pub fn raw_xinput_keysym_to_vk(keysym: u64) -> VK {
    match keysym as u32 {
        _ => VK::Unknown,
    }
}
