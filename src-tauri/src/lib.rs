pub mod models;
pub mod database;
pub mod game_detection;
pub mod archives;
pub mod installation;
pub mod conflicts;
pub mod profiles;
pub mod sources;
pub mod downloads;
pub mod game_launch;
pub mod diagnostics;
pub mod existing_mods;
pub mod commands;
#[cfg(test)]
mod tests;

use std::sync::Arc;
use tauri::Manager;
use commands::AppState;
use database::Database;
use downloads::DownloadManager;
use sources::SourceManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("SAMM");

    let _ = std::fs::create_dir_all(&app_data_dir);
    let db_path = app_data_dir.join("samm.db");
    let log_file = app_data_dir.join("samm_startup.log");

    let log = |msg: &str| {
        let _ = std::fs::OpenOptions::new().create(true).append(true).open(&log_file).map(|mut f| {
            use std::io::Write;
            let _ = writeln!(f, "{}", msg);
        });
    };

    log(&format!("Initializing SQLite DB at {:?}", db_path));

    let db = match Database::init(&db_path) {
        Ok(d) => d,
        Err(_) => {
            std::thread::sleep(std::time::Duration::from_millis(200));
            match Database::init(&db_path) {
                Ok(d) => d,
                Err(e) => {
                    log(&format!("Database init retry failed: {:?}; opening recovered db", e));
                    let backup_db_path = app_data_dir.join("samm_recovered.db");
                    Database::init(&backup_db_path).unwrap_or_else(|_| Database::in_memory().expect("In-memory DB fallback"))
                }
            }
        }
    };

    log("[CHECKPOINT 1] DB ready, creating app_state");

    let source_manager = SourceManager::new();
    let download_manager = DownloadManager::new(db.clone());

    let app_state = Arc::new(AppState {
        db,
        source_manager,
        download_manager,
        app_data_dir,
    });

    log("[CHECKPOINT 2] Building tauri app");

    let log_file_clone1 = log_file.clone();
    let log_file_clone2 = log_file.clone();
    let log_file_clone3 = log_file.clone();

    let app = match tauri::Builder::default()
        .setup(move |app| {
            let _ = std::fs::OpenOptions::new().create(true).append(true).open(&log_file_clone1).map(|mut f| {
                use std::io::Write;
                let _ = writeln!(f, "[CHECKPOINT 3] Setup hook running");
            });

            if let Some(window) = app.get_webview_window("main") {
                if let Ok(icon) = tauri::image::Image::from_bytes(include_bytes!("../icons/icon.png")) {
                    let _ = window.set_icon(icon);
                }

                #[cfg(windows)]
                {
                    use std::ptr::null_mut;
                    if let Ok(hwnd) = window.hwnd() {
                        let hwnd_ptr = hwnd.0 as *mut std::ffi::c_void;
                        #[link(name = "user32")]
                        extern "system" {
                            fn SendMessageW(hWnd: *mut std::ffi::c_void, Msg: u32, wParam: usize, lParam: isize) -> isize;
                            fn LoadImageW(
                                hInst: *mut std::ffi::c_void,
                                name: *const u16,
                                type_: u32,
                                cx: i32,
                                cy: i32,
                                fuLoad: u32,
                            ) -> *mut std::ffi::c_void;
                            fn GetSystemMetrics(nIndex: i32) -> i32;
                        }
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn GetModuleHandleW(lpModuleName: *const u16) -> *mut std::ffi::c_void;
                        }

                        unsafe {
                            let h_module = GetModuleHandleW(null_mut());
                            let sm_cx = GetSystemMetrics(49); // SM_CXSMICON
                            let sm_cy = GetSystemMetrics(50); // SM_CYSMICON
                            let lg_cx = GetSystemMetrics(11); // SM_CXICON
                            let lg_cy = GetSystemMetrics(12); // SM_CYICON

                            let h_icon_big = LoadImageW(h_module, 1 as *const u16, 1 /* IMAGE_ICON */, lg_cx, lg_cy, 0);
                            let h_icon_small = LoadImageW(h_module, 1 as *const u16, 1 /* IMAGE_ICON */, sm_cx, sm_cy, 0);

                            if !h_icon_big.is_null() {
                                SendMessageW(hwnd_ptr, 0x0080 /* WM_SETICON */, 1 /* ICON_BIG */, h_icon_big as isize);
                            }
                            if !h_icon_small.is_null() {
                                SendMessageW(hwnd_ptr, 0x0080 /* WM_SETICON */, 0 /* ICON_SMALL */, h_icon_small as isize);
                            }
                        }
                    }
                }
            }
            Ok(())
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_installations,
            commands::inspect_game_path,
            commands::save_game_installation,
            commands::set_active_installation,
            commands::delete_game_installation,
            commands::discover_installations,
            commands::list_sources,
            commands::search_mods,
            commands::get_mod_details,
            commands::inspect_archive,
            commands::generate_installation_plan,
            commands::install_mod,
            commands::uninstall_mod,
            commands::get_installed_mods,
            commands::toggle_mod_state,
            commands::get_profiles,
            commands::create_profile,
            commands::set_active_profile,
            commands::delete_profile,
            commands::export_profile,
            commands::import_profile,
            commands::get_conflicts,
            commands::check_for_conflicts,
            commands::resolve_conflict,
            commands::auto_resolve_conflicts,
            commands::get_backups,
            commands::start_download,
            commands::get_download_jobs,
            commands::cancel_download,
            commands::delete_download,
            commands::clear_completed_downloads,
            commands::launch_game,
            commands::get_diagnostic_report,
            commands::get_settings,
            commands::save_settings,
            commands::sync_existing_mods,
            commands::pick_folder,
            commands::pick_archive_file,
            commands::open_path_in_explorer,
            commands::open_url_in_browser,
            commands::is_game_process_running,
            commands::kill_game_process,
            commands::batch_toggle_mods,
            commands::batch_uninstall_mods,
            commands::get_recent_downloaded_archives,
            commands::clear_download_cache,
            commands::purge_stale_backups,
        ])
        .build(tauri::generate_context!()) {
            Ok(a) => {
                let _ = std::fs::OpenOptions::new().create(true).append(true).open(&log_file_clone2).map(|mut f| {
                    use std::io::Write;
                    let _ = writeln!(f, "[CHECKPOINT 4] App built OK, calling app.run()");
                });
                a
            }
            Err(e) => {
                let _ = std::fs::OpenOptions::new().create(true).append(true).open(&log_file_clone2).map(|mut f| {
                    use std::io::Write;
                    let _ = writeln!(f, "Tauri build error: {:?}", e);
                });
                return;
            }
        };

    app.run(move |_app_handle, event| {
        let _ = std::fs::OpenOptions::new().create(true).append(true).open(&log_file_clone3).map(|mut f| {
            use std::io::Write;
            let _ = writeln!(f, "App RunEvent: {:?}", event);
        });
    });
}
