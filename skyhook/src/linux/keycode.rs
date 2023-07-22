use x11::keysym::*;

use crate::KeyCode;

impl KeyCode {
    #[allow(non_upper_case_globals)]
    pub(crate) fn from_keysym(keysym: u64) -> KeyCode {
        match keysym as u32 {
            XK_BackSpace => Self::Backspace,
            XK_Tab => Self::Tab,
            // XK_Linefeed
            // XK_Clear
            XK_Return => Self::Enter,
            XK_Pause => Self::Pause,
            XK_Scroll_Lock => Self::ScrollLock,
            // XK_Sys_Req
            XK_Escape => Self::Escape,
            XK_Delete => Self::Delete,
            // XK_Multi_key
            XK_Home => Self::Home,
            XK_Left => Self::ArrowLeft,
            XK_Up => Self::ArrowUp,
            XK_Right => Self::ArrowRight,
            XK_Down => Self::ArrowDown,
            // XK_Prior
            XK_Page_Up => Self::PageUp,
            XK_Page_Down => Self::PageDown,
            XK_End => Self::End,
            // XK_Begin
            XK_Win_L => Self::LMeta,
            XK_Win_R => Self::RMeta,
            // XK_App
            // XK_Select
            XK_Print => Self::PrintScreen,
            // XK_Execute
            XK_Insert => Self::Insert,
            // XK_Undo
            // XK_Redo
            // XK_Menu
            // XK_Find
            // XK_Cancel
            // XK_Help
            // XK_Break
            // ...
            XK_Num_Lock => Self::NumLock,
            // XK_KP_Space
            // XK_KP_Tab
            XK_KP_Enter => Self::NumPadEnter,
            // XK_KP_F1
            // XK_KP_F2
            // XK_KP_F3
            // XK_KP_F4
            XK_KP_Home => Self::Home,
            XK_KP_Left => Self::ArrowLeft,
            XK_KP_Up => Self::ArrowUp,
            XK_KP_Right => Self::ArrowRight,
            XK_KP_Down => Self::ArrowDown,
            // XK_KP_Prior
            XK_KP_Page_Up => Self::PageUp,
            // XK_KP_Next
            XK_KP_Page_Down => Self::PageDown,
            XK_KP_End => Self::End,
            // XK_KP_Begin
            XK_KP_Insert => Self::Insert,
            XK_KP_Delete => Self::Delete,
            // XK_KP_Equal
            XK_KP_Multiply => Self::NumPadMultiply,
            XK_KP_Add => Self::NumPadAdd,
            // XK_KP_Separator,
            XK_KP_Subtract => Self::NumPadSubtract,
            XK_KP_Decimal => Self::NumPadDecimal,
            XK_KP_Divide => Self::NumPadDivide,

            XK_KP_0 => Self::NumPad0,
            XK_KP_1 => Self::NumPad1,
            XK_KP_2 => Self::NumPad2,
            XK_KP_3 => Self::NumPad3,
            XK_KP_4 => Self::NumPad4,
            XK_KP_5 => Self::NumPad5,
            XK_KP_6 => Self::NumPad6,
            XK_KP_7 => Self::NumPad7,
            XK_KP_8 => Self::NumPad8,
            XK_KP_9 => Self::NumPad9,

            XK_F1 => Self::F1,
            XK_F2 => Self::F2,
            XK_F3 => Self::F3,
            XK_F4 => Self::F4,
            XK_F5 => Self::F5,
            XK_F6 => Self::F6,
            XK_F7 => Self::F7,
            XK_F8 => Self::F8,
            XK_F9 => Self::F9,
            XK_F10 => Self::F10,
            XK_F11 => Self::F11,
            XK_F12 => Self::F12,
            XK_F13 => Self::F13,
            XK_F14 => Self::F14,
            XK_F15 => Self::F15,
            XK_F16 => Self::F16,
            XK_F17 => Self::F17,
            XK_F18 => Self::F18,
            XK_F19 => Self::F19,
            XK_F20 => Self::F20,
            XK_F21 => Self::F21,
            XK_F22 => Self::F22,
            XK_F23 => Self::F23,
            XK_F24 => Self::F24,

            XK_Shift_L => Self::LShift,
            XK_Shift_R => Self::RShift,

            XK_Control_L => Self::LControl,
            XK_Control_R => Self::RControl,

            XK_Caps_Lock => Self::CapsLock,

            // XK_Meta_L | XK_Meta_R
            XK_Alt_L => Self::LAlt,
            XK_Alt_R => Self::RAlt,

            XK_Super_L => Self::LMeta,
            XK_Super_R => Self::RMeta,

            // XK_HYPER_L | XK_HYPER_R
            XK_space => Self::Space,

            XK_comma => Self::Comma,
            XK_plus => Self::Equal,
            XK_minus => Self::Minus,
            XK_period => Self::Dot,
            XK_slash => Self::Slash,

            // skipping keys with shift
            XK_0 => Self::Alpha0,
            XK_1 => Self::Alpha1,
            XK_2 => Self::Alpha2,
            XK_3 => Self::Alpha3,
            XK_4 => Self::Alpha4,
            XK_5 => Self::Alpha5,
            XK_6 => Self::Alpha6,
            XK_7 => Self::Alpha7,
            XK_8 => Self::Alpha8,
            XK_9 => Self::Alpha9,

            XK_equal => Self::Equal,

            XK_semicolon => Self::Semicolon,
            XK_apostrophe => Self::Apostrophe,

            XK_a => Self::A,
            XK_b => Self::B,
            XK_c => Self::C,
            XK_d => Self::D,
            XK_e => Self::E,
            XK_f => Self::F,
            XK_g => Self::G,
            XK_h => Self::H,
            XK_i => Self::I,
            XK_j => Self::J,
            XK_k => Self::K,
            XK_l => Self::L,
            XK_m => Self::M,
            XK_n => Self::N,
            XK_o => Self::O,
            XK_p => Self::P,
            XK_q => Self::Q,
            XK_r => Self::R,
            XK_s => Self::S,
            XK_t => Self::T,
            XK_u => Self::U,
            XK_v => Self::V,
            XK_w => Self::W,
            XK_x => Self::X,
            XK_y => Self::Y,
            XK_z => Self::Z,

            XK_bracketleft => Self::LeftBrace,
            XK_bracketright => Self::RightBrace,
            XK_backslash => Self::BackSlash,
            XK_grave => Self::Grave,

            _ => Self::Unknown,
        }
    }
}
