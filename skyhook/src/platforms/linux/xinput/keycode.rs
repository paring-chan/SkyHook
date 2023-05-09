use x11::keysym::*;

use crate::keycodes::VK;

#[allow(non_upper_case_globals)]
pub fn raw_xinput_keysym_to_vk(keysym: u64) -> VK {
    match keysym as u32 {
        XK_BackSpace => VK::Backspace,
        XK_Tab => VK::Tab,
        // XK_Linefeed
        // XK_Clear
        XK_Return => VK::Enter,
        XK_Pause => VK::PauseBreak,
        XK_Scroll_Lock => VK::ScrollLock,
        // XK_Sys_Req
        XK_Escape => VK::Escape,
        XK_Delete => VK::Delete,
        // XK_Multi_key
        XK_Home => VK::Home,
        XK_Left => VK::ArrowLeft,
        XK_Up => VK::ArrowUp,
        XK_Right => VK::ArrowRight,
        XK_Down => VK::ArrowDown,
        // XK_Prior
        XK_Page_Up => VK::PageUp,
        XK_Page_Down => VK::PageDown,
        XK_End => VK::End,
        // XK_Begin
        // XK_Win_L
        // XK_Win_R
        // XK_App
        // XK_Select
        XK_Print => VK::PrintScreen,
        // XK_Execute
        XK_Insert => VK::Insert,
        // XK_Undo
        // XK_Redo
        // XK_Menu
        // XK_Find
        // XK_Cancel
        // XK_Help
        // XK_Break
        // ...
        XK_Num_Lock => VK::NumLock,
        // XK_KP_Space
        // XK_KP_Tab
        XK_KP_Enter => VK::KeypadEnter,
        // XK_KP_F1
        // XK_KP_F2
        // XK_KP_F3
        // XK_KP_F4
        XK_KP_Home => VK::Home,
        XK_KP_Left => VK::ArrowLeft,
        XK_KP_Up => VK::ArrowUp,
        XK_KP_Right => VK::ArrowRight,
        XK_KP_Down => VK::ArrowDown,
        // XK_KP_Prior
        XK_KP_Page_Up => VK::PageUp,
        // XK_KP_Next
        XK_KP_Page_Down => VK::PageDown,
        XK_KP_End => VK::End,
        // XK_KP_Begin
        XK_KP_Insert => VK::Insert,
        XK_KP_Delete => VK::Delete,
        // XK_KP_Equal
        XK_KP_Multiply => VK::KeypadAsterisk,
        XK_KP_Add => VK::KeypadPlus,
        // XK_KP_Separator,
        XK_KP_Subtract => VK::KeypadMinus,
        XK_KP_Decimal => VK::KeypadDot,
        XK_KP_Divide => VK::KeypadSlash,

        XK_KP_0 => VK::Keypad0,
        XK_KP_1 => VK::Keypad1,
        XK_KP_2 => VK::Keypad2,
        XK_KP_3 => VK::Keypad3,
        XK_KP_4 => VK::Keypad4,
        XK_KP_5 => VK::Keypad5,
        XK_KP_6 => VK::Keypad6,
        XK_KP_7 => VK::Keypad7,
        XK_KP_8 => VK::Keypad8,
        XK_KP_9 => VK::Keypad9,

        XK_F1 => VK::F1,
        XK_F2 => VK::F2,
        XK_F3 => VK::F3,
        XK_F4 => VK::F4,
        XK_F5 => VK::F5,
        XK_F6 => VK::F6,
        XK_F7 => VK::F7,
        XK_F8 => VK::F8,
        XK_F9 => VK::F9,
        XK_F10 => VK::F10,
        XK_F11 => VK::F11,
        XK_F12 => VK::F12,
        XK_F13 => VK::F13,
        XK_F14 => VK::F14,
        XK_F15 => VK::F15,
        XK_F16 => VK::F16,
        XK_F17 => VK::F17,
        XK_F18 => VK::F18,
        XK_F19 => VK::F19,
        XK_F20 => VK::F20,
        XK_F21 => VK::F21,
        XK_F22 => VK::F22,
        XK_F23 => VK::F23,
        XK_F24 => VK::F24,

        XK_Shift_L => VK::LShift,
        XK_Shift_R => VK::RShift,

        XK_Control_L => VK::LControl,
        XK_Control_R => VK::RControl,

        XK_Caps_Lock => VK::CapsLock,

        // XK_Meta_L | XK_Meta_R
        XK_Alt_L => VK::LAlt,
        XK_Alt_R => VK::RAlt,

        XK_Super_L | XK_Super_R => VK::Super,

        // XK_HYPER_L | XK_HYPER_R
        XK_space => VK::Space,

        XK_comma => VK::Comma,
        XK_plus => VK::Equal,
        XK_minus => VK::Minus,
        XK_period => VK::Dot,
        XK_slash => VK::Slash,

        // skipping keys with shift
        XK_0 => VK::Alpha0,
        XK_1 => VK::Alpha1,
        XK_2 => VK::Alpha2,
        XK_3 => VK::Alpha3,
        XK_4 => VK::Alpha4,
        XK_5 => VK::Alpha5,
        XK_6 => VK::Alpha6,
        XK_7 => VK::Alpha7,
        XK_8 => VK::Alpha8,
        XK_9 => VK::Alpha9,

        XK_equal => VK::Equal,

        XK_semicolon => VK::Semicolon,
        XK_apostrophe => VK::Apostrophe,

        XK_a => VK::A,
        XK_b => VK::B,
        XK_c => VK::C,
        XK_d => VK::D,
        XK_e => VK::E,
        XK_f => VK::F,
        XK_g => VK::G,
        XK_h => VK::H,
        XK_i => VK::I,
        XK_j => VK::J,
        XK_k => VK::K,
        XK_l => VK::L,
        XK_m => VK::M,
        XK_n => VK::N,
        XK_o => VK::O,
        XK_p => VK::P,
        XK_q => VK::Q,
        XK_r => VK::R,
        XK_s => VK::S,
        XK_t => VK::T,
        XK_u => VK::U,
        XK_v => VK::V,
        XK_w => VK::W,
        XK_x => VK::X,
        XK_y => VK::Y,
        XK_z => VK::Z,

        XK_bracketleft => VK::LeftBrace,
        XK_bracketright => VK::RightBrace,
        XK_backslash => VK::BackSlash,
        XK_grave => VK::Grave,

        _ => VK::Unknown,
    }
}
