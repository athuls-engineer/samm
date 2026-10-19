use std::path::{Path, PathBuf};
use std::fs;
use crate::models::*;
use crate::database::Database;
use crate::archives::{inspect_archive, extract_archive_safely, sanitize_archive_entry_path};

pub fn generate_installation_plan(
    archive_path: &Path,
    game: &GameInstallation,
    mod_id: &str,
    mod_title: &str,
) -> Result<InstallationPlan, String> {
    let inspection = inspect_archive(archive_path)?;
    let game_root = Path::new(&game.root_path);

    let has_cleo_framework = game_root.join("cleo").is_dir() || game_root.join("cleo.asi").is_file();
    let has_asi_loader = game_root.join("vorbisFile.dll").is_file()
        || game_root.join("vorbisHooked.dll").is_file()
        || game_root.join("d3d9.dll").is_file();
    let has_modloader = game_root.join("modloader").is_dir();

    let mut operations = Vec::new();
    let mut missing_prerequisites = Vec::new();
    let mut warnings = Vec::new();
    let mut manual_instructions = None;

    // Detect overall archive pattern
    let mut has_exe_or_installer = false;
    let mut has_cs_files = false;
    let mut has_asi_files = false;
    let mut has_dff_txd_assets = false;

    for entry in &inspection.entries {
        if entry.is_directory {
            continue;
        }
        let ext = Path::new(&entry.path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        if ["exe", "msi", "bat", "cmd"].contains(&ext.as_str()) {
            has_exe_or_installer = true;
        } else if ext == "cs" || ext == "fxt" || ext == "cleo" {
            has_cs_files = true;
        } else if ext == "asi" {
            has_asi_files = true;
        } else if ["dff", "txd", "col", "ifp"].contains(&ext.as_str()) {
            has_dff_txd_assets = true;
        }
    }

    // Determine strategy
    let (strategy, install_type, confidence) = if has_exe_or_installer {
        warnings.push("Archive contains executable installer or batch script. Automated execution is strictly prohibited for security.".to_string());
        manual_instructions = Some("Extract the archive manually in a sandbox, review the included files and readme, and run external installers with caution.".to_string());
        (InstallationStrategyType::Manual, "manual", "manual_required")
    } else if has_cs_files {
        if !has_cleo_framework {
            missing_prerequisites.push("CLEO 4 library is required to run .cs scripts".to_string());
        }
        (InstallationStrategyType::Cleo, "automatic", "high")
    } else if has_asi_files {
        if !has_asi_loader {
            missing_prerequisites.push("ASI Loader (vorbisFile.dll or SilentPatch) is required to load .asi plugins".to_string());
        }
        (InstallationStrategyType::Asi, "automatic", "high")
    } else if has_dff_txd_assets {
        if has_modloader {
            (InstallationStrategyType::Modloader, "automatic", "high")
        } else {
            warnings.push("ModLoader is not detected in game folder. Installing model/texture assets into ModLoader is strongly recommended to protect gta3.img.".to_string());
            (InstallationStrategyType::Assets, "guided", "medium")
        }
    } else {
        (InstallationStrategyType::Loose, "guided", "medium")
    };

    // Clean mod folder name for ModLoader isolation
    let safe_mod_folder_name: String = mod_title
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
        .collect::<String>()
        .trim()
        .replace(' ', "_");

    let clean_folder = if safe_mod_folder_name.is_empty() {
        "Custom_Mod".to_string()
    } else {
        safe_mod_folder_name
    };

    let mut new_count = 0;
    let mut overwrite_count = 0;

    let resolved_entries = resolve_archive_layout(&inspection.entries, mod_title, archive_path, &mut warnings);

    for resolved in &resolved_entries {
        let rel_path = &resolved.normalized_subpath;
        let ext = Path::new(rel_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let file_name = Path::new(rel_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file");

        let (dest_rel, rule) = match strategy {
            InstallationStrategyType::Manual => {
                (format!("manual_review/{}", file_name), "Manual inspection required".to_string())
            }
            InstallationStrategyType::Cleo => {
                if ext == "cs" || ext == "cleo" {
                    (format!("cleo/{}", file_name), "Place script in cleo/ directory".to_string())
                } else if ext == "fxt" {
                    (format!("cleo/cleo_text/{}", file_name), "Place text translation in cleo/cleo_text/".to_string())
                } else if ext == "ini" || ext == "cfg" {
                    (format!("cleo/{}", file_name), "Place script config in cleo/".to_string())
                } else {
                    (format!("cleo/{}", file_name), "Auxiliary CLEO file".to_string())
                }
            }
            InstallationStrategyType::Asi => {
                // If ModLoader is available and the mod contains companion subdirectories (e.g. data/, models/, VHud/), isolate inside ModLoader
                let has_companion_dirs = resolved_entries.iter().any(|r| r.normalized_subpath.contains('/') || r.normalized_subpath.contains('\\'));
                if has_modloader && has_companion_dirs {
                    let clean_rel = if rel_path.to_lowercase().starts_with("modloader/") {
                        &rel_path["modloader/".len()..]
                    } else if rel_path.to_lowercase().starts_with("modloader\\") {
                        &rel_path["modloader\\".len()..]
                    } else {
                        rel_path.as_str()
                    };
                    (format!("modloader/{}/{}", clean_folder, clean_rel), "Isolate plugin and companion assets inside ModLoader".to_string())
                } else if ext == "asi" || ext == "ini" || ext == "dll" || ext == "toml" || ext == "cfg" || ext == "json" || ext == "sp" || ext == "fp" {
                    (file_name.to_string(), "Place plugin in game root directory".to_string())
                } else {
                    // Keep companion data folders alongside the ASI in game root
                    (rel_path.clone(), "Place companion plugin asset relative to game root".to_string())
                }
            }
            InstallationStrategyType::Modloader | InstallationStrategyType::Assets => {
                let clean_rel = if rel_path.to_lowercase().starts_with("modloader/") {
                    &rel_path["modloader/".len()..]
                } else if rel_path.to_lowercase().starts_with("modloader\\") {
                    &rel_path["modloader\\".len()..]
                } else {
                    rel_path.as_str()
                };
                (format!("modloader/{}/{}", clean_folder, clean_rel), "Isolate package inside ModLoader folder".to_string())
            }
            _ => {
                if has_modloader {
                    let clean_rel = if rel_path.to_lowercase().starts_with("modloader/") {
                        &rel_path["modloader/".len()..]
                    } else if rel_path.to_lowercase().starts_with("modloader\\") {
                        &rel_path["modloader\\".len()..]
                    } else {
                        rel_path.as_str()
                    };
                    (format!("modloader/{}/{}", clean_folder, clean_rel), "Sandboxed ModLoader directory".to_string())
                } else {
                    (rel_path.clone(), "Direct placement relative to game root".to_string())
                }
            }
        };

        let dest_abs = game_root.join(&dest_rel);
        let exists = dest_abs.exists();

        let state = if exists {
            overwrite_count += 1;
            "overwrite"
        } else {
            new_count += 1;
            "new"
        };

        operations.push(PlannedFileOperation {
            archive_rel_path: resolved.archive_path.clone(),
            destination_rel_path: dest_rel,
            destination_absolute_path: dest_abs.to_string_lossy().to_string(),
            file_type: ext,
            rule,
            destination_state: state.to_string(),
            backup_required: exists,
            file_size_bytes: resolved.size,
        });
    }

    Ok(InstallationPlan {
        mod_id: mod_id.to_string(),
        mod_title: mod_title.to_string(),
        installation_type: install_type.to_string(),
        strategy,
        total_files: operations.len(),
        new_files_count: new_count,
        overwrite_files_count: overwrite_count,
        missing_prerequisites,
        warnings,
        file_operations: operations,
        confidence: confidence.to_string(),
        manual_instructions,
    })
}

pub fn execute_installation_transaction(
    archive_path: &Path,
    plan: &InstallationPlan,
    game: &GameInstallation,
    profile_id: &str,
    backup_storage_root: &Path,
    db: &Database,
) -> Result<InstalledMod, String> {
    if plan.installation_type == "manual" {
        return Err("Cannot execute automatic installation for a mod categorized as Manual. Review instructions and proceed manually.".to_string());
    }

    let game_root = Path::new(&game.root_path);
    if !game_root.is_dir() {
        return Err(format!("Game directory '{}' does not exist", game.root_path));
    }

    let operation_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Create temp staging directory
    let temp_staging = tempfile::tempdir().map_err(|e| format!("Failed to create temporary staging directory: {}", e))?;
    let staging_path = temp_staging.path();

    // Safely extract archive to staging
    let _ = extract_archive_safely(archive_path, staging_path)?;

    // Backup directory for this specific transaction
    let safe_ts = timestamp.replace(':', "-");
    let tx_backup_dir = backup_storage_root
        .join(&game.id)
        .join(&plan.mod_id)
        .join(&safe_ts);

    fs::create_dir_all(&tx_backup_dir).map_err(|e| format!("Failed to create backup directory: {}", e))?;

    // Record recovery journal entry
    let mut journal_entry = RecoveryJournalEntry {
        id: uuid::Uuid::new_v4().to_string(),
        operation_id: operation_id.clone(),
        operation_type: "install".to_string(),
        status: "in_progress".to_string(),
        target_mod_id: plan.mod_id.clone(),
        target_mod_name: plan.mod_title.clone(),
        timestamp: timestamp.clone(),
        staged_backup_paths: Vec::new(),
        affected_files: Vec::new(),
    };
    db.save_journal_entry(&journal_entry).map_err(|e| format!("Failed to initialize recovery journal: {}", e))?;

    let mut created_backups = Vec::new();
    let mut installed_files_records = Vec::new();

    // Step 1: Backup all files that will be overwritten
    for op in &plan.file_operations {
        let dest = Path::new(&op.destination_absolute_path);
        if dest.exists() && dest.is_file() {
            let fname = dest.file_name().unwrap_or_default().to_string_lossy();
            let backup_filename = format!("{}_{}", uuid::Uuid::new_v4().simple(), fname);
            let backup_file = tx_backup_dir.join(&backup_filename);

            fs::copy(dest, &backup_file).map_err(|e| {
                format!("Failed to create backup of existing file {:?}: {}", dest, e)
            })?;

            let (orig_hash, orig_size) = crate::game_detection::compute_file_sha256(dest)
                .unwrap_or_else(|_| (String::new(), 0));

            let backup_rec = BackupRecord {
                id: uuid::Uuid::new_v4().to_string(),
                game_installation_id: game.id.clone(),
                mod_id: plan.mod_id.clone(),
                mod_name: plan.mod_title.clone(),
                destination_rel_path: op.destination_rel_path.clone(),
                backup_file_path: backup_file.to_string_lossy().to_string(),
                original_sha256: orig_hash,
                original_size_bytes: orig_size,
                timestamp: timestamp.clone(),
                restored: false,
            };

            db.save_backup(&backup_rec).map_err(|e| format!("Failed to record backup: {}", e))?;
            journal_entry.staged_backup_paths.push(backup_file.to_string_lossy().to_string());
            journal_entry.affected_files.push(op.destination_rel_path.clone());
            created_backups.push(backup_rec);
        }
    }
    let _ = db.save_journal_entry(&journal_entry);

    // Step 2: Copy staged files to actual destinations
    for op in &plan.file_operations {
        let safe_archive_rel = match sanitize_archive_entry_path(&op.archive_rel_path) {
            Ok(p) => p,
            Err(e) => {
                // Rollback!
                rollback_transaction(&created_backups, &installed_files_records, game_root);
                journal_entry.status = "rolled_back".to_string();
                let _ = db.save_journal_entry(&journal_entry);
                return Err(format!("Unsafe archive path detected: {}", e));
            }
        };

        let staged_source = staging_path.join(safe_archive_rel);
        if !staged_source.exists() {
            continue; // Could be directory or omitted
        }

        let target_dest = Path::new(&op.destination_absolute_path);
        if let Some(parent) = target_dest.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    rollback_transaction(&created_backups, &installed_files_records, game_root);
                    journal_entry.status = "rolled_back".to_string();
                    let _ = db.save_journal_entry(&journal_entry);
                    return Err(format!("Failed to create directory {:?}: {}", parent, e));
                }
            }
        }

        if let Err(e) = fs::copy(&staged_source, target_dest) {
            rollback_transaction(&created_backups, &installed_files_records, game_root);
            journal_entry.status = "rolled_back".to_string();
            let _ = db.save_journal_entry(&journal_entry);
            return Err(format!("Failed to install file {:?}: {}", target_dest, e));
        }

        // Verify destination file
        let (hash, size) = match crate::game_detection::compute_file_sha256(target_dest) {
            Ok(h) => h,
            Err(e) => {
                rollback_transaction(&created_backups, &installed_files_records, game_root);
                journal_entry.status = "rolled_back".to_string();
                let _ = db.save_journal_entry(&journal_entry);
                return Err(format!("Post-installation hash verification failed for {:?}: {}", target_dest, e));
            }
        };

        let has_backup = created_backups.iter().any(|b| b.destination_rel_path == op.destination_rel_path);
        let backup_path = created_backups
            .iter()
            .find(|b| b.destination_rel_path == op.destination_rel_path)
            .map(|b| b.backup_file_path.clone());

        installed_files_records.push(InstalledFileRecord {
            id: uuid::Uuid::new_v4().to_string(),
            installed_mod_id: plan.mod_id.clone(),
            destination_rel_path: op.destination_rel_path.clone(),
            sha256: hash,
            file_size_bytes: size,
            has_backup,
            backup_path,
        });
    }

    // Step 3: Record installed mod in database
    let installed_mod = InstalledMod {
        id: plan.mod_id.clone(),
        game_installation_id: game.id.clone(),
        profile_id: profile_id.to_string(),
        name: plan.mod_title.clone(),
        author: "Unknown Author".to_string(),
        version: "1.0.0".to_string(),
        source_name: "Local Archive".to_string(),
        source_url: None,
        installed_at: timestamp,
        strategy: format!("{:?}", plan.strategy),
        enabled: true,
        files: installed_files_records,
        has_conflicts: false,
        conflict_notes: Vec::new(),
    };

    if let Err(e) = db.save_installed_mod(&installed_mod) {
        rollback_transaction(&created_backups, &installed_mod.files, game_root);
        journal_entry.status = "rolled_back".to_string();
        let _ = db.save_journal_entry(&journal_entry);
        return Err(format!("Failed to record installed mod in database: {}", e));
    }

    // Update recovery journal to committed
    journal_entry.status = "committed".to_string();
    let _ = db.save_journal_entry(&journal_entry);

    // Auto-resolve any file collisions, root duplicates, or conflicting plugins to guarantee total compatibility
    let _ = crate::conflicts::auto_resolve_all_conflicts_physical(game, db);

    Ok(installed_mod)
}

fn rollback_transaction(
    backups: &[BackupRecord],
    installed_files: &[InstalledFileRecord],
    game_root: &Path,
) {
    // 1. Remove files that were newly written
    for f in installed_files {
        let dest = game_root.join(&f.destination_rel_path);
        if !f.has_backup && dest.exists() {
            let _ = fs::remove_file(&dest);
        }
    }

    // 2. Restore all files that had backups
    for b in backups {
        let dest = game_root.join(&b.destination_rel_path);
        let src = Path::new(&b.backup_file_path);
        if src.exists() {
            let _ = fs::copy(src, &dest);
        }
    }
}

pub fn get_mod_dedicated_directories(m: &InstalledMod, game_root: &Path) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let vanilla_protected = [
        "anim", "audio", "data", "models", "movies", "text", "samp", "atp", "scripts", "cleo", "modloader",
    ];

    // Explicit dedicated directory check for Rainbomizer / Randomizer
    let mod_name_lower = m.name.to_lowercase();
    if m.id == "github_rainbomizer"
        || mod_name_lower.contains("rainbomizer")
        || mod_name_lower.contains("randomizer")
        || mod_name_lower.contains("randomiser")
    {
        let r_dir = game_root.join("rainbomizer");
        if !dirs.contains(&r_dir) {
            dirs.push(r_dir);
        }
    }

    for f in &m.files {
        let rel = f.destination_rel_path.replace('\\', "/");
        let parts: Vec<&str> = rel.split('/').filter(|p| !p.is_empty()).collect();
        if parts.is_empty() {
            continue;
        }

        let first = parts[0];
        let first_clean = first.trim_end_matches(".disabled");

        // ModLoader folder: modloader/<folder>/... or modloader/_ESSENTIALS/<subfolder>/...
        if first_clean.eq_ignore_ascii_case("modloader") && parts.len() >= 2 {
            let mod_folder = parts[1].trim_end_matches(".disabled");
            if (mod_folder.starts_with('_') || mod_folder.eq_ignore_ascii_case("essentials")) && parts.len() >= 3 {
                let sub_folder = parts[2].trim_end_matches(".disabled");
                let d = game_root.join("modloader").join(mod_folder).join(sub_folder);
                if !dirs.contains(&d) {
                    dirs.push(d);
                }
            } else {
                let d = game_root.join("modloader").join(mod_folder);
                if !dirs.contains(&d) {
                    dirs.push(d);
                }
            }
        }
        // Subdirectory in scripts or cleo: scripts/<folder>/... or cleo/<folder>/...
        else if (first_clean.eq_ignore_ascii_case("scripts") || first_clean.eq_ignore_ascii_case("cleo")) && parts.len() >= 3 {
            let sub_folder = parts[1].trim_end_matches(".disabled");
            let d = game_root.join(first_clean).join(sub_folder);
            if !dirs.contains(&d) {
                dirs.push(d);
            }
        }
        // Top-level custom mod folder: <custom_folder>/...
        else if !vanilla_protected.iter().any(|v| v.eq_ignore_ascii_case(first_clean)) {
            if parts.len() >= 2 {
                let d = game_root.join(first_clean);
                if !dirs.contains(&d) {
                    dirs.push(d);
                }
            }
        }
    }

    dirs
}

pub fn restore_disabled_files_recursive(dir: &Path) {
    if !dir.is_dir() {
        return;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                restore_disabled_files_recursive(&path);
            } else if path.is_file() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                if file_name.ends_with(".disabled") {
                    let new_name = file_name.trim_end_matches(".disabled");
                    let restored_path = path.with_file_name(new_name);
                    let _ = fs::rename(&path, &restored_path);
                }
            }
        }
    }
}

pub fn cleanup_empty_parents(dir: &Path, game_root: &Path) {
    let vanilla_protected = [
        "anim", "audio", "data", "models", "movies", "text", "samp", "atp", "scripts", "cleo", "modloader",
    ];

    let mut current = dir.to_path_buf();
    while current != game_root && current.starts_with(game_root) {
        if let Some(name) = current.file_name() {
            let name_str = name.to_string_lossy();
            if vanilla_protected.iter().any(|p| p.eq_ignore_ascii_case(&name_str)) {
                break;
            }
        } else {
            break;
        }

        // Try removing the dir; if it fails (not empty or in use), stop climbing
        if fs::remove_dir(&current).is_err() {
            break;
        }

        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }
}

pub fn uninstall_mod(
    mod_id: &str,
    game: &GameInstallation,
    db: &Database,
) -> Result<(), String> {
    let installed_mods = db.get_installed_mods(&game.id).map_err(|e| e.to_string())?;
    let target_mod = match installed_mods.into_iter().find(|m| m.id == mod_id) {
        Some(m) => m,
        None => return Err(format!("Mod with ID '{}' not found", mod_id)),
    };

    let game_root = Path::new(&game.root_path);
    let backups = db.get_backups(&game.id).map_err(|e| e.to_string())?;

    // 1. Identify and purge all dedicated directories (both active and .disabled) recursively
    let dedicated_dirs = get_mod_dedicated_directories(&target_mod, game_root);
    for dir in &dedicated_dirs {
        let dir_str = dir.to_string_lossy().to_string();
        let clean_str = dir_str.trim_end_matches(".disabled");
        let active_dir = PathBuf::from(clean_str);
        let disabled_dir = PathBuf::from(format!("{}.disabled", clean_str));

        if active_dir.is_dir() {
            let _ = fs::remove_dir_all(&active_dir);
        }
        if disabled_dir.is_dir() {
            let _ = fs::remove_dir_all(&disabled_dir);
        }
    }

    // 2. Remove all individual files tracked for this mod (and restore any backups)
    for file_rec in &target_mod.files {
        let rel = file_rec.destination_rel_path.replace('\\', "/");
        let rel_clean = rel.trim_end_matches(".disabled");
        let dest_abs = game_root.join(rel_clean);
        let disabled_abs = game_root.join(format!("{}.disabled", rel_clean));

        if file_rec.has_backup {
            let file_rel_norm = file_rec.destination_rel_path.replace('\\', "/");
            // Find most recent backup for this file and restore original
            if let Some(backup) = backups.iter().find(|b| b.destination_rel_path.replace('\\', "/") == file_rel_norm && !b.restored) {
                let backup_file = Path::new(&backup.backup_file_path);
                if backup_file.exists() {
                    let _ = fs::copy(backup_file, &dest_abs);
                    let mut updated_backup = backup.clone();
                    updated_backup.restored = true;
                    let _ = db.save_backup(&updated_backup);
                    if disabled_abs.is_file() {
                        let _ = fs::remove_file(&disabled_abs);
                    }
                    continue;
                }
            }
        }

        // Delete disabled variant if exists
        if disabled_abs.is_file() {
            let _ = fs::remove_file(&disabled_abs);
        }

        // Delete active variant if exists
        if dest_abs.is_file() {
            let _ = fs::remove_file(&dest_abs);
        }

        // Clean up companion .ini files for ASI or CLEO plugins
        if rel_clean.ends_with(".asi") || rel_clean.ends_with(".cs") {
            let stem = if rel_clean.ends_with(".asi") {
                rel_clean.trim_end_matches(".asi")
            } else {
                rel_clean.trim_end_matches(".cs")
            };
            let ini_active = game_root.join(format!("{}.ini", stem));
            let ini_disabled = game_root.join(format!("{}.ini.disabled", stem));
            if ini_active.is_file() {
                let _ = fs::remove_file(&ini_active);
            }
            if ini_disabled.is_file() {
                let _ = fs::remove_file(&ini_disabled);
            }
        }

        // Clean up empty parent directory if safe
        if let Some(parent) = dest_abs.parent() {
            cleanup_empty_parents(parent, game_root);
        }
    }

    // 3. Mod-specific post-removal cleanup (e.g. standalone readme in game root)
    let mod_name_lower = target_mod.name.to_lowercase();
    if target_mod.id == "github_rainbomizer" || mod_name_lower.contains("rainbomizer") || mod_name_lower.contains("randomizer") {
        let r_readme = game_root.join("readme.txt");
        if r_readme.is_file() {
            if let Ok(content) = fs::read_to_string(&r_readme) {
                if content.contains("Rainbomizer") {
                    let _ = fs::remove_file(&r_readme);
                }
            }
        }
    }

    db.delete_installed_mod(mod_id).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn apply_physical_mod_state(m: &mut InstalledMod, enabled: bool, game_root: &Path) {
    let dedicated_dirs = get_mod_dedicated_directories(m, game_root);

    // 1. Toggle dedicated directories
    for dir in &dedicated_dirs {
        let dir_str = dir.to_string_lossy().to_string();
        let clean_str = dir_str.trim_end_matches(".disabled");
        let active_dir = PathBuf::from(clean_str);
        let disabled_dir = PathBuf::from(format!("{}.disabled", clean_str));

        if !enabled {
            if active_dir.is_dir() {
                // Restore any individually disabled files inside first so files remain clean
                restore_disabled_files_recursive(&active_dir);
                let _ = fs::rename(&active_dir, &disabled_dir);
            }
        } else {
            if disabled_dir.is_dir() {
                let _ = fs::rename(&disabled_dir, &active_dir);
            }
            if active_dir.is_dir() {
                // Ensure all files inside active directory have active names
                restore_disabled_files_recursive(&active_dir);
            }
        }
    }

    // 2. Individual file toggling (ASI plugins, CLEO scripts, etc. outside dedicated directories)
    for f in &m.files {
        let rel = f.destination_rel_path.replace('\\', "/");
        let rel_clean = rel.trim_end_matches(".disabled").to_string();
        let active_path = game_root.join(&rel_clean);
        let disabled_path = game_root.join(format!("{}.disabled", rel_clean));

        // Skip if this file is located inside one of the dedicated directories already handled
        let inside_dedicated = dedicated_dirs.iter().any(|d| {
            let d_clean = PathBuf::from(d.to_string_lossy().trim_end_matches(".disabled"));
            active_path.starts_with(&d_clean)
        });
        if inside_dedicated {
            continue;
        }

        if !enabled {
            if active_path.is_file() {
                let _ = fs::rename(&active_path, &disabled_path);
            }
            // Companion .ini toggling for ASI or CLEO
            if rel_clean.ends_with(".asi") || rel_clean.ends_with(".cs") {
                let stem = if rel_clean.ends_with(".asi") {
                    rel_clean.trim_end_matches(".asi")
                } else {
                    rel_clean.trim_end_matches(".cs")
                };
                let ini_active = game_root.join(format!("{}.ini", stem));
                let ini_disabled = game_root.join(format!("{}.ini.disabled", stem));
                if ini_active.is_file() {
                    let _ = fs::rename(&ini_active, &ini_disabled);
                }
            }
        } else {
            if disabled_path.is_file() {
                let _ = fs::rename(&disabled_path, &active_path);
            }
            // Companion .ini restoration
            if rel_clean.ends_with(".asi") || rel_clean.ends_with(".cs") {
                let stem = if rel_clean.ends_with(".asi") {
                    rel_clean.trim_end_matches(".asi")
                } else {
                    rel_clean.trim_end_matches(".cs")
                };
                let ini_active = game_root.join(format!("{}.ini", stem));
                let ini_disabled = game_root.join(format!("{}.ini.disabled", stem));
                if ini_disabled.is_file() {
                    let _ = fs::rename(&ini_disabled, &ini_active);
                }
            }
        }
    }
}

pub struct ResolvedArchiveEntry {
    pub archive_path: String,
    pub normalized_subpath: String,
    pub size: u64,
}

pub fn resolve_archive_layout(
    entries: &[ArchiveEntry],
    mod_title: &str,
    archive_path: &Path,
    warnings: &mut Vec<String>,
) -> Vec<ResolvedArchiveEntry> {
    let vanilla_dirs = [
        "anim", "audio", "data", "models", "movies", "text", "samp", "atp", "scripts", "cleo", "modloader",
    ];

    let file_entries: Vec<&ArchiveEntry> = entries.iter().filter(|e| !e.is_directory).collect();
    if file_entries.is_empty() {
        return Vec::new();
    }

    let mut top_level_map: std::collections::HashMap<String, Vec<&ArchiveEntry>> = std::collections::HashMap::new();
    let mut root_files: Vec<&ArchiveEntry> = Vec::new();

    for entry in &file_entries {
        let norm = entry.path.replace('\\', "/");
        let parts: Vec<&str> = norm.split('/').filter(|p| !p.is_empty()).collect();
        if parts.len() > 1 {
            let top = parts[0].to_string();
            top_level_map.entry(top).or_default().push(entry);
        } else {
            root_files.push(entry);
        }
    }

    let non_vanilla_tops: Vec<String> = top_level_map
        .keys()
        .filter(|k| !vanilla_dirs.iter().any(|v| v.eq_ignore_ascii_case(k)))
        .cloned()
        .collect();

    let mut chosen_variant: Option<String> = None;
    if non_vanilla_tops.len() >= 2 {
        let mut variant_scores: Vec<(String, i32)> = Vec::new();
        let title_lower = mod_title.to_lowercase();
        let archive_stem = archive_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        for top in &non_vanilla_tops {
            let top_lower = top.to_lowercase();
            let mut score = 0;

            if title_lower.contains("4k") && top_lower.contains("4k") { score += 50; }
            if title_lower.contains("2k") && top_lower.contains("2k") { score += 50; }
            if archive_stem.contains("4k") && top_lower.contains("4k") { score += 30; }
            if archive_stem.contains("2k") && top_lower.contains("2k") { score += 30; }
            if title_lower.contains("definitive") && top_lower.contains("definitive") { score += 20; }
            if title_lower.contains("remaster") && top_lower.contains("remaster") { score += 20; }

            if top_lower.contains("4k") || top_lower.contains("uhd") { score += 15; }
            if top_lower.contains("2k") || top_lower.contains("1080p") || top_lower.contains("fhd") { score += 10; }
            if top_lower.contains("hq") || top_lower.contains("high") { score += 8; }
            if top_lower.contains("recommended") || top_lower.contains("default") { score += 25; }

            variant_scores.push((top.clone(), score));
        }

        variant_scores.sort_by(|a, b| b.1.cmp(&a.1));
        if let Some((best, _)) = variant_scores.first() {
            chosen_variant = Some(best.clone());
            let other_variants: Vec<String> = non_vanilla_tops
                .iter()
                .filter(|t| *t != best)
                .cloned()
                .collect();
            warnings.push(format!(
                "Archive contains multiple variants ({:?}). Automatically selected '{}' for optimal game quality and compatibility.",
                other_variants, best
            ));
        }
    }

    let mut result = Vec::new();

    // Case 1: Variant chosen
    if let Some(ref variant) = chosen_variant {
        if let Some(variant_entries) = top_level_map.get(variant) {
            let prefix = format!("{}/", variant);
            let prefix_win = format!("{}\\", variant);
            for entry in variant_entries {
                let norm = entry.path.replace('\\', "/");
                let stripped = if norm.starts_with(&prefix) {
                    norm[prefix.len()..].to_string()
                } else if norm.starts_with(&prefix_win) {
                    norm[prefix_win.len()..].to_string()
                } else {
                    norm
                };

                result.push(ResolvedArchiveEntry {
                    archive_path: entry.path.clone(),
                    normalized_subpath: stripped,
                    size: entry.size,
                });
            }
        }

        for entry in root_files {
            result.push(ResolvedArchiveEntry {
                archive_path: entry.path.clone(),
                normalized_subpath: entry.path.replace('\\', "/"),
                size: entry.size,
            });
        }

        return result;
    }

    // Case 2: Single non-vanilla top-level wrapper folder
    if non_vanilla_tops.len() == 1 && root_files.iter().all(|r| {
        let ext = Path::new(&r.path).extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        ["txt", "md", "url", "pdf", "html"].contains(&ext.as_str())
    }) {
        let single_top = &non_vanilla_tops[0];
        let prefix = format!("{}/", single_top);
        let prefix_win = format!("{}\\", single_top);

        for entry in &file_entries {
            let norm = entry.path.replace('\\', "/");
            let stripped = if norm.starts_with(&prefix) {
                norm[prefix.len()..].to_string()
            } else if norm.starts_with(&prefix_win) {
                norm[prefix_win.len()..].to_string()
            } else {
                norm
            };

            result.push(ResolvedArchiveEntry {
                archive_path: entry.path.clone(),
                normalized_subpath: stripped,
                size: entry.size,
            });
        }
        return result;
    }

    // Case 3: Standard layout
    for entry in file_entries {
        result.push(ResolvedArchiveEntry {
            archive_path: entry.path.clone(),
            normalized_subpath: entry.path.replace('\\', "/"),
            size: entry.size,
        });
    }

    result
}

