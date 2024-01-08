use io_kit_sys::hid::usage_tables::*;
use crate::keycodes::VK;
use crate::keycodes::VK::*;

#[allow(non_upper_case_globals)]
pub fn raw_keycode_to_vk(code: u32) -> VK {
    match code {
        kHIDUsage_KeyboardA => A,
        kHIDUsage_KeyboardB => B,
        kHIDUsage_KeyboardC => C,
        kHIDUsage_KeyboardD => D,
        kHIDUsage_KeyboardE => E,
        kHIDUsage_KeyboardF => F,
        kHIDUsage_KeyboardG => G,
        kHIDUsage_KeyboardH => H,
        kHIDUsage_KeyboardI => I,
        kHIDUsage_KeyboardJ => J,
        kHIDUsage_KeyboardK => K,
        kHIDUsage_KeyboardL => L,
        kHIDUsage_KeyboardM => M,
        kHIDUsage_KeyboardN => N,
        kHIDUsage_KeyboardO => O,
        kHIDUsage_KeyboardP => P,
        kHIDUsage_KeyboardQ => Q,
        kHIDUsage_KeyboardR => R,
        kHIDUsage_KeyboardS => S,
        kHIDUsage_KeyboardT => T,
        kHIDUsage_KeyboardU => U,
        kHIDUsage_KeyboardV => V,
        kHIDUsage_KeyboardW => W,
        kHIDUsage_KeyboardX => X,
        kHIDUsage_KeyboardY => Y,
        kHIDUsage_KeyboardZ => Z,
        kHIDUsage_Keyboard1 => Alpha1,
        kHIDUsage_Keyboard2 => Alpha2,
        kHIDUsage_Keyboard3 => Alpha3,
        kHIDUsage_Keyboard4 => Alpha4,
        kHIDUsage_Keyboard5 => Alpha5,
        kHIDUsage_Keyboard6 => Alpha6,
        kHIDUsage_Keyboard7 => Alpha7,
        kHIDUsage_Keyboard8 => Alpha8,
        kHIDUsage_Keyboard9 => Alpha9,
        kHIDUsage_Keyboard0 => Alpha0,
        kHIDUsage_KeyboardReturnOrEnter => Enter,
        kHIDUsage_KeyboardEscape => Escape,
        kHIDUsage_KeyboardDeleteOrBackspace => Backspace,
        kHIDUsage_KeyboardTab => Tab,
        kHIDUsage_KeyboardSpacebar => Space,
        kHIDUsage_KeyboardHyphen => Minus,
        kHIDUsage_KeyboardEqualSign => Equal,
        kHIDUsage_KeyboardOpenBracket => LeftBrace,
        kHIDUsage_KeyboardCloseBracket => RightBrace,
        kHIDUsage_KeyboardBackslash => BackSlash,
        // kHIDUsage_KeyboardNonUSPound => Unknown,
        kHIDUsage_KeyboardSemicolon => Semicolon,
        kHIDUsage_KeyboardQuote => Apostrophe,
        kHIDUsage_KeyboardGraveAccentAndTilde => Grave,
        kHIDUsage_KeyboardComma => Comma,
        kHIDUsage_KeyboardPeriod => Dot,
        kHIDUsage_KeyboardSlash => Slash,
        kHIDUsage_KeyboardCapsLock => CapsLock,
        kHIDUsage_KeyboardF1 => F1,
        kHIDUsage_KeyboardF2 => F2,
        kHIDUsage_KeyboardF3 => F3,
        kHIDUsage_KeyboardF4 => F4,
        kHIDUsage_KeyboardF5 => F5,
        kHIDUsage_KeyboardF6 => F6,
        kHIDUsage_KeyboardF7 => F7,
        kHIDUsage_KeyboardF8 => F8,
        kHIDUsage_KeyboardF9 => F9,
        kHIDUsage_KeyboardF10 => F10,
        kHIDUsage_KeyboardF11 => F11,
        kHIDUsage_KeyboardF12 => F12,
        kHIDUsage_KeyboardPrintScreen => PrintScreen,
        kHIDUsage_KeyboardScrollLock => ScrollLock,
        kHIDUsage_KeyboardPause => PauseBreak,
        kHIDUsage_KeyboardInsert => Insert,
        kHIDUsage_KeyboardHome => Home,
        kHIDUsage_KeyboardPageUp => PageUp,
        kHIDUsage_KeyboardDeleteForward => Delete,
        kHIDUsage_KeyboardEnd => End,
        kHIDUsage_KeyboardPageDown => PageDown,
        kHIDUsage_KeyboardRightArrow => ArrowRight,
        kHIDUsage_KeyboardLeftArrow => ArrowLeft,
        kHIDUsage_KeyboardDownArrow => ArrowDown,
        kHIDUsage_KeyboardUpArrow => ArrowUp,
        kHIDUsage_KeypadNumLock => NumLock,
        kHIDUsage_KeypadSlash => KeypadSlash,
        kHIDUsage_KeypadAsterisk => KeypadAsterisk,
        kHIDUsage_KeypadHyphen => KeypadMinus,
        kHIDUsage_KeypadPlus => KeypadPlus,
        kHIDUsage_KeypadEnter => KeypadEnter,
        kHIDUsage_Keypad1 => Keypad1,
        kHIDUsage_Keypad2 => Keypad2,
        kHIDUsage_Keypad3 => Keypad3,
        kHIDUsage_Keypad4 => Keypad4,
        kHIDUsage_Keypad5 => Keypad5,
        kHIDUsage_Keypad6 => Keypad6,
        kHIDUsage_Keypad7 => Keypad7,
        kHIDUsage_Keypad8 => Keypad8,
        kHIDUsage_Keypad9 => Keypad9,
        kHIDUsage_Keypad0 => Keypad0,
        kHIDUsage_KeypadPeriod => KeypadDot,
        kHIDUsage_KeyboardNonUSBackslash => BackSlash,
        // Application
        // Power
        // KeypadEqualSign
        kHIDUsage_KeyboardF13 => F13,
        kHIDUsage_KeyboardF14 => F14,
        kHIDUsage_KeyboardF15 => F15,
        kHIDUsage_KeyboardF16 => F16,
        kHIDUsage_KeyboardF17 => F17,
        kHIDUsage_KeyboardF18 => F18,
        kHIDUsage_KeyboardF19 => F19,
        kHIDUsage_KeyboardF20 => F20,
        kHIDUsage_KeyboardF21 => F21,
        kHIDUsage_KeyboardF22 => F22,
        kHIDUsage_KeyboardF23 => F23,
        kHIDUsage_KeyboardF24 => F24,
        // Skip
        kHIDUsage_KeyboardReturn => Enter,
        kHIDUsage_KeyboardLeftControl => LControl,
        kHIDUsage_KeyboardRightControl => RControl,
        kHIDUsage_KeyboardLeftAlt => LAlt,
        kHIDUsage_KeyboardLeftGUI => Super,
        kHIDUsage_KeyboardRightAlt => RAlt,
        kHIDUsage_KeyboardRightGUI => Super,
        _ => Unknown
    }
}