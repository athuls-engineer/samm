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
                    return false; // Active window exists and focused
                }
                // No active window found; mutex was abandoned or stale from previous run, proceed with startup
            }
        }
        true
    }
}

fn main() {
    let _ = std::fs::write("samm_startup.log", "SAMM main entry reached\n");
    #[cfg(windows)]
    {
        // Explicitly set AppUserModelID so Windows taskbar pins and displays the embedded icon
        #[link(name = "shell32")]
        extern "system" {
            fn SetCurrentProcessExplicitAppUserModelID(AppID: *const u16) -> i32;
        }
        let app_id: Vec<u16> = "org.samm.modmanager\0".encode_utf16().collect();
        unsafe {
            let _ = SetCurrentProcessExplicitAppUserModelID(app_id.as_ptr());
        }

        if !single_instance::check_or_focus() {
            let _ = std::fs::write("samm_startup.log", "Exiting: single_instance check returned false (already running)\n");
            return;
        }
    }

    let _ = std::fs::write("samm_startup.log", "Invoking samm_lib::run()\n");
    samm_lib::run();
}
