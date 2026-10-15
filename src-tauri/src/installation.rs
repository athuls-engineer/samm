use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use sha2::{Digest, Sha256};
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
        } else if ext == "cs" || ext == "fxt" {
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

    for entry in &inspection.entries {
        if entry.is_directory {
            continue;
        }

        let rel_path = entry.path.replace('\\', "/");
        let ext = Path::new(&rel_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Skip readme/url/txt files from dirtying game root if not needed, or place in modloader
        let file_name = Path::new(&rel_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file");

        let (dest_rel, rule) = match strategy {
            InstallationStrategyType::Manual => {
                (format!("manual_review/{}", file_name), "Manual inspection required".to_string())
            }
            InstallationStrategyType::Cleo => {
                if ext == "cs" {
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
                if ext == "asi" || ext == "ini" || ext == "dll" {
                    (file_name.to_string(), "Place plugin in game root directory".to_string())
                } else {
                    (format!("modloader/{}/{}", clean_folder, file_name), "Place asset in ModLoader sandbox".to_string())
                }
            }
            InstallationStrategyType::Modloader | InstallationStrategyType::Assets => {
                (format!("modloader/{}/{}", clean_folder, rel_path), "Isolate package inside ModLoader folder".to_string())
            }
            _ => {
                if has_modloader {
                    (format!("modloader/{}/{}", clean_folder, rel_path), "Sandboxed ModLoader directory".to_string())
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
            archive_rel_path: entry.path.clone(),
            destination_rel_path: dest_rel,
            destination_absolute_path: dest_abs.to_string_lossy().to_string(),
            file_type: ext,
            rule,
            destination_state: state.to_string(),
            backup_required: exists,
            file_size_bytes: entry.size,
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
            let backup_filename = format!("{}_{}", uuid::Uuid::new_v4().simple(), dest.file_name().unwrap().to_string_lossy());
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

    Ok(installed_mod)
}

fn rollback_transaction(
    backups: &[BackupRecord],
    installed_files: &[InstalledFileRecord],
    _game_root: &Path,
) {
    // 1. Remove files that were newly written
    for f in installed_files {
        let dest = Path::new(&f.destination_rel_path);
        if !f.has_backup && dest.exists() {
            let _ = fs::remove_file(dest);
        }
    }

    // 2. Restore all files that had backups
    for b in backups {
        let dest = Path::new(&b.destination_rel_path);
        let src = Path::new(&b.backup_file_path);
        if src.exists() {
            let _ = fs::copy(src, dest);
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

    for file_rec in &target_mod.files {
        let dest_abs = game_root.join(&file_rec.destination_rel_path);

        if file_rec.has_backup {
            // Find most recent backup for this file
            if let Some(backup) = backups.iter().find(|b| b.destination_rel_path == file_rec.destination_rel_path && !b.restored) {
                let backup_file = Path::new(&backup.backup_file_path);
                if backup_file.exists() {
                    let _ = fs::copy(backup_file, &dest_abs);
                    let mut updated_backup = backup.clone();
                    updated_backup.restored = true;
                    let _ = db.save_backup(&updated_backup);
                    continue;
                }
            }
        }

        // If no backup was required (it was a brand new file placed by mod), delete it safely
        if dest_abs.exists() && dest_abs.is_file() {
            let _ = fs::remove_file(&dest_abs);

            // If empty parent dir (e.g. inside modloader/<mod_name>/), clean up
            if let Some(parent) = dest_abs.parent() {
                let _ = fs::remove_dir(parent); // Only succeeds if directory is empty
            }
        }
    }

    db.delete_installed_mod(mod_id).map_err(|e| e.to_string())?;
    Ok(())
}
