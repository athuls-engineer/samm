use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::State;
use crate::models::*;
use crate::database::Database;
use crate::sources::SourceManager;
use crate::downloads::DownloadManager;

pub struct AppState {
    pub db: Database,
    pub source_manager: SourceManager,
    pub download_manager: DownloadManager,
    pub app_data_dir: PathBuf,
}

#[tauri::command]
pub fn get_installations(state: State<'_, Arc<AppState>>) -> Result<Vec<GameInstallation>, String> {
    state.db.get_game_installations().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn inspect_game_path(path: String) -> Result<GameInstallation, String> {
    let p = Path::new(&path);
    crate::game_detection::inspect_game_installation(p)
}

#[tauri::command]
pub fn save_game_installation(
    game: GameInstallation,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    state.db.save_game_installation(&game).map_err(|e| e.to_string())?;

    // Check if profile exists; if not, create default profile
    if let Ok(profiles) = state.db.get_profiles(&game.id) {
        if profiles.is_empty() {
            let _ = crate::profiles::create_default_profiles(&game.id, &state.db);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn set_active_installation(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    state.db.set_active_installation(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_game_installation(
    id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    state.db.delete_game_installation(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn discover_installations() -> Vec<GameInstallation> {
    crate::game_detection::discover_known_installations()
}

#[tauri::command]
pub fn list_sources(state: State<'_, Arc<AppState>>) -> Vec<ModSource> {
    state.source_manager.list_sources()
}

#[tauri::command]
pub async fn search_mods(
    query: String,
    category: Option<String>,
    source_id: Option<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<ModRecord>, String> {
    // 1. Query cached mods in database
    let mut cached = state.db.query_cached_mods(&query, category.as_deref(), source_id.as_deref())
        .unwrap_or_default();

    // 2. Fetch live results from source manager
    let live_results = state.source_manager.search_all(&query, category.as_deref(), source_id.as_deref()).await;

    for r in &live_results {
        let _ = state.db.cache_mod_record(r);
        if !cached.iter().any(|c| c.id == r.id) {
            cached.push(r.clone());
        }
    }

    Ok(cached)
}

#[tauri::command]
pub async fn get_mod_details(
    source_id: String,
    source_mod_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<ModRecord, String> {
    state.source_manager.get_details(&source_id, &source_mod_id).await
}

#[tauri::command]
pub fn inspect_archive(archive_path: String) -> Result<ArchiveInspection, String> {
    let p = Path::new(&archive_path);
    crate::archives::inspect_archive(p)
}

#[tauri::command]
pub fn generate_installation_plan(
    archive_path: String,
    game_id: String,
    mod_id: String,
    mod_title: String,
    state: State<'_, Arc<AppState>>,
) -> Result<InstallationPlan, String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game installation '{}' not found", game_id))?;

    let p = Path::new(&archive_path);
    crate::installation::generate_installation_plan(p, &game, &mod_id, &mod_title)
}

#[tauri::command]
pub fn install_mod(
    archive_path: String,
    plan: InstallationPlan,
    game_id: String,
    profile_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<InstalledMod, String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game installation '{}' not found", game_id))?;

    let backup_dir = state.app_data_dir.join("backups");
    let p = Path::new(&archive_path);

    crate::installation::execute_installation_transaction(
        p,
        &plan,
        &game,
        &profile_id,
        &backup_dir,
        &state.db,
    )
}

#[tauri::command]
pub fn uninstall_mod(
    mod_id: String,
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game installation '{}' not found", game_id))?;

    crate::installation::uninstall_mod(&mod_id, &game, &state.db)
}

#[tauri::command]
pub fn get_installed_mods(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<InstalledMod>, String> {
    state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_mod_state(
    mod_id: String,
    enabled: bool,
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let mut mods = state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())?;
    if let Some(m) = mods.iter_mut().find(|m| m.id == mod_id) {
        m.enabled = enabled;
        state.db.save_installed_mod(m).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_profiles(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<Profile>, String> {
    state.db.get_profiles(&game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_profile(
    game_id: String,
    name: String,
    description: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Profile, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let p = Profile {
        id: uuid::Uuid::new_v4().to_string(),
        game_installation_id: game_id,
        name,
        description,
        is_default: false,
        is_active: false,
        installed_mod_ids: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    };
    state.db.save_profile(&p).map_err(|e| e.to_string())?;
    Ok(p)
}

#[tauri::command]
pub fn set_active_profile(
    profile_id: String,
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let mut profiles = state.db.get_profiles(&game_id).map_err(|e| e.to_string())?;
    for p in &mut profiles {
        p.is_active = p.id == profile_id;
        state.db.save_profile(p).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_profile(
    profile_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    state.db.delete_profile(&profile_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_profile(
    profile_id: String,
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    crate::profiles::export_profile_manifest(&profile_id, &game_id, &state.db)
}

#[tauri::command]
pub fn import_profile(
    manifest_json: String,
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Profile, String> {
    crate::profiles::import_profile_manifest(&manifest_json, &game_id, &state.db)
}

#[tauri::command]
pub fn get_conflicts(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<FileConflict>, String> {
    let installed_mods = state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())?;
    Ok(crate::conflicts::scan_existing_installed_conflicts(&installed_mods))
}

#[tauri::command]
pub fn get_backups(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<BackupRecord>, String> {
    state.db.get_backups(&game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_download(
    mod_id: String,
    mod_title: String,
    source_id: String,
    source_name: String,
    url: String,
    expected_sha256: Option<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<DownloadJob, String> {
    let dl_dir = state.app_data_dir.join("downloads");
    state.download_manager.start_download(
        &mod_id,
        &mod_title,
        &source_id,
        &source_name,
        &url,
        &dl_dir,
        expected_sha256,
    )
}

#[tauri::command]
pub fn get_download_jobs(state: State<'_, Arc<AppState>>) -> Result<Vec<DownloadJob>, String> {
    state.db.get_download_jobs().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn launch_game(
    game_id: String,
    custom_args: Vec<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<u32, String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game installation '{}' not found", game_id))?;

    crate::game_launch::launch_game(&game, &custom_args)
}

#[tauri::command]
pub fn get_diagnostic_report(state: State<'_, Arc<AppState>>) -> Result<DiagnosticReport, String> {
    let games = state.db.get_game_installations().unwrap_or_default();
    let active_game = games.into_iter().find(|g| g.is_active);
    Ok(crate::diagnostics::generate_diagnostic_report(&state.db, active_game))
}

#[tauri::command]
pub fn get_settings(state: State<'_, Arc<AppState>>) -> Result<AppSettings, String> {
    let default_game = state.db.get_setting("default_game_id").unwrap_or(None);
    let download_dir = state.db.get_setting("download_dir")
        .unwrap_or(None)
        .unwrap_or_else(|| state.app_data_dir.join("downloads").to_string_lossy().to_string());
    let backup_dir = state.db.get_setting("backup_dir")
        .unwrap_or(None)
        .unwrap_or_else(|| state.app_data_dir.join("backups").to_string_lossy().to_string());

    Ok(AppSettings {
        default_game_installation_id: default_game,
        download_directory: download_dir,
        backup_directory: backup_dir,
        max_concurrent_downloads: 3,
        require_install_confirmation: true,
        auto_check_updates: false,
        theme: "dark".to_string(),
        log_level: "info".to_string(),
    })
}

#[tauri::command]
pub fn save_settings(
    settings: AppSettings,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    if let Some(ref gid) = settings.default_game_installation_id {
        let _ = state.db.set_setting("default_game_id", gid);
    }
    let _ = state.db.set_setting("download_dir", &settings.download_directory);
    let _ = state.db.set_setting("backup_dir", &settings.backup_directory);
    let _ = state.db.set_setting("theme", &settings.theme);
    Ok(())
}
