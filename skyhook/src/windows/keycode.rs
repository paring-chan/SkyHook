use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::{debug, KeyCode};

const SM_SWAPBUTTON: i32 = 23;

unsafe fn get_system_metrics(index: i32) -> i32 {
    windows_targets::link!("user32.dll" "system" fn GetSystemMetrics(nIndex: i32) -> i32);
    GetSystemMetrics(index)
}

unsafe fn is_mouse_left_handed() -> bool {
    debug!(get_system_metrics(SM_SWAPBUTTON)) != 0
}

fn swap_button(left: bool) -> (KeyCode, i32) {
    unsafe {
        let mut left = left;
        if debug!(is_mouse_left_handed()) {
            left = !left;
        }

        match left {
            true => (KeyCode::MouseLeft, VK_LBUTTON.0 as i32),
            false => (KeyCode::MouseRight, VK_RBUTTON.0 as i32),
        }
    }
}

pub fn get_keycode(code: u16) -> (KeyCode, i32) {
    let get = |key: KeyCode| (key, code as i32);

    match VIRTUAL_KEY(code) {
        VK_LBUTTON => swap_button(true),
        VK_RBUTTON => swap_button(false),
        // VK_CANCEL
        VK_MBUTTON => get(KeyCode::MouseMiddle),
        VK_XBUTTON1 => get(KeyCode::MouseX1),
        VK_XBUTTON2 => get(KeyCode::MouseX2),
        VK_BACK => get(KeyCode::Backspace),
        VK_TAB => get(KeyCode::Tab),
        // VK_CLEAR
        VK_RETURN => get(KeyCode::Enter),
        // VK_SHIFT
        // VK_CONTROL
        // VK_MENU
        VK_PAUSE => get(KeyCode::Pause),
        VK_CAPITAL => get(KeyCode::CapsLock),
        // VK_KANA
        // VK_HANGEUL
        // VK_HANGUL
        // VK_IME_ON
        // VK_JUNJA
        // VK_FINAL
        // VK_HANJA
        // VK_KANJI
        // VK_IME_OFF
        VK_ESCAPE => get(KeyCode::Escape),
        // VK_CONVERT
        // VK_NONCONVERT
        // VK_ACCEPT
        // VK_MODECHANGE
        VK_SPACE => get(KeyCode::Space),
        VK_PRIOR => get(KeyCode::PageUp),
        VK_NEXT => get(KeyCode::PageDown),
        VK_END => get(KeyCode::End),
        VK_HOME => get(KeyCode::Home),
        VK_LEFT => get(KeyCode::ArrowLeft),
        VK_UP => get(KeyCode::ArrowUp),
        VK_RIGHT => get(KeyCode::ArrowRight),
        VK_DOWN => get(KeyCode::ArrowDown),
        // VK_SELECT
        // VK_PRINT
        // VK_EXECUTE
        VK_SNAPSHOT => get(KeyCode::PrintScreen),
        VK_INSERT => get(KeyCode::Insert),
        VK_DELETE => get(KeyCode::Delete),
        // VK_HELP
        VK_0 => get(KeyCode::Alpha0),
        VK_1 => get(KeyCode::Alpha1),
        VK_2 => get(KeyCode::Alpha2),
        VK_3 => get(KeyCode::Alpha3),
        VK_4 => get(KeyCode::Alpha4),
        VK_5 => get(KeyCode::Alpha5),
        VK_6 => get(KeyCode::Alpha6),
        VK_7 => get(KeyCode::Alpha7),
        VK_8 => get(KeyCode::Alpha8),
        VK_9 => get(KeyCode::Alpha9),

        VK_A => get(KeyCode::A),
        VK_B => get(KeyCode::B),
        VK_C => get(KeyCode::C),
        VK_D => get(KeyCode::D),
        VK_E => get(KeyCode::E),
        VK_F => get(KeyCode::F),
        VK_G => get(KeyCode::G),
        VK_H => get(KeyCode::H),
        VK_I => get(KeyCode::I),
        VK_J => get(KeyCode::J),
        VK_K => get(KeyCode::K),
        VK_L => get(KeyCode::L),
        VK_M => get(KeyCode::M),
        VK_N => get(KeyCode::N),
        VK_O => get(KeyCode::O),
        VK_P => get(KeyCode::P),
        VK_Q => get(KeyCode::Q),
        VK_R => get(KeyCode::R),
        VK_S => get(KeyCode::S),
        VK_T => get(KeyCode::T),
        VK_U => get(KeyCode::U),
        VK_V => get(KeyCode::V),
        VK_W => get(KeyCode::W),
        VK_X => get(KeyCode::X),
        VK_Y => get(KeyCode::Y),
        VK_Z => get(KeyCode::Z),

        VK_LWIN => get(KeyCode::LMeta),
        VK_RWIN => get(KeyCode::RMeta),

        // VK_APPS
        // VK_SLEEP
        VK_NUMPAD0 => get(KeyCode::NumPad0),
        VK_NUMPAD1 => get(KeyCode::NumPad1),
        VK_NUMPAD2 => get(KeyCode::NumPad2),
        VK_NUMPAD3 => get(KeyCode::NumPad3),
        VK_NUMPAD4 => get(KeyCode::NumPad4),
        VK_NUMPAD5 => get(KeyCode::NumPad5),
        VK_NUMPAD6 => get(KeyCode::NumPad6),
        VK_NUMPAD7 => get(KeyCode::NumPad7),
        VK_NUMPAD8 => get(KeyCode::NumPad8),
        VK_NUMPAD9 => get(KeyCode::NumPad9),
        VK_MULTIPLY => get(KeyCode::NumPadMultiply),
        VK_ADD => get(KeyCode::NumPadAdd),
        // VK_SEPARATOR
        VK_SUBTRACT => get(KeyCode::NumPadSubtract),
        VK_DECIMAL => get(KeyCode::NumPadDecimal),
        VK_DIVIDE => get(KeyCode::NumPadDivide),

        VK_F1 => get(KeyCode::F1),
        VK_F2 => get(KeyCode::F2),
        VK_F3 => get(KeyCode::F3),
        VK_F4 => get(KeyCode::F4),
        VK_F5 => get(KeyCode::F5),
        VK_F6 => get(KeyCode::F6),
        VK_F7 => get(KeyCode::F7),
        VK_F8 => get(KeyCode::F8),
        VK_F9 => get(KeyCode::F9),
        VK_F10 => get(KeyCode::F10),
        VK_F11 => get(KeyCode::F11),
        VK_F12 => get(KeyCode::F12),
        VK_F13 => get(KeyCode::F13),
        VK_F14 => get(KeyCode::F14),
        VK_F15 => get(KeyCode::F15),
        VK_F16 => get(KeyCode::F16),
        VK_F17 => get(KeyCode::F17),
        VK_F18 => get(KeyCode::F18),
        VK_F19 => get(KeyCode::F19),
        VK_F20 => get(KeyCode::F20),
        VK_F21 => get(KeyCode::F21),
        VK_F22 => get(KeyCode::F22),
        VK_F23 => get(KeyCode::F23),
        VK_F24 => get(KeyCode::F24),

        VK_NUMLOCK => get(KeyCode::NumLock),
        VK_SCROLL => get(KeyCode::ScrollLock),

        VK_LSHIFT => get(KeyCode::LShift),
        VK_RSHIFT => get(KeyCode::RShift),
        VK_LCONTROL => get(KeyCode::LControl),
        VK_RCONTROL => get(KeyCode::RControl),
        VK_LMENU => get(KeyCode::LAlt),
        VK_RMENU => get(KeyCode::RAlt),

        // VK_BROWSER_BACK
        // VK_BROWSER_FORWARD
        // VK_BROWSER_REFRESH
        // VK_BROWSER_STOP
        // VK_BROWSER_SEARCH
        // VK_BROWSER_FAVORITES
        // VK_BROWSER_HOME
        // VK_VOLUME_MUTE
        // VK_VOLUME_DOWN
        // VK_VOLUME_UP
        // VK_MEDIA_NEXT_TRACK
        // VK_MEDIA_PREV_TRACK
        // VK_MEDIA_STOP
        // VK_MEDIA_PLAY_PAUSE
        // VK_LAUNCH_MAIL
        // VK_LAUNCH_MEDIA_SELECT
        // VK_LAUNCH_APP1
        // VK_LAUNCH_APP2

        // VK_OEM_1
        // VK_OEM_PLUS
        // VK_OEM_COMMA
        // VK_OEM_MINUS
        // VK_OEM_PERIOD
        // VK_OEM_2
        // VK_OEM_3
        // VK_OEM_4
        // VK_OEM_5
        // VK_OEM_6
        // VK_OEM_7
        // VK_OEM_8

        // VK_OEM_102

        // VK_PROCESSKEY

        // VK_PACKET

        // VK_ATTN
        // VK_CRSEL
        // VK_EXSEL
        // VK_EREOF
        // VK_PLAY
        // VK_ZOOM
        // VK_NONAME
        // VK_PA1
        // VK_OEM_CLEAR
        _ => get(KeyCode::Unknown),
    }
}
