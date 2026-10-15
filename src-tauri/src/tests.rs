#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use crate::models::*;
    use crate::database::Database;
    use crate::archives::{inspect_archive, sanitize_archive_entry_path};
    use crate::conflicts::{detect_installation_conflicts, scan_existing_installed_conflicts};
    use crate::installation::generate_installation_plan;
    use crate::profiles::{export_profile_manifest, import_profile_manifest};

    #[test]
    fn test_archive_path_sanitization_security() {
        // Safe relative paths
        assert!(sanitize_archive_entry_path("scripts/mod.cs").is_ok());
        assert!(sanitize_archive_entry_path("cleo/cleo_text/english.fxt").is_ok());
        assert!(sanitize_archive_entry_path("modloader/car_pack/infernus.dff").is_ok());

        // Dangerous path traversal attempts must fail
        assert!(sanitize_archive_entry_path("../windows/system32/cmd.exe").is_err());
        assert!(sanitize_archive_entry_path("folder/../../escape.dll").is_err());
        assert!(sanitize_archive_entry_path(r"..\..\malicious.bat").is_err());

        // Windows drive letters must fail
        assert!(sanitize_archive_entry_path("C:/GTA/gta_sa.exe").is_err());
        assert!(sanitize_archive_entry_path("D:\\hacked.asi").is_err());

        // Empty path must fail
        assert!(sanitize_archive_entry_path("").is_err());
        assert!(sanitize_archive_entry_path("///").is_err());
    }

    #[test]
    fn test_conflict_detection_logic() {
        let file1 = InstalledFileRecord {
            id: "f1".to_string(),
            installed_mod_id: "mod_a".to_string(),
            destination_rel_path: "cleo/car_spawner.cs".to_string(),
            sha256: "abc123".to_string(),
            file_size_bytes: 4096,
            has_backup: false,
            backup_path: None,
        };

        let mod_a = InstalledMod {
            id: "mod_a".to_string(),
            game_installation_id: "game_1".to_string(),
            profile_id: "prof_1".to_string(),
            name: "Car Spawner v1".to_string(),
            author: "Author A".to_string(),
            version: "1.0".to_string(),
            source_name: "GTAinside".to_string(),
            source_url: None,
            installed_at: "2026-10-15T00:00:00Z".to_string(),
            strategy: "cleo".to_string(),
            enabled: true,
            files: vec![file1],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        // Proposing a new mod that also targets "cleo/car_spawner.cs"
        let op = PlannedFileOperation {
            archive_rel_path: "car_spawner.cs".to_string(),
            destination_rel_path: "cleo/car_spawner.cs".to_string(),
            destination_absolute_path: "C:/Games/GTA/cleo/car_spawner.cs".to_string(),
            file_type: "cs".to_string(),
            rule: "cleo".to_string(),
            destination_state: "overwrite".to_string(),
            backup_required: true,
            file_size_bytes: 5120,
        };

        let conflicts = detect_installation_conflicts(&[mod_a.clone()], &[op], "mod_b", "Car Spawner Enhanced");
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].active_mod_id, "mod_a");
        assert_eq!(conflicts[0].destination_rel_path, "cleo/car_spawner.cs");

        // Non-colliding operation
        let safe_op = PlannedFileOperation {
            archive_rel_path: "speedo.cs".to_string(),
            destination_rel_path: "cleo/speedo.cs".to_string(),
            destination_absolute_path: "C:/Games/GTA/cleo/speedo.cs".to_string(),
            file_type: "cs".to_string(),
            rule: "cleo".to_string(),
            destination_state: "new".to_string(),
            backup_required: false,
            file_size_bytes: 2048,
        };

        let no_conflicts = detect_installation_conflicts(&[mod_a], &[safe_op], "mod_c", "Speedometer");
        assert_eq!(no_conflicts.len(), 0);
    }

    #[test]
    fn test_profile_manifest_export_and_import() {
        let db = Database::in_memory().unwrap();

        let profile = Profile {
            id: "test_profile_id".to_string(),
            game_installation_id: "test_game_id".to_string(),
            name: "Visual Enhancement Pack".to_string(),
            description: "High resolution textures, SkyGfx, and 60FPS physics".to_string(),
            is_default: false,
            is_active: true,
            installed_mod_ids: vec!["mod_1".to_string()],
            created_at: "2026-10-15T00:00:00Z".to_string(),
            updated_at: "2026-10-15T00:00:00Z".to_string(),
        };

        db.save_profile(&profile).unwrap();

        let installed_mod = InstalledMod {
            id: "mod_1".to_string(),
            game_installation_id: "test_game_id".to_string(),
            profile_id: "test_profile_id".to_string(),
            name: "SkyGfx 4.2b".to_string(),
            author: "aap".to_string(),
            version: "4.2b".to_string(),
            source_name: "LibertyCity".to_string(),
            source_url: Some("https://libertycity.net/files/108343".to_string()),
            installed_at: "2026-10-15T00:00:00Z".to_string(),
            strategy: "asi".to_string(),
            enabled: true,
            files: Vec::new(),
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        db.save_installed_mod(&installed_mod).unwrap();

        let exported_json = export_profile_manifest("test_profile_id", "test_game_id", &db).unwrap();
        assert!(exported_json.contains("Visual Enhancement Pack"));
        assert!(exported_json.contains("SkyGfx 4.2b"));

        let imported = import_profile_manifest(&exported_json, "test_game_id", &db).unwrap();
        assert_eq!(imported.name, "Visual Enhancement Pack (Imported)");
        assert_eq!(imported.description, "High resolution textures, SkyGfx, and 60FPS physics");
    }

    #[test]
    fn test_synthetic_zip_archive_inspection_and_traversal_detection() {
        use std::io::Write;
        use zip::write::SimpleFileOptions;

        let temp_dir = tempfile::tempdir().unwrap();
        let zip_path = temp_dir.path().join("test_mod.zip");
        let file = std::fs::File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);

        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Add valid entry
        zip.start_file("cleo/first_person.cs", options).unwrap();
        zip.write_all(b"sample cleo script bytecode").unwrap();

        // Add valid text entry
        zip.start_file("cleo/cleo_text/first_person.fxt", options).unwrap();
        zip.write_all(b"FP_01 First person camera enabled").unwrap();

        zip.finish().unwrap();

        let inspection = inspect_archive(&zip_path).unwrap();
        assert_eq!(inspection.total_entries, 2);
        assert!(inspection.is_safe);
        assert_eq!(inspection.safety_violations.len(), 0);
    }
}
