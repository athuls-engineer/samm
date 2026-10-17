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
    let active_profile_id = if let Ok(profiles) = state.db.get_profiles(&game.id) {
        if profiles.is_empty() {
            let _ = crate::profiles::create_default_profiles(&game.id, &state.db);
            "default_profile".to_string()
        } else {
            profiles.iter().find(|p| p.is_active).map(|p| p.id.clone()).unwrap_or_else(|| profiles[0].id.clone())
        }
    } else {
        "default_profile".to_string()
    };

    // Automatically detect and register any existing mods already in the game folder
    let _ = crate::existing_mods::sync_existing_mods_into_db(&game, &active_profile_id, &state.db);

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
    // 1. In-memory curated catalog search is lightning fast (sub-millisecond)
    let live_results = state.source_manager.search_all(&query, category.as_deref(), source_id.as_deref()).await;

    // 2. Combine with any user-imported custom mods from SQLite
    let custom_cached = state.db.query_cached_mods(&query, category.as_deref(), source_id.as_deref())
        .unwrap_or_default();

    if custom_cached.is_empty() {
        return Ok(live_results);
    }

    let mut results = live_results;
    for c in custom_cached {
        if !results.iter().any(|r| r.id == c.id) {
            results.push(c);
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn get_mod_details(
    source_id: String,
    source_mod_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<ModRecord, String> {
    state.source_manager.get_details(&source_id, &source_mod_id).await
}

fn resolve_archive_path(archive_path: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(archive_path);
    if p.exists() {
        return Ok(p);
    }

    if archive_path.ends_with(".part") {
        let stripped = PathBuf::from(archive_path.trim_end_matches(".part"));
        if stripped.exists() {
            return Ok(stripped);
        }
    } else {
        let with_part = PathBuf::from(format!("{}.part", archive_path));
        if with_part.exists() {
            return Ok(with_part);
        }
    }

    Err(format!("Archive file not found on disk: '{}'", archive_path))
}

#[tauri::command]
pub fn inspect_archive(archive_path: String) -> Result<ArchiveInspection, String> {
    let p = resolve_archive_path(&archive_path)?;
    crate::archives::inspect_archive(&p)
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

    let p = resolve_archive_path(&archive_path)?;
    crate::installation::generate_installation_plan(&p, &game, &mod_id, &mod_title)
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
    let p = resolve_archive_path(&archive_path)?;

    crate::installation::execute_installation_transaction(
        &p,
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
    let mods = state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())?;
    if mods.is_empty() {
        // Initial scan for newly imported/detected games with no cached records yet
        if let Ok(games) = state.db.get_game_installations() {
            if let Some(game) = games.into_iter().find(|g| g.id == game_id) {
                let active_profile_id = if let Ok(profiles) = state.db.get_profiles(&game.id) {
                    profiles.iter().find(|p| p.is_active).map(|p| p.id.clone()).unwrap_or_else(|| "default_profile".to_string())
                } else {
                    "default_profile".to_string()
                };
                let _ = crate::existing_mods::sync_existing_mods_into_db(&game, &active_profile_id, &state.db);
                return state.db.get_installed_mods(&game_id).map_err(|e| e.to_string());
            }
        }
    }
    Ok(mods)
}

#[tauri::command]
pub fn toggle_mod_state(
    mod_id: String,
    enabled: bool,
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game '{}' not found", game_id))?;
    let game_root = Path::new(&game.root_path);

    let mut mods = state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())?;
    if let Some(m) = mods.iter_mut().find(|m| m.id == mod_id) {
        m.enabled = enabled;
        crate::installation::apply_physical_mod_state(m, enabled, game_root);
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
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game '{}' not found", game_id))?;
    let installed_mods = state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())?;
    Ok(crate::conflicts::scan_all_conflicts(&game, &installed_mods))
}

#[tauri::command]
pub fn check_for_conflicts(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<FileConflict>, String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game '{}' not found", game_id))?;

    // 1. Rescan existing mods on disk to ensure any external files are registered
    let profiles = state.db.get_profiles(&game_id).unwrap_or_default();
    let active_profile_id = profiles.iter().find(|p| p.is_active).map(|p| p.id.clone())
        .unwrap_or_else(|| profiles.get(0).map(|p| p.id.clone()).unwrap_or_else(|| "default_profile".to_string()));
    let _ = crate::existing_mods::sync_existing_mods_into_db(&game, &active_profile_id, &state.db);

    // 2. Scan for all conflicts across ModLoader and game root
    let installed_mods = state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())?;
    let detected = crate::conflicts::scan_all_conflicts(&game, &installed_mods);

    // 3. Persist detected conflicts in DB
    for c in &detected {
        let _ = state.db.save_conflict(c);
    }

    Ok(detected)
}

#[tauri::command]
pub fn resolve_conflict(
    game_id: String,
    destination_rel_path: String,
    keep_mod_id: String,
    yield_mod_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game '{}' not found", game_id))?;
    let root_path = Path::new(&game.root_path);

    // If destination_rel_path is in game root or modloader, physically disable the yielding file
    let target = root_path.join(&destination_rel_path);
    if target.is_file() {
        let disabled_name = format!("{}.disabled", target.to_string_lossy());
        let _ = std::fs::rename(&target, &disabled_name);
    }

    state.db.resolve_file_conflict(&destination_rel_path, &keep_mod_id, &yield_mod_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn auto_resolve_conflicts(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<ConflictResolutionResult, String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game '{}' not found", game_id))?;
    crate::conflicts::auto_resolve_all_conflicts_physical(&game, &state.db)
}

#[tauri::command]
pub fn get_backups(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<BackupRecord>, String> {
    state.db.get_backups(&game_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_download(
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
    let max_dls = state.db.get_setting("max_concurrent_downloads")
        .unwrap_or(None)
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(3);
    let req_confirm = state.db.get_setting("require_install_confirmation")
        .unwrap_or(None)
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true);
    let auto_updates = state.db.get_setting("auto_check_updates")
        .unwrap_or(None)
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);
    let theme = state.db.get_setting("theme")
        .unwrap_or(None)
        .unwrap_or_else(|| "cyberpunk".to_string());
    let log_level = state.db.get_setting("log_level")
        .unwrap_or(None)
        .unwrap_or_else(|| "info".to_string());
    let default_launch_args = state.db.get_setting("default_launch_args")
        .unwrap_or(None)
        .or_else(|| Some("-novid".to_string()));
    let default_aspect_ratio = state.db.get_setting("default_aspect_ratio")
        .unwrap_or(None)
        .or_else(|| Some("16:9".to_string()));
    let auto_resolve_conflicts = state.db.get_setting("auto_resolve_conflicts")
        .unwrap_or(None)
        .map(|v| v == "true" || v == "1")
        .or(Some(true));
    let auto_detect_game_on_startup = state.db.get_setting("auto_detect_game_on_startup")
        .unwrap_or(None)
        .map(|v| v == "true" || v == "1")
        .or(Some(true));

    Ok(AppSettings {
        default_game_installation_id: default_game,
        download_directory: download_dir,
        backup_directory: backup_dir,
        max_concurrent_downloads: max_dls,
        require_install_confirmation: req_confirm,
        auto_check_updates: auto_updates,
        theme,
        log_level,
        default_launch_args,
        default_aspect_ratio,
        auto_resolve_conflicts,
        auto_detect_game_on_startup,
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
    let _ = state.db.set_setting("max_concurrent_downloads", &settings.max_concurrent_downloads.to_string());
    let _ = state.db.set_setting("require_install_confirmation", if settings.require_install_confirmation { "true" } else { "false" });
    let _ = state.db.set_setting("auto_check_updates", if settings.auto_check_updates { "true" } else { "false" });
    let _ = state.db.set_setting("theme", &settings.theme);
    let _ = state.db.set_setting("log_level", &settings.log_level);
    if let Some(ref args) = settings.default_launch_args {
        let _ = state.db.set_setting("default_launch_args", args);
    }
    if let Some(ref ar) = settings.default_aspect_ratio {
        let _ = state.db.set_setting("default_aspect_ratio", ar);
    }
    if let Some(res) = settings.auto_resolve_conflicts {
        let _ = state.db.set_setting("auto_resolve_conflicts", if res { "true" } else { "false" });
    }
    if let Some(det) = settings.auto_detect_game_on_startup {
        let _ = state.db.set_setting("auto_detect_game_on_startup", if det { "true" } else { "false" });
    }
    Ok(())
}

#[tauri::command]
pub fn clear_download_cache(
    download_dir: Option<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<CacheCleanupResult, String> {
    let dl_path = download_dir
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| state.app_data_dir.join("downloads"));

    if !dl_path.is_dir() {
        return Ok(CacheCleanupResult { files_removed: 0, bytes_freed: 0 });
    }

    let mut files_removed = 0;
    let mut bytes_freed = 0;

    if let Ok(entries) = std::fs::read_dir(&dl_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if ext == "zip" || ext == "rar" || ext == "7z" || ext == "tmp" || ext == "crdownload" {
                    if let Ok(meta) = std::fs::metadata(&path) {
                        bytes_freed += meta.len();
                    }
                    if std::fs::remove_file(&path).is_ok() {
                        files_removed += 1;
                    }
                }
            }
        }
    }

    Ok(CacheCleanupResult { files_removed, bytes_freed })
}

#[tauri::command]
pub fn purge_stale_backups(
    game_id: Option<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<CacheCleanupResult, String> {
    let backup_dir = state.app_data_dir.join("backups");
    if !backup_dir.is_dir() {
        return Ok(CacheCleanupResult { files_removed: 0, bytes_freed: 0 });
    }

    let mut files_removed = 0;
    let mut bytes_freed = 0;

    let active_records = if let Some(ref gid) = game_id {
        state.db.get_backups(gid).unwrap_or_default()
    } else {
        let games = state.db.get_game_installations().unwrap_or_default();
        let mut all = Vec::new();
        for g in games {
            if let Ok(b) = state.db.get_backups(&g.id) {
                all.extend(b);
            }
        }
        all
    };

    let active_paths: std::collections::HashSet<String> = active_records
        .into_iter()
        .map(|r| r.backup_file_path.replace('\\', "/").to_lowercase())
        .collect();

    fn walk_backup_dir(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    walk_backup_dir(&p, out);
                } else if p.is_file() {
                    out.push(p);
                }
            }
        }
    }

    let mut all_files = Vec::new();
    walk_backup_dir(&backup_dir, &mut all_files);

    for path in all_files {
        let norm = path.to_string_lossy().replace('\\', "/").to_lowercase();
        if !active_paths.contains(&norm) {
            if let Ok(meta) = std::fs::metadata(&path) {
                bytes_freed += meta.len();
            }
            if std::fs::remove_file(&path).is_ok() {
                files_removed += 1;
            }
        }
    }

    Ok(CacheCleanupResult { files_removed, bytes_freed })
}


#[tauri::command]
pub fn sync_existing_mods(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<InstalledMod>, String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game installation '{}' not found", game_id))?;

    let profiles = state.db.get_profiles(&game_id).map_err(|e| e.to_string())?;
    let active_pid = profiles.iter().find(|p| p.is_active).map(|p| p.id.clone())
        .unwrap_or_else(|| profiles.get(0).map(|p| p.id.clone()).unwrap_or_else(|| "default_profile".to_string()));

    crate::existing_mods::sync_existing_mods_into_db(&game, &active_pid, &state.db)
}

#[tauri::command]
pub async fn pick_folder() -> Result<Option<String>, String> {
    tokio::task::spawn_blocking(|| {
        let script = r#"
            Add-Type -AssemblyName System.Windows.Forms
            $f = New-Object System.Windows.Forms.FolderBrowserDialog
            $f.Description = 'Select Grand Theft Auto: San Andreas Game Directory'
            $f.ShowNewFolderButton = $false
            if ($f.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
                Write-Output $f.SelectedPath
            }
        "#;
        let output = std::process::Command::new("powershell")
            .args(&["-STA", "-NoProfile", "-NonInteractive", "-Command", script])
            .output()
            .map_err(|e| e.to_string())?;

        let res = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn pick_archive_file() -> Result<Option<String>, String> {
    tokio::task::spawn_blocking(|| {
        let script = r#"
            Add-Type -AssemblyName System.Windows.Forms
            $f = New-Object System.Windows.Forms.OpenFileDialog
            $f.Title = 'Select GTA: San Andreas Mod Archive'
            $f.Filter = 'Mod Files (*.zip;*.rar;*.7z;*.asi;*.cs;*.cleo)|*.zip;*.rar;*.7z;*.asi;*.cs;*.cleo|Archives (*.zip;*.rar;*.7z)|*.zip;*.rar;*.7z|Plugins & Scripts (*.asi;*.cs;*.cleo)|*.asi;*.cs;*.cleo|All Files (*.*)|*.*'
            if ($f.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
                Write-Output $f.FileName
            }
        "#;
        let output = std::process::Command::new("powershell")
            .args(&["-STA", "-NoProfile", "-NonInteractive", "-Command", script])
            .output()
            .map_err(|e| e.to_string())?;

        let res = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadedArchiveItem {
    pub file_name: String,
    pub absolute_path: String,
    pub file_size_bytes: u64,
    pub modified_timestamp: u64,
}

#[tauri::command]
pub fn get_recent_downloaded_archives() -> Vec<DownloadedArchiveItem> {
    let dl_dir = match dirs::download_dir() {
        Some(d) => d,
        None => return Vec::new(),
    };

    let mut items = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dl_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                if ["zip", "rar", "7z", "asi", "cs", "cleo"].contains(&ext.as_str()) {
                    if let Ok(meta) = entry.metadata() {
                        let size = meta.len();
                        let modified = meta.modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs())
                            .unwrap_or(0);

                        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        items.push(DownloadedArchiveItem {
                            file_name,
                            absolute_path: path.to_string_lossy().to_string(),
                            file_size_bytes: size,
                            modified_timestamp: modified,
                        });
                    }
                }
            }
        }
    }

    items.sort_by(|a, b| b.modified_timestamp.cmp(&a.modified_timestamp));
    items.truncate(8);
    items
}

#[tauri::command]
pub fn open_path_in_explorer(target_path: String) -> Result<(), String> {
    let mut p = PathBuf::from(&target_path);
    if !p.exists() {
        let disabled_variant = PathBuf::from(format!("{}.disabled", target_path));
        if disabled_variant.exists() {
            p = disabled_variant;
        } else {
            return Err(format!("Path does not exist: {}", target_path));
        }
    }

    #[cfg(target_os = "windows")]
    {
        let final_path_str = p.to_string_lossy().to_string();
        if p.is_file() {
            let win_path = final_path_str.replace('/', "\\");
            std::process::Command::new("explorer")
                .arg(format!("/select,{}", win_path))
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            let win_path = final_path_str.replace('/', "\\");
            std::process::Command::new("explorer")
                .arg(win_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = target_path;
        Ok(())
    }
}

#[tauri::command]
pub async fn open_url_in_browser(url: String) -> Result<(), String> {
    let trimmed = url.trim();
    if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
        return Err("Only HTTP and HTTPS URLs can be opened".to_string());
    }

    let mut final_url = trimmed.to_string();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(1500))
        .build()
        .unwrap_or_default();

    if let Ok(resp) = client.head(trimmed).header("User-Agent", "Mozilla/5.0").send().await {
        if resp.status().as_u16() == 404 || resp.status().as_u16() == 410 {
            if trimmed.contains("mixmods.com.br") {
                let slug = trimmed.trim_end_matches('/').split('/').last().unwrap_or("").replace('-', "+").replace(".html", "");
                if !slug.is_empty() {
                    final_url = format!("https://www.mixmods.com.br/?s={}", slug);
                }
            } else if trimmed.contains("gtainside.com") {
                let slug = trimmed.trim_end_matches('/').split('/').last().unwrap_or("");
                let clean = regex::Regex::new(r"^\d+-").ok().map(|re| re.replace(slug, "").to_string()).unwrap_or_else(|| slug.to_string());
                final_url = format!("https://www.gtainside.com/en/sanandreas/search/{}/", clean.replace('-', "+"));
            } else if trimmed.contains("libertycity.net") {
                let slug = trimmed.trim_end_matches('/').split('/').last().unwrap_or("").replace(".html", "");
                let clean = regex::Regex::new(r"^\d+-").ok().map(|re| re.replace(&slug, "").to_string()).unwrap_or(slug);
                final_url = format!("https://libertycity.net/index.php?do=search&subaction=search&story={}", clean.replace('-', "+"));
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("rundll32")
            .args(&["url.dll,FileProtocolHandler", &final_url])
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&final_url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&final_url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn is_process_running_windows(target_name: &str) -> bool {
    use std::mem::size_of;

    #[repr(C)]
    struct ProcessEntry32W {
        dw_size: u32,
        cnt_usage: u32,
        th32_process_id: u32,
        th32_default_heap_id: usize,
        th32_module_id: u32,
        cnt_threads: u32,
        th32_parent_process_id: u32,
        pc_pri_class_base: i32,
        dw_flags: u32,
        sz_exe_file: [u16; 260],
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn CreateToolhelp32Snapshot(dw_flags: u32, th32_process_id: u32) -> *mut std::ffi::c_void;
        fn Process32FirstW(h_snapshot: *mut std::ffi::c_void, lppe: *mut ProcessEntry32W) -> i32;
        fn Process32NextW(h_snapshot: *mut std::ffi::c_void, lppe: *mut ProcessEntry32W) -> i32;
        fn CloseHandle(h_object: *mut std::ffi::c_void) -> i32;
    }

    const TH32CS_SNAPPROCESS: u32 = 0x00000002;
    let invalid_handle: *mut std::ffi::c_void = -1isize as *mut std::ffi::c_void;
    let target = target_name.to_lowercase();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot.is_null() || snapshot == invalid_handle {
            return false;
        }

        let mut entry = ProcessEntry32W {
            dw_size: size_of::<ProcessEntry32W>() as u32,
            cnt_usage: 0,
            th32_process_id: 0,
            th32_default_heap_id: 0,
            th32_module_id: 0,
            cnt_threads: 0,
            th32_parent_process_id: 0,
            pc_pri_class_base: 0,
            dw_flags: 0,
            sz_exe_file: [0u16; 260],
        };

        let mut found = false;
        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let len = entry.sz_exe_file.iter().position(|&c| c == 0).unwrap_or(260);
                let current_name = String::from_utf16_lossy(&entry.sz_exe_file[..len]).to_lowercase();
                if current_name == target {
                    found = true;
                    break;
                }
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
        found
    }
}

#[tauri::command]
pub fn is_game_process_running(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<bool, String> {
    let exe_name = if let Ok(games) = state.db.get_game_installations() {
        games.into_iter()
            .find(|g| g.id == game_id)
            .and_then(|g| {
                Path::new(&g.executable_path)
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| "gta_sa.exe".to_string())
    } else {
        "gta_sa.exe".to_string()
    };

    #[cfg(target_os = "windows")]
    {
        Ok(is_process_running_windows(&exe_name))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = exe_name;
        Ok(false)
    }
}

#[tauri::command]
pub fn kill_game_process(
    game_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game '{}' not found", game_id))?;

    let exe_name = Path::new(&game.executable_path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "gta_sa.exe".to_string());

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let _ = std::process::Command::new("taskkill")
            .args(&["/F", "/IM", &exe_name])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = exe_name;
        Ok(())
    }
}

#[tauri::command]
pub fn batch_toggle_mods(
    game_id: String,
    mod_ids: Vec<String>,
    enabled: bool,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game '{}' not found", game_id))?;
    let game_root = Path::new(&game.root_path);

    let mut mods = state.db.get_installed_mods(&game_id).map_err(|e| e.to_string())?;
    let id_set: std::collections::HashSet<String> = mod_ids.into_iter().collect();

    for m in mods.iter_mut() {
        if id_set.contains(&m.id) {
            m.enabled = enabled;
            crate::installation::apply_physical_mod_state(m, enabled, game_root);
            let _ = state.db.save_installed_mod(m);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn batch_uninstall_mods(
    game_id: String,
    mod_ids: Vec<String>,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let games = state.db.get_game_installations().map_err(|e| e.to_string())?;
    let game = games.into_iter().find(|g| g.id == game_id)
        .ok_or_else(|| format!("Game installation '{}' not found", game_id))?;

    for id in mod_ids {
        let _ = crate::installation::uninstall_mod(&id, &game, &state.db);
    }
    Ok(())
}
