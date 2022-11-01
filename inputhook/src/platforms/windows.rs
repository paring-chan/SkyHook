extern crate user32;
extern crate winapi;

//#region Constants
const KEY_PRESSED: i32 = 0x8000;
const WM_KEYDOWN: i32 = 0x0100;
const WM_KEYUP: i32 = 0x0101;
const WM_SYSKEYDOWN: i32 = 0x0104;
const WM_SYSKEYUP: i32 = 0x0105;
const WH_KEYBOARD_LL: i32 = 13;
//#endregion


fn main() {
    let hook_id = user32::SetWindowsHookExA(
        WH_KEYBOARD_LL,
        Some(hook_callback),
        // No idea what goes here ,
        0,
    );
}

fn hook_callback(code: i32, wParam: u64, lParam: i64) -> i64 {
    
}
/**
        internal delegate IntPtr LowLevelKeyboardProc(int nCode, IntPtr wParam, IntPtr lParam);

		[DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
		internal static extern IntPtr SetWindowsHookEx(int idHook, LowLevelKeyboardProc lpfn, IntPtr hMod, uint dwThreadId);

		[DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
		[return: MarshalAs(UnmanagedType.Bool)]
		internal static extern bool UnhookWindowsHookEx(IntPtr hhk);

		[DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
		internal static extern IntPtr CallNextHookEx(IntPtr hhk, int nCode, IntPtr wParam, IntPtr lParam);

		[DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
		internal static extern short GetKeyState(int nVirtKey);

		[DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
		internal static extern uint MapVirtualKey(uint uCode, uint uMapType);

		internal static class KeyStateConstants
		{
			internal const int KEY_PRESSED = 0x8000;
			internal const int WM_KEYDOWN = 0x0100;
			internal const int WM_KEYUP = 0x0101;
			internal const int WM_SYSKEYDOWN = 0x0104;
			internal const int WM_SYSKEYUP = 0x0105;
			internal const int WH_KEYBOARD_LL = 13;
		}
*/