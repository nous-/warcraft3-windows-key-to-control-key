extern crate winapi;

use std::ptr::null_mut;
use std::thread;
use std::time::Duration;
use winapi::shared::minwindef::{BYTE, DWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    keybd_event, CallNextHookEx, DispatchMessageW, GetMessageW, PostQuitMessage, SetWindowsHookExW,
    TranslateMessage, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, KEYEVENTF_KEYUP, VK_CONTROL, VK_LWIN,
    WH_KEYBOARD_LL, WM_KEYDOWN,
};

fn main() {
    println!("Pressing the left Windows key will simulate the Ctrl key. Enjoy");
    println!("andrew@variancestudios.com");
    println!("Ctrl+C to exit");

    let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), null_mut(), 0) };

    if hook.is_null() {
        panic!("Failed to set keyboard hook");
    }

    loop {
        let mut msg = winapi::um::winuser::MSG {
            hwnd: 0 as HWND,
            message: 0 as UINT,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as DWORD,
            pt: winapi::shared::windef::POINT { x: 0, y: 0 },
        };

        let get_result = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };

        if get_result == -1 {
            panic!("Failed to get message");
        }

        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        if msg.message == winapi::um::winuser::WM_QUIT {
            break;
        }

        thread::sleep(Duration::from_millis(10));
    }

    unsafe {
        UnhookWindowsHookEx(hook);
        PostQuitMessage(0);
    }
}

unsafe extern "system" fn keyboard_hook(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code >= 0 {
        let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode as i32;

        if vk_code == VK_LWIN {
            if w_param == WM_KEYDOWN as WPARAM {
                keybd_event(VK_CONTROL as BYTE, 0, 0, 0);
            } else {
                keybd_event(VK_CONTROL as BYTE, 0, KEYEVENTF_KEYUP, 0);
            }
            return 1;
        }
    }
    return CallNextHookEx(null_mut(), code, w_param, l_param);
}
