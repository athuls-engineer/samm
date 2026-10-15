// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
mod single_instance {
    use std::ptr::null_mut;

    #[link(name = "user32")]
    extern "system" {
        fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> *mut std::ffi::c_void;
        fn SetForegroundWindow(hWnd: *mut std::ffi::c_void) -> i32;
        fn ShowWindow(hWnd: *mut std::ffi::c_void, nCmdShow: i32) -> i32;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn CreateMutexW(
            lpMutexAttributes: *mut std::ffi::c_void,
            bInitialOwner: i32,
            lpName: *const u16,
        ) -> *mut std::ffi::c_void;
        fn GetLastError() -> u32;
    }

    pub fn check_or_focus() -> bool {
        let name: Vec<u16> = "Global\\SAMM_SanAndreasModManager_Instance\0"
            .encode_utf16()
            .collect();

        unsafe {
            let _handle = CreateMutexW(null_mut(), 1, name.as_ptr());
            if GetLastError() == 183 {
                // 183 = ERROR_ALREADY_EXISTS: Another SAMM instance is already running
                let win_title: Vec<u16> = "San Andreas Mod Manager (SAMM)\0"
                    .encode_utf16()
                    .collect();
                let hwnd = FindWindowW(null_mut(), win_title.as_ptr());
                if !hwnd.is_null() {
                    ShowWindow(hwnd, 9); // SW_RESTORE
                    SetForegroundWindow(hwnd);
                }
                return false;
            }
        }
        true
    }
}

fn main() {
    #[cfg(windows)]
    {
        if !single_instance::check_or_focus() {
            // Already running: Focused the existing window and exiting gracefully without WebView2 lock collision
            return;
        }
    }

    samm_lib::run();
}
