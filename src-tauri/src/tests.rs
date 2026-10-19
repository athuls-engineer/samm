#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::database::Database;
    use crate::archives::{inspect_archive, sanitize_archive_entry_path};
    use crate::conflicts::detect_installation_conflicts;
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

    #[test]
    fn test_existing_mods_scanning() {
        use crate::existing_mods::scan_existing_mods;
        use std::fs;

        let temp_dir = tempfile::tempdir().unwrap();
        let root = temp_dir.path();

        // 1. Simulate root ASI plugins
        fs::write(root.join("SilentPatchSA.asi"), b"dummy silentpatch").unwrap();
        fs::write(root.join("SilentPatchSA.ini"), b"dummy config").unwrap();
        fs::write(root.join("GTASA.WidescreenFix.asi"), b"dummy widescreen").unwrap();

        // 2. Simulate ModLoader
        fs::create_dir_all(root.join("modloader").join("SkyGfx")).unwrap();
        fs::write(root.join("modloader").join("SkyGfx").join("skygfx.asi"), b"dummy skygfx").unwrap();
        fs::create_dir_all(root.join("modloader").join("RoSA Evolved")).unwrap();
        fs::write(root.join("modloader").join("RoSA Evolved").join("desert.txd"), b"dummy texture").unwrap();

        // 3. Simulate CLEO
        fs::create_dir_all(root.join("cleo")).unwrap();
        fs::write(root.join("cleo").join("CarSpawner.cs"), b"dummy cleo script").unwrap();
        fs::write(root.join("CLEO.asi"), b"dummy cleo asi").unwrap();

        // 4. Simulate ASI Loader
        fs::write(root.join("vorbisFile.dll"), b"dummy vorbis asi loader").unwrap();

        // 5. Simulate Rainbomizer (Chaos / Randomizer mod)
        let scripts_dir = root.join("scripts");
        fs::create_dir_all(&scripts_dir).unwrap();
        fs::write(scripts_dir.join("SA.Rainbomizer.asi"), b"dummy rainbomizer plugin").unwrap();
        let rainbomizer_dir = root.join("rainbomizer");
        fs::create_dir_all(&rainbomizer_dir).unwrap();
        fs::write(rainbomizer_dir.join("config.toml"), b"# rainbomizer config").unwrap();

        let detected = scan_existing_mods(root);

        // Verify detection
        assert!(detected.iter().any(|m| m.id.contains("silentpatch")), "SilentPatch should be detected");
        assert!(detected.iter().any(|m| m.id.contains("widescreen")), "WidescreenFix should be detected");
        assert!(detected.iter().any(|m| m.id == "libertycity_cleo4"), "CLEO 4 should be detected");
        assert!(detected.iter().any(|m| m.name.contains("RoSA Evolved")), "RoSA Evolved modloader folder should be detected");
        assert!(detected.iter().any(|m| m.name.contains("CarSpawner")), "CarSpawner CLEO script should be detected");
        assert!(detected.iter().any(|m| m.id == "local_asi_loader"), "Silent ASI Loader should be detected");
        assert!(detected.iter().any(|m| m.id == "github_rainbomizer"), "Rainbomizer should be detected");
        let rainbomizer = detected.iter().find(|m| m.id == "github_rainbomizer").unwrap();
        assert!(rainbomizer.relative_files.iter().any(|f| f == "scripts/SA.Rainbomizer.asi"));
        assert!(rainbomizer.relative_files.iter().any(|f| f == "rainbomizer/config.toml"));
    }

    #[test]
    fn test_conflict_resolution_and_auto_resolve() {
        let db = Database::in_memory().unwrap();

        let mod1 = InstalledMod {
            id: "local_asi_loader".to_string(),
            game_installation_id: "game_sa".to_string(),
            profile_id: "default".to_string(),
            name: "Silent's ASI Loader".to_string(),
            author: "Silent".to_string(),
            version: "1.3".to_string(),
            source_name: "Pre-installed".to_string(),
            source_url: None,
            installed_at: "2026-10-16T00:00:00Z".to_string(),
            strategy: "asi".to_string(),
            enabled: true,
            files: vec![InstalledFileRecord {
                id: "f1".to_string(),
                installed_mod_id: "local_asi_loader".to_string(),
                destination_rel_path: "vorbisFile.dll".to_string(),
                sha256: "aaa111".to_string(),
                file_size_bytes: 50000,
                has_backup: false,
                backup_path: None,
            }],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        let mod2 = InstalledMod {
            id: "gtainside_widescreen_fix".to_string(),
            game_installation_id: "game_sa".to_string(),
            profile_id: "default".to_string(),
            name: "Widescreen Fix".to_string(),
            author: "ThirteenAG".to_string(),
            version: "1.0".to_string(),
            source_name: "Catalog".to_string(),
            source_url: None,
            installed_at: "2026-10-16T00:00:00Z".to_string(),
            strategy: "asi".to_string(),
            enabled: true,
            files: vec![
                InstalledFileRecord {
                    id: "f2".to_string(),
                    installed_mod_id: "gtainside_widescreen_fix".to_string(),
                    destination_rel_path: "vorbisFile.dll".to_string(),
                    sha256: "aaa111".to_string(),
                    file_size_bytes: 50000,
                    has_backup: false,
                    backup_path: None,
                },
                InstalledFileRecord {
                    id: "f3".to_string(),
                    installed_mod_id: "gtainside_widescreen_fix".to_string(),
                    destination_rel_path: "scripts/GTASA.WidescreenFix.asi".to_string(),
                    sha256: "bbb222".to_string(),
                    file_size_bytes: 120000,
                    has_backup: false,
                    backup_path: None,
                },
            ],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        db.save_installed_mod(&mod1).unwrap();
        db.save_installed_mod(&mod2).unwrap();

        // Detect conflicts
        let installed = db.get_installed_mods("game_sa").unwrap();
        let conflicts = crate::conflicts::scan_existing_installed_conflicts(&installed);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].destination_rel_path, "vorbisFile.dll");

        // Auto resolve conflicts
        let resolved_count = db.auto_resolve_all_conflicts("game_sa").unwrap();
        assert_eq!(resolved_count, 1);

        // Verify zero conflicts remain
        let after_installed = db.get_installed_mods("game_sa").unwrap();
        let remaining_conflicts = crate::conflicts::scan_existing_installed_conflicts(&after_installed);
        assert_eq!(remaining_conflicts.len(), 0);

        // Verify Silent's ASI Loader still keeps vorbisFile.dll
        let asi_loader = after_installed.iter().find(|m| m.id == "local_asi_loader").unwrap();
        assert!(asi_loader.files.iter().any(|f| f.destination_rel_path == "vorbisFile.dll"));

        // Verify Widescreen Fix still has its main script file
        let wsf = after_installed.iter().find(|m| m.id == "gtainside_widescreen_fix").unwrap();
        assert!(wsf.files.iter().any(|f| f.destination_rel_path == "scripts/GTASA.WidescreenFix.asi"));
        assert!(!wsf.files.iter().any(|f| f.destination_rel_path == "vorbisFile.dll"));
    }

    #[test]
    fn test_physical_mod_toggling_and_restoration() {
        use std::fs;
        use crate::installation::apply_physical_mod_state;

        let temp_dir = tempfile::tempdir().unwrap();
        let game_root = temp_dir.path();

        // 1. Test ModLoader directory toggling
        let ml_dir = game_root.join("modloader").join("CarPack");
        fs::create_dir_all(&ml_dir).unwrap();
        fs::write(ml_dir.join("infernus.dff"), b"dummy dff").unwrap();

        let mut ml_mod = InstalledMod {
            id: "local_ml_carpack".to_string(),
            game_installation_id: "game1".to_string(),
            profile_id: "prof1".to_string(),
            name: "Car Pack".to_string(),
            author: "Author".to_string(),
            version: "1.0".to_string(),
            source_name: "Pre-installed".to_string(),
            source_url: None,
            installed_at: "2026-10-16T00:00:00Z".to_string(),
            strategy: "modloader".to_string(),
            enabled: true,
            files: vec![InstalledFileRecord {
                id: "f1".to_string(),
                installed_mod_id: "local_ml_carpack".to_string(),
                destination_rel_path: "modloader/CarPack/infernus.dff".to_string(),
                sha256: "".to_string(),
                file_size_bytes: 100,
                has_backup: false,
                backup_path: None,
            }],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        // Disable: should rename modloader/CarPack -> modloader/CarPack.disabled
        apply_physical_mod_state(&mut ml_mod, false, game_root);
        assert!(!game_root.join("modloader").join("CarPack").exists());
        assert!(game_root.join("modloader").join("CarPack.disabled").is_dir());
        assert!(game_root.join("modloader").join("CarPack.disabled").join("infernus.dff").is_file());

        // Re-enable: should restore modloader/CarPack.disabled -> modloader/CarPack
        apply_physical_mod_state(&mut ml_mod, true, game_root);
        assert!(game_root.join("modloader").join("CarPack").is_dir());
        assert!(!game_root.join("modloader").join("CarPack.disabled").exists());

        // 2. Test ASI plugin file toggling
        let asi_path = game_root.join("SilentPatchSA.asi");
        fs::write(&asi_path, b"dummy asi content").unwrap();

        let mut asi_mod = InstalledMod {
            id: "gtainside_silentpatch".to_string(),
            game_installation_id: "game1".to_string(),
            profile_id: "prof1".to_string(),
            name: "SilentPatch".to_string(),
            author: "Silent".to_string(),
            version: "1.0".to_string(),
            source_name: "Pre-installed".to_string(),
            source_url: None,
            installed_at: "2026-10-16T00:00:00Z".to_string(),
            strategy: "asi".to_string(),
            enabled: true,
            files: vec![InstalledFileRecord {
                id: "f2".to_string(),
                installed_mod_id: "gtainside_silentpatch".to_string(),
                destination_rel_path: "SilentPatchSA.asi".to_string(),
                sha256: "".to_string(),
                file_size_bytes: 200,
                has_backup: false,
                backup_path: None,
            }],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        // Disable: should rename SilentPatchSA.asi -> SilentPatchSA.asi.disabled
        apply_physical_mod_state(&mut asi_mod, false, game_root);
        assert!(!game_root.join("SilentPatchSA.asi").exists());
        assert!(game_root.join("SilentPatchSA.asi.disabled").is_file());

        // Re-enable: should restore SilentPatchSA.asi.disabled -> SilentPatchSA.asi
        apply_physical_mod_state(&mut asi_mod, true, game_root);
        assert!(game_root.join("SilentPatchSA.asi").is_file());
        assert!(!game_root.join("SilentPatchSA.asi.disabled").exists());

        // 3. Test CLEO script toggling
        let cleo_dir = game_root.join("cleo");
        fs::create_dir_all(&cleo_dir).unwrap();
        let cs_path = cleo_dir.join("CarSpawner.cs");
        fs::write(&cs_path, b"dummy script").unwrap();

        let mut cleo_mod = InstalledMod {
            id: "local_cleo_carspawner".to_string(),
            game_installation_id: "game1".to_string(),
            profile_id: "prof1".to_string(),
            name: "Car Spawner".to_string(),
            author: "Author".to_string(),
            version: "1.0".to_string(),
            source_name: "Pre-installed".to_string(),
            source_url: None,
            installed_at: "2026-10-16T00:00:00Z".to_string(),
            strategy: "cleo".to_string(),
            enabled: true,
            files: vec![InstalledFileRecord {
                id: "f3".to_string(),
                installed_mod_id: "local_cleo_carspawner".to_string(),
                destination_rel_path: "cleo/CarSpawner.cs".to_string(),
                sha256: "".to_string(),
                file_size_bytes: 50,
                has_backup: false,
                backup_path: None,
            }],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        // Disable: should rename cleo/CarSpawner.cs -> cleo/CarSpawner.cs.disabled
        apply_physical_mod_state(&mut cleo_mod, false, game_root);
        assert!(!game_root.join("cleo").join("CarSpawner.cs").exists());
        assert!(game_root.join("cleo").join("CarSpawner.cs.disabled").is_file());

        // Re-enable: should restore
        apply_physical_mod_state(&mut cleo_mod, true, game_root);
        assert!(game_root.join("cleo").join("CarSpawner.cs").is_file());
        assert!(!game_root.join("cleo").join("CarSpawner.cs.disabled").exists());
    }

    #[test]
    fn test_ghost_mod_cleanup_on_resync() {
        use std::fs;
        use crate::existing_mods::sync_existing_mods_into_db;

        let temp_dir = tempfile::tempdir().unwrap();
        let game_root = temp_dir.path();
        let db = Database::in_memory().unwrap();

        let game = GameInstallation {
            id: "game_resync".to_string(),
            name: "GTA SA".to_string(),
            executable_path: game_root.join("gta_sa.exe").to_string_lossy().to_string(),
            root_path: game_root.to_string_lossy().to_string(),
            detected_edition: "1.0".to_string(),
            version_confidence: ConfidenceLevel::Confirmed,
            executable_sha256: "".to_string(),
            executable_size_bytes: 0,
            pe_timestamp: None,
            evidence_list: Vec::new(),
            uncertainties: Vec::new(),
            is_classic_1_0: true,
            is_active: true,
            created_date: "".to_string(),
            last_verified_date: "".to_string(),
        };

        // Create an ASI plugin on disk
        let asi_file = game_root.join("SilentPatchSA.asi");
        fs::write(&asi_file, b"silentpatch").unwrap();

        // Initial sync: detects SilentPatch
        let synced1 = sync_existing_mods_into_db(&game, "default", &db).unwrap();
        assert_eq!(synced1.len(), 1);
        assert_eq!(synced1[0].id, "root_asi_silentpatchsa");

        let in_db = db.get_installed_mods("game_resync").unwrap();
        assert_eq!(in_db.len(), 1);

        // User deletes SilentPatchSA.asi from disk manually in Windows Explorer
        fs::remove_file(&asi_file).unwrap();

        // Re-sync: SAMM should automatically purge the ghost mod
        let _ = sync_existing_mods_into_db(&game, "default", &db).unwrap();
        let in_db_after = db.get_installed_mods("game_resync").unwrap();
        assert_eq!(in_db_after.len(), 0, "Ghost mod should be automatically purged from DB");
    }

    #[test]
    fn test_uninstall_mod_removes_disabled_files_and_modloader_folders() {
        use std::fs;
        use crate::installation::{uninstall_mod, apply_physical_mod_state};

        let temp_dir = tempfile::tempdir().unwrap();
        let game_root = temp_dir.path();
        let db = Database::in_memory().unwrap();

        let game = GameInstallation {
            id: "game_uninst".to_string(),
            name: "GTA SA".to_string(),
            executable_path: game_root.join("gta_sa.exe").to_string_lossy().to_string(),
            root_path: game_root.to_string_lossy().to_string(),
            detected_edition: "1.0".to_string(),
            version_confidence: ConfidenceLevel::Confirmed,
            executable_sha256: "".to_string(),
            executable_size_bytes: 0,
            pe_timestamp: None,
            evidence_list: Vec::new(),
            uncertainties: Vec::new(),
            is_classic_1_0: true,
            is_active: true,
            created_date: "".to_string(),
            last_verified_date: "".to_string(),
        };

        // Create ModLoader mod folder
        let ml_dir = game_root.join("modloader").join("WeatherPack");
        fs::create_dir_all(&ml_dir).unwrap();
        fs::write(ml_dir.join("timecyc.dat"), b"weather data").unwrap();

        let mut mod_rec = InstalledMod {
            id: "local_ml_weatherpack".to_string(),
            game_installation_id: "game_uninst".to_string(),
            profile_id: "default".to_string(),
            name: "Weather Pack".to_string(),
            author: "Modder".to_string(),
            version: "1.0".to_string(),
            source_name: "ModLoader".to_string(),
            source_url: None,
            installed_at: "".to_string(),
            strategy: "modloader".to_string(),
            enabled: true,
            files: vec![InstalledFileRecord {
                id: "f1".to_string(),
                installed_mod_id: "local_ml_weatherpack".to_string(),
                destination_rel_path: "modloader/WeatherPack/timecyc.dat".to_string(),
                sha256: "".to_string(),
                file_size_bytes: 50,
                has_backup: false,
                backup_path: None,
            }],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        db.save_installed_mod(&mod_rec).unwrap();

        // Disable the mod first (so it becomes WeatherPack.disabled)
        apply_physical_mod_state(&mut mod_rec, false, game_root);
        assert!(game_root.join("modloader").join("WeatherPack.disabled").exists());

        // Now uninstall the mod: must completely remove WeatherPack.disabled directory
        uninstall_mod("local_ml_weatherpack", &game, &db).unwrap();

        assert!(!game_root.join("modloader").join("WeatherPack").exists());
        assert!(!game_root.join("modloader").join("WeatherPack.disabled").exists(), "Disabled directory must be removed on uninstall");
        assert!(db.get_installed_mods("game_uninst").unwrap().is_empty());
    }

    #[test]
    fn test_rainbomizer_toggling_and_complete_uninstallation() {
        use std::fs;
        use crate::installation::{uninstall_mod, apply_physical_mod_state};

        let temp_dir = tempfile::tempdir().unwrap();
        let game_root = temp_dir.path();
        let db = Database::in_memory().unwrap();

        let game = GameInstallation {
            id: "game_rainbomizer_test".to_string(),
            name: "GTA SA".to_string(),
            executable_path: game_root.join("gta_sa.exe").to_string_lossy().to_string(),
            root_path: game_root.to_string_lossy().to_string(),
            detected_edition: "1.0".to_string(),
            version_confidence: ConfidenceLevel::Confirmed,
            executable_sha256: "".to_string(),
            executable_size_bytes: 0,
            pe_timestamp: None,
            evidence_list: Vec::new(),
            uncertainties: Vec::new(),
            is_classic_1_0: true,
            is_active: true,
            created_date: "".to_string(),
            last_verified_date: "".to_string(),
        };

        // 1. Setup mock Rainbomizer layout on disk
        let scripts_dir = game_root.join("scripts");
        fs::create_dir_all(&scripts_dir).unwrap();
        fs::write(scripts_dir.join("SA.Rainbomizer.asi"), b"dummy asi binary").unwrap();

        let r_dir = game_root.join("rainbomizer");
        let r_data = r_dir.join("data");
        let r_logs = r_dir.join("logs");
        fs::create_dir_all(&r_data).unwrap();
        fs::create_dir_all(&r_logs).unwrap();

        fs::write(r_dir.join("config.toml"), b"[General]\nSeed = 12345").unwrap();
        fs::write(r_data.join("Cutscene_Models.txt"), b"model list").unwrap();
        // Dynamic log created at runtime by game
        fs::write(r_logs.join("session_16-10-2026.txt"), b"runtime log").unwrap();

        fs::write(game_root.join("readme.txt"), b"# Rainbomizer for GTA San Andreas\nParik27").unwrap();

        let mut mod_rec = InstalledMod {
            id: "github_rainbomizer".to_string(),
            game_installation_id: "game_rainbomizer_test".to_string(),
            profile_id: "default".to_string(),
            name: "San Andreas Rainbomizer (Chaos & Randomizer Mod)".to_string(),
            author: "Parik27 & 123robot".to_string(),
            version: "3.3.1".to_string(),
            source_name: "Pre-installed (Rainbomizer)".to_string(),
            source_url: None,
            installed_at: "".to_string(),
            strategy: "asi".to_string(),
            enabled: true,
            files: vec![
                InstalledFileRecord {
                    id: "f1".to_string(),
                    installed_mod_id: "github_rainbomizer".to_string(),
                    destination_rel_path: "scripts/SA.Rainbomizer.asi".to_string(),
                    sha256: "".to_string(),
                    file_size_bytes: 50,
                    has_backup: false,
                    backup_path: None,
                },
                InstalledFileRecord {
                    id: "f2".to_string(),
                    installed_mod_id: "github_rainbomizer".to_string(),
                    destination_rel_path: "rainbomizer/config.toml".to_string(),
                    sha256: "".to_string(),
                    file_size_bytes: 50,
                    has_backup: false,
                    backup_path: None,
                },
                InstalledFileRecord {
                    id: "f3".to_string(),
                    installed_mod_id: "github_rainbomizer".to_string(),
                    destination_rel_path: "rainbomizer/data/Cutscene_Models.txt".to_string(),
                    sha256: "".to_string(),
                    file_size_bytes: 50,
                    has_backup: false,
                    backup_path: None,
                },
            ],
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        db.save_installed_mod(&mod_rec).unwrap();

        // 2. Test toggling OFF: dedicated directory and scripts plugin must both be disabled
        apply_physical_mod_state(&mut mod_rec, false, game_root);
        assert!(!scripts_dir.join("SA.Rainbomizer.asi").exists(), "Active ASI must be renamed");
        assert!(scripts_dir.join("SA.Rainbomizer.asi.disabled").is_file(), "Disabled ASI must exist");
        assert!(!game_root.join("rainbomizer").exists(), "Active rainbomizer dir must be renamed");
        assert!(game_root.join("rainbomizer.disabled").is_dir(), "rainbomizer.disabled must exist");
        assert!(game_root.join("rainbomizer.disabled").join("config.toml").is_file());
        assert!(game_root.join("rainbomizer.disabled").join("logs").join("session_16-10-2026.txt").is_file());

        // 3. Test toggling ON: dedicated directory and scripts plugin must both be cleanly restored
        apply_physical_mod_state(&mut mod_rec, true, game_root);
        assert!(scripts_dir.join("SA.Rainbomizer.asi").is_file(), "Active ASI must be restored");
        assert!(!scripts_dir.join("SA.Rainbomizer.asi.disabled").exists());
        assert!(game_root.join("rainbomizer").is_dir(), "Active rainbomizer dir must be restored");
        assert!(!game_root.join("rainbomizer.disabled").exists());
        assert!(game_root.join("rainbomizer").join("config.toml").is_file());

        // 4. Test complete uninstallation: everything including dynamic logs and readme must be wiped
        uninstall_mod("github_rainbomizer", &game, &db).unwrap();
        assert!(!scripts_dir.join("SA.Rainbomizer.asi").exists(), "ASI must be deleted");
        assert!(!scripts_dir.join("SA.Rainbomizer.asi.disabled").exists());
        assert!(!game_root.join("rainbomizer").exists(), "rainbomizer folder must be completely deleted");
        assert!(!game_root.join("rainbomizer.disabled").exists());
        assert!(!game_root.join("readme.txt").exists(), "Rainbomizer readme in game root must be removed");
        assert!(db.get_installed_mods("game_rainbomizer_test").unwrap().is_empty(), "DB record must be removed");
    }

    #[tokio::test]
    async fn test_essential_popular_catalog_has_over_50_mods() {
        use crate::sources::SourceManager;
        let source_mgr = SourceManager::new();
        let results = source_mgr.search_all("", Some("Essential & Popular"), None).await;
        println!("Total Essential & Popular mods found: {}", results.len());
        assert!(
            results.len() >= 50,
            "Expected at least 50 essential/popular mods in catalog, but found {}",
            results.len()
        );
        assert!(
            results.iter().any(|m| m.id == "mixmods_loadscreens_4k" || m.title.contains("Loadscreens 4K")),
            "Loadscreens 4K Definitive must be present in Essential catalog"
        );
    }

    #[tokio::test]
    async fn test_one_click_mods_exceed_150_and_catalog_exceeds_500() {
        use crate::sources::SourceManager;
        let source_mgr = SourceManager::new();
        let all_mods = source_mgr.search_all("", None, None).await;
        let one_click_count = all_mods.iter().filter(|m| m.download_url.is_some()).count();
        println!("Total catalog mods: {}, One-click installable mods: {}", all_mods.len(), one_click_count);
        let mut id_map = std::collections::HashMap::new();
        let mut title_map = std::collections::HashMap::new();
        let mut url_map = std::collections::HashMap::new();
        for m in &all_mods {
            *id_map.entry(&m.id).or_insert(0) += 1;
            let norm_title = m.title.trim().to_lowercase();
            title_map.entry(norm_title).or_insert_with(Vec::new).push(&m.id);
            if let Some(ref u) = m.download_url {
                url_map.entry(u.as_str()).or_insert_with(Vec::new).push(&m.id);
            }
        }
        for (id, count) in &id_map {
            if *count > 1 {
                println!("Duplicate ID found: {} (count: {})", id, count);
            }
        }
        for (title, ids) in &title_map {
            if ids.len() > 1 {
                println!("Duplicate normalized title found: '{}' -> {:?}", title, ids);
            }
        }
        for (url, ids) in &url_map {
            if ids.len() > 1 {
                println!("Duplicate download URL found: '{}' -> {:?}", url, ids);
            }
        }
        let dup_id_count = id_map.values().filter(|&&c| c > 1).count();
        let dup_url_count = url_map.values().filter(|v| v.len() > 1).count();
        assert_eq!(dup_id_count, 0, "All mods must have completely unique IDs");
        assert_eq!(dup_url_count, 0, "All 1-click mods must have unique download URLs");
        assert!(
            all_mods.len() >= 700,
            "Expected at least 700 mods in total catalog, found {}",
            all_mods.len()
        );
        assert!(
            one_click_count >= 700,
            "Expected at least 700 1-click installable mods, found {}",
            one_click_count
        );
    }

    #[test]
    fn test_7z_inspection_and_variant_resolution() {
        let sample_7z = std::path::Path::new(r"C:\Users\athul_nuy2ni9\Downloads\Loadscreens_4K_Definitive.7z");
        if sample_7z.exists() {
            let inspection = inspect_archive(sample_7z).expect("7z inspection must succeed");
            assert!(inspection.is_safe, "7z archive must be considered safe");
            assert_eq!(inspection.format, "7z", "Format should be detected as 7z");
            assert!(inspection.entries.iter().any(|e| e.path.contains("LOADSCS.txd")));

            let temp_dir = tempfile::tempdir().unwrap();
            let game_root = temp_dir.path();
            std::fs::create_dir_all(game_root.join("modloader")).unwrap();

            let fake_game = GameInstallation {
                id: "test_game_7z".to_string(),
                name: "GTA SA Test".to_string(),
                executable_path: game_root.join("gta_sa.exe").to_string_lossy().to_string(),
                root_path: game_root.to_string_lossy().to_string(),
                detected_edition: "1.0 US".to_string(),
                version_confidence: ConfidenceLevel::Confirmed,
                executable_sha256: "abc".to_string(),
                executable_size_bytes: 14383616,
                pe_timestamp: None,
                evidence_list: Vec::new(),
                uncertainties: Vec::new(),
                is_classic_1_0: true,
                is_active: true,
                created_date: "2026-10-17T00:00:00Z".to_string(),
                last_verified_date: "2026-10-17T00:00:00Z".to_string(),
            };

            let plan = generate_installation_plan(
                sample_7z,
                &fake_game,
                "mixmods_loadscreens_4k",
                "[SA] Loadscreens 4K Definitive v1.1",
            ).expect("Plan generation must succeed");

            assert_eq!(plan.installation_type, "automatic");
            assert_eq!(plan.strategy, InstallationStrategyType::Modloader);
            // Verify that 4K variant was chosen and 2K was omitted
            assert!(
                plan.file_operations.iter().any(|op| op.destination_rel_path.contains("models/txd/LOADSCS.txd")),
                "LOADSCS.txd must be placed in models/txd/"
            );
            assert!(
                !plan.file_operations.iter().any(|op| op.archive_rel_path.contains("Loadscreens 2K Definitive")),
                "Loadscreens 2K variant must be omitted when 4K is selected"
            );
            assert!(
                plan.warnings.iter().any(|w| w.to_lowercase().contains("selected 'loadscreens 4k definitive'")),
                "Notice must indicate automatic selection of 4K variant, got: {:?}",
                plan.warnings
            );

            let db = Database::in_memory().unwrap();
            let backup_dir = temp_dir.path().join("backups");
            let installed = crate::installation::execute_installation_transaction(
                sample_7z,
                &plan,
                &fake_game,
                "profile_test",
                &backup_dir,
                &db,
            ).expect("Installation transaction must succeed");

            assert_eq!(installed.id, "mixmods_loadscreens_4k");
            assert!(
                installed.files.iter().any(|f| f.destination_rel_path.contains("LOADSCS.txd")),
                "LOADSCS.txd must be recorded as installed file"
            );
        }
    }

    #[test]
    fn test_install_loadscreens_into_game() {
        let sample_7z = std::path::Path::new(r"C:\Users\athul_nuy2ni9\Downloads\Loadscreens_4K_Definitive.7z");
        let game_root = std::path::Path::new(r"C:\Games\GTA San Andreas");
        if sample_7z.exists() && game_root.exists() {
            let app_data_dir = dirs::data_local_dir().unwrap().join("SAMM");
            let db = Database::init(&app_data_dir.join("samm.db")).unwrap();
            let games = db.get_game_installations().unwrap();
            if let Some(game) = games.into_iter().find(|g| g.id == "27652f56-9968-4345-8f63-b9941bff9f8b") {
                let plan = generate_installation_plan(
                    sample_7z,
                    &game,
                    "mixmods_loadscreens_4k",
                    "[SA] Loadscreens 4K Definitive v1.1",
                ).unwrap();

                let backup_dir = app_data_dir.join("backups");
                let res = crate::installation::execute_installation_transaction(
                    sample_7z,
                    &plan,
                    &game,
                    "08fcadbb-dc7f-40d8-9005-83fe6d52505e",
                    &backup_dir,
                    &db,
                ).unwrap();
                println!("Successfully installed into game: {:?}", res.name);
            }
        }
    }

    #[test]
    fn test_duplicate_mod_detection_and_safe_removal() {
        let temp_dir = tempfile::tempdir().unwrap();
        let game_root = temp_dir.path();
        let db = Database::in_memory().unwrap();

        let game = GameInstallation {
            id: "game_dup_test".to_string(),
            name: "GTA San Andreas Duplicates Test".to_string(),
            root_path: game_root.to_string_lossy().to_string(),
            executable_path: game_root.join("gta_sa.exe").to_string_lossy().to_string(),
            version_confidence: ConfidenceLevel::Confirmed,
            detected_edition: "Standard 1.0".to_string(),
            executable_sha256: "".to_string(),
            executable_size_bytes: 0,
            pe_timestamp: None,
            evidence_list: Vec::new(),
            uncertainties: Vec::new(),
            is_classic_1_0: true,
            is_active: true,
            created_date: "".to_string(),
            last_verified_date: "".to_string(),
        };

        // Create ModLoader CheatMenu folder
        let ml_cheat_dir = game_root.join("modloader").join("Cheat_Menu_Trainer");
        std::fs::create_dir_all(&ml_cheat_dir).unwrap();
        std::fs::write(ml_cheat_dir.join("menu.dat"), b"textures and data").unwrap();

        // Create redundant root CheatMenu ASI plugin
        std::fs::write(game_root.join("CheatMenuSA.asi"), b"binary asi").unwrap();
        std::fs::write(game_root.join("CheatMenuSA.ini"), b"config").unwrap();

        // Scan duplicates
        let groups = crate::duplicates::scan_all_duplicates(&game, &db);
        assert_eq!(groups.len(), 1, "Should detect 1 duplicate group");
        assert_eq!(groups[0].group_id, "cheat_menu");
        assert_eq!(groups[0].items.len(), 2, "Group should contain ModLoader and Root ASI copies");

        let recommended = groups[0].items.iter().find(|i| i.is_recommended_keep).unwrap();
        assert_eq!(recommended.location_type, "modloader");

        let redundant = groups[0].items.iter().find(|i| !i.is_recommended_keep).unwrap();
        assert_eq!(redundant.location_type, "root_asi");

        // Remove all duplicates
        let res = crate::duplicates::remove_all_duplicates(&game, &db).unwrap();
        assert_eq!(res.removed_count, 1);

        // Verify root ASI was removed, and consolidated into ModLoader
        assert!(!game_root.join("CheatMenuSA.asi").exists(), "Root ASI should be removed");
        assert!(ml_cheat_dir.join("CheatMenuSA.asi").exists(), "ASI should be safely consolidated into ModLoader");

        // After resolution, 0 duplicates remain
        let after_groups = crate::duplicates::scan_all_duplicates(&game, &db);
        assert_eq!(after_groups.len(), 0, "No duplicate groups should remain after removal");
    }

    #[test]
    fn test_multiple_modloader_duplicate_copies_keep_invariant() {
        let temp_dir = tempfile::tempdir().unwrap();
        let game_root = temp_dir.path();
        let db = Database::in_memory().unwrap();

        let game = GameInstallation {
            id: "game_dup_multi".to_string(),
            name: "GTA San Andreas Multi-Duplicate Test".to_string(),
            root_path: game_root.to_string_lossy().to_string(),
            executable_path: game_root.join("gta_sa.exe").to_string_lossy().to_string(),
            version_confidence: ConfidenceLevel::Confirmed,
            detected_edition: "Standard 1.0".to_string(),
            executable_sha256: "".to_string(),
            executable_size_bytes: 0,
            pe_timestamp: None,
            evidence_list: Vec::new(),
            uncertainties: Vec::new(),
            is_classic_1_0: true,
            is_active: true,
            created_date: "".to_string(),
            last_verified_date: "".to_string(),
        };

        // Create 3 separate ModLoader copies of Loading Screens
        let ml_dir1 = game_root.join("modloader").join("SA_Loadscreens_4K_Definitive_v11").join("models").join("txd");
        let ml_dir2 = game_root.join("modloader").join("Loadscreens_Backup").join("models").join("txd");
        let ml_dir3 = game_root.join("modloader").join("Alt_Loadscreens").join("models").join("txd");

        std::fs::create_dir_all(&ml_dir1).unwrap();
        std::fs::create_dir_all(&ml_dir2).unwrap();
        std::fs::create_dir_all(&ml_dir3).unwrap();

        // Give copy 1 the largest size (e.g. 500 bytes vs 100 bytes vs 50 bytes)
        std::fs::write(ml_dir1.join("LOADSCS.txd"), vec![0u8; 500]).unwrap();
        std::fs::write(ml_dir2.join("LOADSCS.txd"), vec![0u8; 100]).unwrap();
        std::fs::write(ml_dir3.join("LOADSCS.txd"), vec![0u8; 50]).unwrap();

        // Scan duplicates
        let groups = crate::duplicates::scan_all_duplicates(&game, &db);
        assert_eq!(groups.len(), 1, "Should detect 1 duplicate group");
        assert_eq!(groups[0].group_id, "loadscreens");
        assert_eq!(groups[0].items.len(), 3, "All 3 ModLoader copies must be detected in group");

        // Verify INVARIANT: Exactly 1 item is recommended keep, exactly 2 are redundant duplicates
        let kept_items: Vec<_> = groups[0].items.iter().filter(|i| i.is_recommended_keep).collect();
        let redundant_items: Vec<_> = groups[0].items.iter().filter(|i| !i.is_recommended_keep).collect();

        assert_eq!(kept_items.len(), 1, "Exactly one item must be marked as recommended keep");
        assert_eq!(redundant_items.len(), 2, "All other copies must be marked as redundant duplicates");
        assert_eq!(groups[0].duplicates_to_remove.len(), 2, "duplicates_to_remove must contain exactly the 2 redundant copies");

        // The recommended copy should be the largest one (copy 1)
        assert_eq!(kept_items[0].size_bytes, 500);

        // Remove all duplicates
        let res = crate::duplicates::remove_all_duplicates(&game, &db).unwrap();
        assert_eq!(res.removed_count, 2, "Must remove exactly the 2 redundant copies");

        // Verify kept version is intact
        assert!(game_root.join("modloader").join("SA_Loadscreens_4K_Definitive_v11").join("models").join("txd").join("LOADSCS.txd").exists(), "Kept copy must remain intact");

        // Verify redundant versions were deleted
        assert!(!game_root.join("modloader").join("Loadscreens_Backup").exists(), "Redundant copy 2 must be deleted");
        assert!(!game_root.join("modloader").join("Alt_Loadscreens").exists(), "Redundant copy 3 must be deleted");

        // Clean re-scan confirms zero duplicates remain
        let after = crate::duplicates::scan_all_duplicates(&game, &db);
        assert_eq!(after.len(), 0, "Zero duplicates remain after cleanup");
    }

    #[test]
    fn test_launch_game_pre_checks_and_auto_heal() {
        use std::fs;
        let temp_dir = tempfile::tempdir().unwrap();
        let game_root = temp_dir.path();

        // Set up mock modloader directory structure
        let cheatmenu_data = game_root.join("modloader").join("Cheat_Menu_Ultimate_In-Game_Trainer_GUI").join("CheatMenuSA").join("data");
        fs::create_dir_all(&cheatmenu_data).unwrap();

        let fp_dir = game_root.join("modloader").join("First_Person_Mod_v3_0_FIXED");
        fs::create_dir_all(&fp_dir).unwrap();
        fs::write(fp_dir.join("msvcr100d.dll"), b"debug runtime").unwrap();

        let vhud_dir = game_root.join("modloader").join("GTA_V_Style_High-Definition_Radar__Minimap");
        fs::create_dir_all(&vhud_dir).unwrap();
        fs::write(vhud_dir.join("bass.dll"), vec![0u8; 129_828]).unwrap();

        // Write ancient 92KB bass.dll in root
        fs::write(game_root.join("bass.dll"), vec![0u8; 92_216]).unwrap();

        // Write duplicate Ultimate ASI Loader dinput8.dll
        fs::write(game_root.join("vorbisFile.dll"), b"Ultimate-ASI-Loader main").unwrap();
        fs::write(game_root.join("dinput8.dll"), b"Ultimate-ASI-Loader duplicate").unwrap();

        // Write mock CheatMenuSA.asi in modloader
        fs::write(game_root.join("modloader").join("Cheat_Menu_Ultimate_In-Game_Trainer_GUI").join("CheatMenuSA.asi"), b"mock crash asi").unwrap();

        // Run auto-heal
        super::super::game_launch::launch_game(&GameInstallation {
            id: "test_game".to_string(),
            name: "GTA San Andreas".to_string(),
            executable_path: game_root.join("gta_sa.exe").to_string_lossy().to_string(),
            root_path: game_root.to_string_lossy().to_string(),
            detected_edition: "1.0".to_string(),
            version_confidence: ConfidenceLevel::Confirmed,
            executable_sha256: "".to_string(),
            executable_size_bytes: 0,
            pe_timestamp: None,
            evidence_list: Vec::new(),
            uncertainties: Vec::new(),
            is_classic_1_0: true,
            is_active: true,
            created_date: "".to_string(),
            last_verified_date: "".to_string(),
        }, &["-novid".to_string()]).unwrap_err(); // returns err because gta_sa.exe doesn't exist, but auto-heal runs

        // Verify CheatMenuSA.asi was neutralized to prevent fatal crash
        assert!(game_root.join("modloader").join("Cheat_Menu_Ultimate_In-Game_Trainer_GUI").join("CheatMenuSA.asi.disabled").exists(), "CheatMenuSA.asi must be isolated");

        // Verify msvcr100d.dll was copied to game root
        assert!(game_root.join("msvcr100d.dll").exists(), "msvcr100d.dll must be copied to root");

        // Verify bass.dll was upgraded in game root
        assert_eq!(fs::metadata(game_root.join("bass.dll")).unwrap().len(), 129_828, "bass.dll must be upgraded to 129KB");

        // Verify duplicate dinput8.dll was renamed
        assert!(!game_root.join("dinput8.dll").exists(), "Duplicate dinput8.dll must be removed from active load path");
        assert!(game_root.join("dinput8.dll.duplicate_bak").exists(), "Duplicate dinput8.dll must be backed up");
    }
}

