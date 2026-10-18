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
    let singleton_lock = webview_dir.join("SingletonLock");

    if lock_file.exists() {
        let _ = std::fs::remove_file(&lock_file);
    }
    if singleton_lock.exists() {
        let _ = std::fs::remove_file(&singleton_lock);
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
