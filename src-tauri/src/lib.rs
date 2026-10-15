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
pub mod commands;
#[cfg(test)]
mod tests;

use std::sync::Arc;
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

    let db = Database::init(&db_path).expect("Failed to initialize SAMM SQLite database");
    let source_manager = SourceManager::new();
    let download_manager = DownloadManager::new(db.clone());

    let app_state = Arc::new(AppState {
        db,
        source_manager,
        download_manager,
        app_data_dir,
    });

    tauri::Builder::default()
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
            commands::get_backups,
            commands::start_download,
            commands::get_download_jobs,
            commands::launch_game,
            commands::get_diagnostic_report,
            commands::get_settings,
            commands::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running SAMM desktop application");
}
