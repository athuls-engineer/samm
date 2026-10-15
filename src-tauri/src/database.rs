use rusqlite::{params, Connection, Result};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::models::*;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            conn: Arc::clone(&self.conn),
        }
    }
}

impl Database {
    pub fn init<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let parent = db_path.as_ref().parent();
        if let Some(p) = parent {
            let _ = std::fs::create_dir_all(p);
        }

        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA synchronous = NORMAL;"
        )?;

        let db = Database {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.run_migrations()?;
        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA synchronous = NORMAL;"
        )?;

        let db = Database {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_versions (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS game_installations (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                executable_path TEXT NOT NULL,
                root_path TEXT NOT NULL,
                detected_edition TEXT NOT NULL,
                version_confidence TEXT NOT NULL,
                executable_sha256 TEXT NOT NULL,
                executable_size_bytes INTEGER NOT NULL,
                pe_timestamp INTEGER,
                evidence_json TEXT NOT NULL,
                uncertainties_json TEXT NOT NULL,
                is_classic_1_0 INTEGER NOT NULL,
                is_active INTEGER NOT NULL,
                created_date TEXT NOT NULL,
                last_verified_date TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS mod_sources (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                base_url TEXT NOT NULL,
                capabilities_json TEXT NOT NULL,
                status TEXT NOT NULL,
                attribution TEXT NOT NULL,
                terms_notice TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS mod_records (
                id TEXT PRIMARY KEY,
                source_id TEXT NOT NULL,
                source_mod_id TEXT NOT NULL,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                description TEXT NOT NULL,
                category TEXT NOT NULL,
                tags_json TEXT NOT NULL,
                thumbnail_url TEXT,
                page_url TEXT NOT NULL,
                download_url TEXT,
                file_size_formatted TEXT,
                file_size_bytes INTEGER,
                release_date TEXT,
                update_date TEXT,
                supported_game_versions_json TEXT NOT NULL,
                required_dependencies_json TEXT NOT NULL,
                incompatibilities_json TEXT NOT NULL,
                installation_strategy TEXT NOT NULL,
                confidence TEXT NOT NULL,
                compatibility_reason TEXT NOT NULL,
                cached_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS download_jobs (
                id TEXT PRIMARY KEY,
                mod_id TEXT NOT NULL,
                mod_title TEXT NOT NULL,
                source_id TEXT NOT NULL,
                source_name TEXT NOT NULL,
                url TEXT NOT NULL,
                temp_file_path TEXT NOT NULL,
                total_bytes INTEGER NOT NULL,
                downloaded_bytes INTEGER NOT NULL,
                speed_bytes_per_sec INTEGER NOT NULL,
                status TEXT NOT NULL,
                error_message TEXT,
                expected_sha256 TEXT,
                computed_sha256 TEXT,
                sha256_matched INTEGER,
                start_time TEXT NOT NULL,
                completed_time TEXT
            );

            CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                game_installation_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                is_default INTEGER NOT NULL,
                is_active INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS profile_mods (
                profile_id TEXT NOT NULL,
                mod_id TEXT NOT NULL,
                PRIMARY KEY (profile_id, mod_id)
            );

            CREATE TABLE IF NOT EXISTS installed_mods (
                id TEXT PRIMARY KEY,
                game_installation_id TEXT NOT NULL,
                profile_id TEXT NOT NULL,
                name TEXT NOT NULL,
                author TEXT NOT NULL,
                version TEXT NOT NULL,
                source_name TEXT NOT NULL,
                source_url TEXT,
                installed_at TEXT NOT NULL,
                strategy TEXT NOT NULL,
                enabled INTEGER NOT NULL,
                has_conflicts INTEGER NOT NULL,
                conflict_notes_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS installed_files (
                id TEXT PRIMARY KEY,
                installed_mod_id TEXT NOT NULL,
                destination_rel_path TEXT NOT NULL,
                sha256 TEXT NOT NULL,
                file_size_bytes INTEGER NOT NULL,
                has_backup INTEGER NOT NULL,
                backup_path TEXT
            );

            CREATE TABLE IF NOT EXISTS conflicts (
                id TEXT PRIMARY KEY,
                destination_rel_path TEXT NOT NULL,
                active_mod_id TEXT NOT NULL,
                active_mod_name TEXT NOT NULL,
                conflicting_mod_id TEXT NOT NULL,
                conflicting_mod_name TEXT NOT NULL,
                status TEXT NOT NULL,
                resolution_notes TEXT
            );

            CREATE TABLE IF NOT EXISTS backups (
                id TEXT PRIMARY KEY,
                game_installation_id TEXT NOT NULL,
                mod_id TEXT NOT NULL,
                mod_name TEXT NOT NULL,
                destination_rel_path TEXT NOT NULL,
                backup_file_path TEXT NOT NULL,
                original_sha256 TEXT NOT NULL,
                original_size_bytes INTEGER NOT NULL,
                timestamp TEXT NOT NULL,
                restored INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS recovery_journal (
                id TEXT PRIMARY KEY,
                operation_id TEXT NOT NULL,
                operation_type TEXT NOT NULL,
                status TEXT NOT NULL,
                target_mod_id TEXT NOT NULL,
                target_mod_name TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                staged_backups_json TEXT NOT NULL,
                affected_files_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS app_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );"
        )?;

        Ok(())
    }

    // --- Game Installations ---
    pub fn save_game_installation(&self, game: &GameInstallation) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let evidence_json = serde_json::to_string(&game.evidence_list).unwrap_or_default();
        let uncertainties_json = serde_json::to_string(&game.uncertainties).unwrap_or_default();
        let conf_str = match game.version_confidence {
            ConfidenceLevel::Confirmed => "confirmed",
            ConfidenceLevel::Likely => "likely",
            ConfidenceLevel::Ambiguous => "ambiguous",
            ConfidenceLevel::Incompatible => "incompatible",
            ConfidenceLevel::Invalid => "invalid",
        };

        // If this installation is marked active, unset any previous active
        if game.is_active {
            let _ = conn.execute("UPDATE game_installations SET is_active = 0", []);
        }

        conn.execute(
            "INSERT INTO game_installations (
                id, name, executable_path, root_path, detected_edition,
                version_confidence, executable_sha256, executable_size_bytes,
                pe_timestamp, evidence_json, uncertainties_json, is_classic_1_0,
                is_active, created_date, last_verified_date
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                executable_path = excluded.executable_path,
                root_path = excluded.root_path,
                detected_edition = excluded.detected_edition,
                version_confidence = excluded.version_confidence,
                executable_sha256 = excluded.executable_sha256,
                executable_size_bytes = excluded.executable_size_bytes,
                pe_timestamp = excluded.pe_timestamp,
                evidence_json = excluded.evidence_json,
                uncertainties_json = excluded.uncertainties_json,
                is_classic_1_0 = excluded.is_classic_1_0,
                is_active = excluded.is_active,
                last_verified_date = excluded.last_verified_date",
            params![
                game.id,
                game.name,
                game.executable_path,
                game.root_path,
                game.detected_edition,
                conf_str,
                game.executable_sha256,
                game.executable_size_bytes,
                game.pe_timestamp,
                evidence_json,
                uncertainties_json,
                if game.is_classic_1_0 { 1 } else { 0 },
                if game.is_active { 1 } else { 0 },
                game.created_date,
                game.last_verified_date,
            ],
        )?;

        Ok(())
    }

    pub fn get_game_installations(&self) -> Result<Vec<GameInstallation>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, executable_path, root_path, detected_edition,
                    version_confidence, executable_sha256, executable_size_bytes,
                    pe_timestamp, evidence_json, uncertainties_json, is_classic_1_0,
                    is_active, created_date, last_verified_date
             FROM game_installations
             ORDER BY is_active DESC, name ASC"
        )?;

        let rows = stmt.query_map([], |row| {
            let conf_str: String = row.get(5)?;
            let confidence = match conf_str.as_str() {
                "confirmed" => ConfidenceLevel::Confirmed,
                "likely" => ConfidenceLevel::Likely,
                "ambiguous" => ConfidenceLevel::Ambiguous,
                "incompatible" => ConfidenceLevel::Incompatible,
                _ => ConfidenceLevel::Invalid,
            };

            let evidence_json: String = row.get(9)?;
            let evidence_list: Vec<String> = serde_json::from_str(&evidence_json).unwrap_or_default();
            let uncertainties_json: String = row.get(10)?;
            let uncertainties: Vec<String> = serde_json::from_str(&uncertainties_json).unwrap_or_default();
            let is_classic_int: i32 = row.get(11)?;
            let is_active_int: i32 = row.get(12)?;

            Ok(GameInstallation {
                id: row.get(0)?,
                name: row.get(1)?,
                executable_path: row.get(2)?,
                root_path: row.get(3)?,
                detected_edition: row.get(4)?,
                version_confidence: confidence,
                executable_sha256: row.get(6)?,
                executable_size_bytes: row.get(7)?,
                pe_timestamp: row.get(8)?,
                evidence_list,
                uncertainties,
                is_classic_1_0: is_classic_int == 1,
                is_active: is_active_int == 1,
                created_date: row.get(13)?,
                last_verified_date: row.get(14)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn set_active_installation(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE game_installations SET is_active = 0", [])?;
        conn.execute("UPDATE game_installations SET is_active = 1 WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn delete_game_installation(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM game_installations WHERE id = ?1", params![id])?;
        Ok(())
    }

    // --- Profiles ---
    pub fn save_profile(&self, profile: &Profile) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO profiles (id, game_installation_id, name, description, is_default, is_active, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(id) DO UPDATE SET
                 name = excluded.name,
                 description = excluded.description,
                 is_default = excluded.is_default,
                 is_active = excluded.is_active,
                 updated_at = excluded.updated_at",
            params![
                profile.id,
                profile.game_installation_id,
                profile.name,
                profile.description,
                if profile.is_default { 1 } else { 0 },
                if profile.is_active { 1 } else { 0 },
                profile.created_at,
                profile.updated_at,
            ],
        )?;

        // Update profile_mods
        conn.execute("DELETE FROM profile_mods WHERE profile_id = ?1", params![profile.id])?;
        for mod_id in &profile.installed_mod_ids {
            conn.execute(
                "INSERT OR IGNORE INTO profile_mods (profile_id, mod_id) VALUES (?1, ?2)",
                params![profile.id, mod_id],
            )?;
        }

        Ok(())
    }

    pub fn get_profiles(&self, game_id: &str) -> Result<Vec<Profile>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, game_installation_id, name, description, is_default, is_active, created_at, updated_at
             FROM profiles
             WHERE game_installation_id = ?1
             ORDER BY is_default DESC, name ASC"
        )?;

        let rows = stmt.query_map(params![game_id], |row| {
            let pid: String = row.get(0)?;
            let is_default_int: i32 = row.get(4)?;
            let is_active_int: i32 = row.get(5)?;
            Ok((pid, row.get(1)?, row.get(2)?, row.get(3)?, is_default_int == 1, is_active_int == 1, row.get(6)?, row.get(7)?))
        })?;

        let mut list = Vec::new();
        for r in rows {
            let (pid, gid, name, desc, def, act, cat, uat) = r?;
            // fetch mod ids
            let mut mod_stmt = conn.prepare("SELECT mod_id FROM profile_mods WHERE profile_id = ?1")?;
            let mod_rows = mod_stmt.query_map(params![pid], |mr| mr.get(0))?;
            let mut mod_ids = Vec::new();
            for m in mod_rows {
                mod_ids.push(m?);
            }

            list.push(Profile {
                id: pid,
                game_installation_id: gid,
                name,
                description: desc,
                is_default: def,
                is_active: act,
                installed_mod_ids: mod_ids,
                created_at: cat,
                updated_at: uat,
            });
        }

        Ok(list)
    }

    pub fn delete_profile(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM profile_mods WHERE profile_id = ?1", params![id])?;
        conn.execute("DELETE FROM profiles WHERE id = ?1", params![id])?;
        Ok(())
    }

    // --- Installed Mods & Files ---
    pub fn save_installed_mod(&self, mod_item: &InstalledMod) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let notes_json = serde_json::to_string(&mod_item.conflict_notes).unwrap_or_default();

        conn.execute(
            "INSERT INTO installed_mods (
                id, game_installation_id, profile_id, name, author, version,
                source_name, source_url, installed_at, strategy, enabled,
                has_conflicts, conflict_notes_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            ON CONFLICT(id) DO UPDATE SET
                enabled = excluded.enabled,
                has_conflicts = excluded.has_conflicts,
                conflict_notes_json = excluded.conflict_notes_json",
            params![
                mod_item.id,
                mod_item.game_installation_id,
                mod_item.profile_id,
                mod_item.name,
                mod_item.author,
                mod_item.version,
                mod_item.source_name,
                mod_item.source_url,
                mod_item.installed_at,
                mod_item.strategy,
                if mod_item.enabled { 1 } else { 0 },
                if mod_item.has_conflicts { 1 } else { 0 },
                notes_json,
            ],
        )?;

        // Save files
        conn.execute("DELETE FROM installed_files WHERE installed_mod_id = ?1", params![mod_item.id])?;
        for file in &mod_item.files {
            conn.execute(
                "INSERT INTO installed_files (id, installed_mod_id, destination_rel_path, sha256, file_size_bytes, has_backup, backup_path)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    file.id,
                    file.installed_mod_id,
                    file.destination_rel_path,
                    file.sha256,
                    file.file_size_bytes,
                    if file.has_backup { 1 } else { 0 },
                    file.backup_path,
                ],
            )?;
        }

        Ok(())
    }

    pub fn get_installed_mods(&self, game_id: &str) -> Result<Vec<InstalledMod>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, game_installation_id, profile_id, name, author, version,
                    source_name, source_url, installed_at, strategy, enabled,
                    has_conflicts, conflict_notes_json
             FROM installed_mods
             WHERE game_installation_id = ?1
             ORDER BY installed_at DESC"
        )?;

        let rows = stmt.query_map(params![game_id], |row| {
            let id: String = row.get(0)?;
            let enabled_int: i32 = row.get(10)?;
            let conflicts_int: i32 = row.get(11)?;
            let notes_json: String = row.get(12)?;
            let notes: Vec<String> = serde_json::from_str(&notes_json).unwrap_or_default();

            Ok((
                id,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
                row.get(9)?,
                enabled_int == 1,
                conflicts_int == 1,
                notes,
            ))
        })?;

        let mut list = Vec::new();
        for r in rows {
            let (id, gid, pid, name, author, ver, sname, surl, inst_at, strat, en, hc, notes) = r?;
            // Fetch files
            let mut file_stmt = conn.prepare(
                "SELECT id, installed_mod_id, destination_rel_path, sha256, file_size_bytes, has_backup, backup_path
                 FROM installed_files
                 WHERE installed_mod_id = ?1"
            )?;
            let file_rows = file_stmt.query_map(params![id], |fr| {
                let hb_int: i32 = fr.get(5)?;
                Ok(InstalledFileRecord {
                    id: fr.get(0)?,
                    installed_mod_id: fr.get(1)?,
                    destination_rel_path: fr.get(2)?,
                    sha256: fr.get(3)?,
                    file_size_bytes: fr.get(4)?,
                    has_backup: hb_int == 1,
                    backup_path: fr.get(6)?,
                })
            })?;

            let mut files = Vec::new();
            for f in file_rows {
                files.push(f?);
            }

            list.push(InstalledMod {
                id,
                game_installation_id: gid,
                profile_id: pid,
                name,
                author,
                version: ver,
                source_name: sname,
                source_url: surl,
                installed_at: inst_at,
                strategy: strat,
                enabled: en,
                files,
                has_conflicts: hc,
                conflict_notes: notes,
            });
        }

        Ok(list)
    }

    pub fn delete_installed_mod(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM installed_files WHERE installed_mod_id = ?1", params![id])?;
        conn.execute("DELETE FROM installed_mods WHERE id = ?1", params![id])?;
        conn.execute("DELETE FROM profile_mods WHERE mod_id = ?1", params![id])?;
        Ok(())
    }

    // --- Backups ---
    pub fn save_backup(&self, record: &BackupRecord) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO backups (
                id, game_installation_id, mod_id, mod_name, destination_rel_path,
                backup_file_path, original_sha256, original_size_bytes, timestamp, restored
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            ON CONFLICT(id) DO UPDATE SET restored = excluded.restored",
            params![
                record.id,
                record.game_installation_id,
                record.mod_id,
                record.mod_name,
                record.destination_rel_path,
                record.backup_file_path,
                record.original_sha256,
                record.original_size_bytes,
                record.timestamp,
                if record.restored { 1 } else { 0 },
            ],
        )?;
        Ok(())
    }

    pub fn get_backups(&self, game_id: &str) -> Result<Vec<BackupRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, game_installation_id, mod_id, mod_name, destination_rel_path,
                    backup_file_path, original_sha256, original_size_bytes, timestamp, restored
             FROM backups
             WHERE game_installation_id = ?1
             ORDER BY timestamp DESC"
        )?;

        let rows = stmt.query_map(params![game_id], |row| {
            let restored_int: i32 = row.get(9)?;
            Ok(BackupRecord {
                id: row.get(0)?,
                game_installation_id: row.get(1)?,
                mod_id: row.get(2)?,
                mod_name: row.get(3)?,
                destination_rel_path: row.get(4)?,
                backup_file_path: row.get(5)?,
                original_sha256: row.get(6)?,
                original_size_bytes: row.get(7)?,
                timestamp: row.get(8)?,
                restored: restored_int == 1,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // --- Recovery Journal ---
    pub fn save_journal_entry(&self, entry: &RecoveryJournalEntry) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let staged_json = serde_json::to_string(&entry.staged_backup_paths).unwrap_or_default();
        let affected_json = serde_json::to_string(&entry.affected_files).unwrap_or_default();

        conn.execute(
            "INSERT INTO recovery_journal (
                id, operation_id, operation_type, status, target_mod_id,
                target_mod_name, timestamp, staged_backups_json, affected_files_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET status = excluded.status",
            params![
                entry.id,
                entry.operation_id,
                entry.operation_type,
                entry.status,
                entry.target_mod_id,
                entry.target_mod_name,
                entry.timestamp,
                staged_json,
                affected_json,
            ],
        )?;
        Ok(())
    }

    pub fn get_pending_journal_entries(&self) -> Result<Vec<RecoveryJournalEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, operation_id, operation_type, status, target_mod_id,
                    target_mod_name, timestamp, staged_backups_json, affected_files_json
             FROM recovery_journal
             WHERE status = 'in_progress'
             ORDER BY timestamp ASC"
        )?;

        let rows = stmt.query_map([], |row| {
            let staged_json: String = row.get(7)?;
            let staged: Vec<String> = serde_json::from_str(&staged_json).unwrap_or_default();
            let affected_json: String = row.get(8)?;
            let affected: Vec<String> = serde_json::from_str(&affected_json).unwrap_or_default();

            Ok(RecoveryJournalEntry {
                id: row.get(0)?,
                operation_id: row.get(1)?,
                operation_type: row.get(2)?,
                status: row.get(3)?,
                target_mod_id: row.get(4)?,
                target_mod_name: row.get(5)?,
                timestamp: row.get(6)?,
                staged_backup_paths: staged,
                affected_files: affected,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // --- Conflicts ---
    pub fn save_conflict(&self, conflict: &FileConflict) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO conflicts (
                id, destination_rel_path, active_mod_id, active_mod_name,
                conflicting_mod_id, conflicting_mod_name, status, resolution_notes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO UPDATE SET
                status = excluded.status,
                resolution_notes = excluded.resolution_notes",
            params![
                conflict.id,
                conflict.destination_rel_path,
                conflict.active_mod_id,
                conflict.active_mod_name,
                conflict.conflicting_mod_id,
                conflict.conflicting_mod_name,
                conflict.status,
                conflict.resolution_notes,
            ],
        )?;
        Ok(())
    }

    pub fn get_conflicts(&self) -> Result<Vec<FileConflict>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, destination_rel_path, active_mod_id, active_mod_name,
                    conflicting_mod_id, conflicting_mod_name, status, resolution_notes
             FROM conflicts
             ORDER BY destination_rel_path ASC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(FileConflict {
                id: row.get(0)?,
                destination_rel_path: row.get(1)?,
                active_mod_id: row.get(2)?,
                active_mod_name: row.get(3)?,
                conflicting_mod_id: row.get(4)?,
                conflicting_mod_name: row.get(5)?,
                status: row.get(6)?,
                resolution_notes: row.get(7)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // --- App Settings ---
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM app_settings WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            let val: String = row.get(0)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO app_settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    // --- Mod Catalog Cache ---
    pub fn cache_mod_record(&self, mod_rec: &ModRecord) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let tags_json = serde_json::to_string(&mod_rec.tags).unwrap_or_default();
        let gv_json = serde_json::to_string(&mod_rec.supported_game_versions).unwrap_or_default();
        let deps_json = serde_json::to_string(&mod_rec.required_dependencies).unwrap_or_default();
        let incomp_json = serde_json::to_string(&mod_rec.incompatibilities).unwrap_or_default();
        let strat_str = match mod_rec.installation_strategy {
            InstallationStrategyType::Cleo => "cleo",
            InstallationStrategyType::Asi => "asi",
            InstallationStrategyType::Modloader => "modloader",
            InstallationStrategyType::Assets => "assets",
            InstallationStrategyType::Audio => "audio",
            InstallationStrategyType::Loose => "loose",
            InstallationStrategyType::Manual => "manual",
            InstallationStrategyType::Guided => "guided",
            InstallationStrategyType::Unknown => "unknown",
        };

        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO mod_records (
                id, source_id, source_mod_id, title, author, description, category,
                tags_json, thumbnail_url, page_url, download_url, file_size_formatted,
                file_size_bytes, release_date, update_date, supported_game_versions_json,
                required_dependencies_json, incompatibilities_json, installation_strategy,
                confidence, compatibility_reason, cached_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                author = excluded.author,
                description = excluded.description,
                category = excluded.category,
                tags_json = excluded.tags_json,
                thumbnail_url = excluded.thumbnail_url,
                page_url = excluded.page_url,
                download_url = excluded.download_url,
                file_size_formatted = excluded.file_size_formatted,
                file_size_bytes = excluded.file_size_bytes,
                release_date = excluded.release_date,
                update_date = excluded.update_date,
                supported_game_versions_json = excluded.supported_game_versions_json,
                required_dependencies_json = excluded.required_dependencies_json,
                incompatibilities_json = excluded.incompatibilities_json,
                installation_strategy = excluded.installation_strategy,
                confidence = excluded.confidence,
                compatibility_reason = excluded.compatibility_reason,
                cached_at = excluded.cached_at",
            params![
                mod_rec.id,
                mod_rec.source_id,
                mod_rec.source_mod_id,
                mod_rec.title,
                mod_rec.author,
                mod_rec.description,
                mod_rec.category,
                tags_json,
                mod_rec.thumbnail_url,
                mod_rec.page_url,
                mod_rec.download_url,
                mod_rec.file_size_formatted,
                mod_rec.file_size_bytes,
                mod_rec.release_date,
                mod_rec.update_date,
                gv_json,
                deps_json,
                incomp_json,
                strat_str,
                mod_rec.confidence,
                mod_rec.compatibility_reason,
                now,
            ],
        )?;

        Ok(())
    }

    pub fn query_cached_mods(&self, query: &str, category: Option<&str>, source: Option<&str>) -> Result<Vec<ModRecord>> {
        let conn = self.conn.lock().unwrap();
        let query_pattern = format!("%{}%", query);

        let mut sql = String::from(
            "SELECT id, source_id, source_mod_id, title, author, description, category,
                    tags_json, thumbnail_url, page_url, download_url, file_size_formatted,
                    file_size_bytes, release_date, update_date, supported_game_versions_json,
                    required_dependencies_json, incompatibilities_json, installation_strategy,
                    confidence, compatibility_reason
             FROM mod_records
             WHERE (title LIKE ?1 OR description LIKE ?1 OR author LIKE ?1)"
        );

        if let Some(cat) = category {
            if !cat.is_empty() && cat != "All" {
                sql.push_str(&format!(" AND category = '{}'", cat.replace('\'', "''")));
            }
        }

        if let Some(src) = source {
            if !src.is_empty() && src != "All" {
                sql.push_str(&format!(" AND source_id = '{}'", src.replace('\'', "''")));
            }
        }

        sql.push_str(" ORDER BY update_date DESC, title ASC LIMIT 100");

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![query_pattern], |row| {
            let tags_json: String = row.get(7)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let gv_json: String = row.get(15)?;
            let gv: Vec<String> = serde_json::from_str(&gv_json).unwrap_or_default();
            let deps_json: String = row.get(16)?;
            let deps: Vec<String> = serde_json::from_str(&deps_json).unwrap_or_default();
            let incomp_json: String = row.get(17)?;
            let incomp: Vec<String> = serde_json::from_str(&incomp_json).unwrap_or_default();
            let strat_str: String = row.get(18)?;
            let strategy = match strat_str.as_str() {
                "cleo" => InstallationStrategyType::Cleo,
                "asi" => InstallationStrategyType::Asi,
                "modloader" => InstallationStrategyType::Modloader,
                "assets" => InstallationStrategyType::Assets,
                "audio" => InstallationStrategyType::Audio,
                "loose" => InstallationStrategyType::Loose,
                "manual" => InstallationStrategyType::Manual,
                "guided" => InstallationStrategyType::Guided,
                _ => InstallationStrategyType::Unknown,
            };

            Ok(ModRecord {
                id: row.get(0)?,
                source_id: row.get(1)?,
                source_mod_id: row.get(2)?,
                title: row.get(3)?,
                author: row.get(4)?,
                description: row.get(5)?,
                category: row.get(6)?,
                tags,
                thumbnail_url: row.get(8)?,
                page_url: row.get(9)?,
                download_url: row.get(10)?,
                file_size_formatted: row.get(11)?,
                file_size_bytes: row.get(12)?,
                release_date: row.get(13)?,
                update_date: row.get(14)?,
                supported_game_versions: gv,
                required_dependencies: deps,
                incompatibilities: incomp,
                installation_strategy: strategy,
                confidence: row.get(19)?,
                compatibility_reason: row.get(20)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // --- Download Jobs ---
    pub fn save_download_job(&self, job: &DownloadJob) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO download_jobs (
                id, mod_id, mod_title, source_id, source_name, url, temp_file_path,
                total_bytes, downloaded_bytes, speed_bytes_per_sec, status, error_message,
                expected_sha256, computed_sha256, sha256_matched, start_time, completed_time
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
            ON CONFLICT(id) DO UPDATE SET
                downloaded_bytes = excluded.downloaded_bytes,
                speed_bytes_per_sec = excluded.speed_bytes_per_sec,
                status = excluded.status,
                error_message = excluded.error_message,
                computed_sha256 = excluded.computed_sha256,
                sha256_matched = excluded.sha256_matched,
                completed_time = excluded.completed_time",
            params![
                job.id,
                job.mod_id,
                job.mod_title,
                job.source_id,
                job.source_name,
                job.url,
                job.temp_file_path,
                job.total_bytes,
                job.downloaded_bytes,
                job.speed_bytes_per_sec,
                job.status,
                job.error_message,
                job.expected_sha256,
                job.computed_sha256,
                job.sha256_matched.map(|b| if b { 1 } else { 0 }),
                job.start_time,
                job.completed_time,
            ],
        )?;
        Ok(())
    }

    pub fn get_download_jobs(&self) -> Result<Vec<DownloadJob>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, mod_id, mod_title, source_id, source_name, url, temp_file_path,
                    total_bytes, downloaded_bytes, speed_bytes_per_sec, status, error_message,
                    expected_sha256, computed_sha256, sha256_matched, start_time, completed_time
             FROM download_jobs
             ORDER BY start_time DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            let matched_opt: Option<i32> = row.get(14)?;
            Ok(DownloadJob {
                id: row.get(0)?,
                mod_id: row.get(1)?,
                mod_title: row.get(2)?,
                source_id: row.get(3)?,
                source_name: row.get(4)?,
                url: row.get(5)?,
                temp_file_path: row.get(6)?,
                total_bytes: row.get(7)?,
                downloaded_bytes: row.get(8)?,
                speed_bytes_per_sec: row.get(9)?,
                status: row.get(10)?,
                error_message: row.get(11)?,
                expected_sha256: row.get(12)?,
                computed_sha256: row.get(13)?,
                sha256_matched: matched_opt.map(|v| v == 1),
                start_time: row.get(15)?,
                completed_time: row.get(16)?,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }
}
