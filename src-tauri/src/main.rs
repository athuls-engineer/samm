// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
fn sanitize_webview_environment() {
    let local_app_data = match dirs::data_local_dir() {
        Some(d) => d,
        None => return,
    };

    let webview_dir = local_app_data.join("org.samm.modmanager").join("EBWebView");
    let lock_file = webview_dir.join("Default").join("LOCK");

    let is_locked = if lock_file.exists() {
        std::fs::OpenOptions::new().read(true).write(true).open(&lock_file).is_err()
    } else {
        false
    };

    if is_locked {
        let _ = std::fs::OpenOptions::new().create(true).append(true).open("samm_startup.log").map(|mut f| {
            use std::io::Write;
            let _ = writeln!(f, "Detected stale EBWebView lock; terminating orphaned msedgewebview2 processes...");
        });

        // Terminate orphaned msedgewebview2 processes belonging to SAMM
        let _ = std::process::Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle", "Hidden",
                "-Command",
                "Get-CimInstance Win32_Process -Filter \"Name = 'msedgewebview2.exe'\" | Where-Object { $_.CommandLine -like '*SAMM.exe*' -or $_.CommandLine -like '*org.samm.modmanager*' } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }"
            ])
            .output();

        std::thread::sleep(std::time::Duration::from_millis(200));
        let _ = std::fs::remove_file(&lock_file);
    } else if lock_file.exists() {
        let _ = std::fs::remove_file(&lock_file);
    }

    // Safety fallback: if the default EBWebView is still locked or inaccessible, use an alternate user data folder
    let still_locked = if lock_file.exists() {
        std::fs::OpenOptions::new().read(true).write(true).open(&lock_file).is_err()
    } else {
        std::fs::create_dir_all(&webview_dir).is_err()
    };

    if still_locked {
        let fallback_dir = local_app_data.join("SAMM").join("EBWebView_SafeSession");
        let _ = std::fs::create_dir_all(&fallback_dir);
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &fallback_dir);
        let _ = std::fs::OpenOptions::new().create(true).append(true).open("samm_startup.log").map(|mut f| {
            use std::io::Write;
            let _ = writeln!(f, "Default EBWebView locked; falling back to safe session: {:?}", fallback_dir);
        });
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
    sanitize_webview_environment();

    samm_lib::run();
}
