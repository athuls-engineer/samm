// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
fn ensure_single_instance_or_clean_environment() {
    use std::time::Duration;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let current_pid = std::process::id();
    let current_exe_stem = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_lowercase()))
        .unwrap_or_else(|| "samm".to_string());

    #[repr(C)]
    #[allow(non_snake_case)]
    struct PROCESSENTRY32W {
        dwSize: u32,
        cntUsage: u32,
        th32ProcessID: u32,
        th32DefaultHeapID: usize,
        th32ModuleID: u32,
        cntThreads: u32,
        th32ParentProcessID: u32,
        pcPriClassBase: i32,
        dwFlags: u32,
        szExeFile: [u16; 260],
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> *mut std::ffi::c_void;
        fn Process32FirstW(hSnapshot: *mut std::ffi::c_void, lppe: *mut PROCESSENTRY32W) -> i32;
        fn Process32NextW(hSnapshot: *mut std::ffi::c_void, lppe: *mut PROCESSENTRY32W) -> i32;
        fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
        fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> *mut std::ffi::c_void;
        fn TerminateProcess(hProcess: *mut std::ffi::c_void, uExitCode: u32) -> i32;
    }

    #[link(name = "user32")]
    extern "system" {
        fn EnumWindows(lpEnumFunc: Option<unsafe extern "system" fn(*mut std::ffi::c_void, isize) -> i32>, lParam: isize) -> i32;
        fn GetWindowThreadProcessId(hWnd: *mut std::ffi::c_void, lpdwProcessId: *mut u32) -> u32;
        fn ShowWindow(hWnd: *mut std::ffi::c_void, nCmdShow: i32) -> i32;
        fn SetForegroundWindow(hWnd: *mut std::ffi::c_void) -> i32;
        fn BringWindowToTop(hWnd: *mut std::ffi::c_void) -> i32;
    }

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(0x00000002 /* TH32CS_SNAPPROCESS */, 0);
        if !snapshot.is_null() && snapshot != -1isize as *mut std::ffi::c_void {
            let mut entry: PROCESSENTRY32W = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            let mut duplicate_pids = Vec::new();

            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    let len = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
                    let exe_name = OsString::from_wide(&entry.szExeFile[..len]).to_string_lossy().to_lowercase();

                    let is_samm_exe = exe_name == "samm.exe" || exe_name.starts_with(&current_exe_stem);
                    if is_samm_exe && entry.th32ProcessID != current_pid {
                        duplicate_pids.push(entry.th32ProcessID);
                    }

                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }
            CloseHandle(snapshot);

            // Check if any duplicate instance has a live window
            for pid in duplicate_pids {
                unsafe extern "system" fn check_window(hwnd: *mut std::ffi::c_void, lparam: isize) -> i32 {
                    let mut window_pid = 0u32;
                    GetWindowThreadProcessId(hwnd, &mut window_pid);
                    let target = *(lparam as *const (u32, bool));
                    if window_pid == target.0 {
                        // Found top-level window belonging to existing instance: restore and focus
                        ShowWindow(hwnd, 9 /* SW_RESTORE */);
                        SetForegroundWindow(hwnd);
                        BringWindowToTop(hwnd);
                        let ptr = lparam as *mut (u32, bool);
                        (*ptr).1 = true;
                        return 0; // stop enumerating
                    }
                    1 // continue enumerating
                }

                let mut state = (pid, false);
                EnumWindows(Some(check_window), &mut state as *mut (u32, bool) as isize);

                if state.1 {
                    // An active window exists and was restored to the front. Cleanly exit the duplicate launcher.
                    std::process::exit(0);
                } else {
                    // Stale headless instance holding file/socket locks. Terminate it so WebView2 can launch cleanly.
                    let h_proc = OpenProcess(0x0001 /* PROCESS_TERMINATE */, 0, pid);
                    if !h_proc.is_null() {
                        let _ = TerminateProcess(h_proc, 1);
                        CloseHandle(h_proc);
                    }
                }
            }
            // Allow operating system brief window to release file locks
            std::thread::sleep(Duration::from_millis(150));
        }
    }

    // Clean up any lingering lock files in EBWebView directory
    let local_app_data = match dirs::data_local_dir() {
        Some(d) => d,
        None => return,
    };

    let webview_dir = local_app_data.join("org.samm.modmanager").join("EBWebView");
    let lock_files = [
        webview_dir.join("Default").join("LOCK"),
        webview_dir.join("SingletonLock"),
        webview_dir.join("SingletonCookie"),
        webview_dir.join("SingletonSocket"),
    ];

    for lock in &lock_files {
        if lock.exists() {
            let _ = std::fs::remove_file(lock);
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            format!("{:?}", info)
        };
        let log_path = dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("SAMM")
            .join("samm_startup.log");
        let _ = std::fs::OpenOptions::new().create(true).append(true).open(&log_path).map(|mut f| {
            use std::io::Write;
            let _ = writeln!(f, "PANIC DETAILED: {}", payload);
            let _ = writeln!(f, "PANIC LOCATION: {:?}", info.location());
        });
    }));

    #[cfg(windows)]
    ensure_single_instance_or_clean_environment();

    samm_lib::run();
}
