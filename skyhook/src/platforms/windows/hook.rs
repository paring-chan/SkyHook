// unsafe stuff
extern crate user32;
extern crate winapi;

// safe stuff
extern crate winsafe;

//#region Constants
const KEY_PRESSED: i32 = 0x8000;
const WM_KEYDOWN: i32 = 0x0100;
const WM_KEYUP: i32 = 0x0101;
const WM_SYSKEYDOWN: i32 = 0x0104;
const WM_SYSKEYUP: i32 = 0x0105;
const WH_KEYBOARD_LL: i32 = 13;
const WH_MOUSE_LL: i32 = 14;
//#endregion

//#region Commons
static mut hook_id: winsafe::HHOOK = 0;

pub fn start() -> () {
    unsafe {
        let hook_id =
            user32::SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), std::ptr::null_mut(), 0);

        // Do something

        // Release hook
        user32::UnhookWindowsHookEx(hook_id);
    }
}

pub fn stop() -> () {}
//#endregion

extern "system" fn hook_callback(code: i32, wParam: u64, lParam: i64) -> i64 {
    unsafe {
        assert!(
            hook_id != 0,
            format!(
                "Windows threw an error with code {}",
                winapi::um::errhandlingapi::GetLastError()
            )
        ); // GetLastError is an unsafe method
    }

    return 0;
}
