use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;
use crate::models::{InstalledMod, InstalledFileRecord, GameInstallation};
use crate::database::Database;

pub struct DetectedModInfo {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub category: String,
    pub strategy: String,
    pub source_name: String,
    pub relative_files: Vec<String>,
    pub is_enabled: bool,
}

pub fn scan_existing_mods(root_path: &Path) -> Vec<DetectedModInfo> {
    let mut detected = Vec::new();
    let mut claimed_files: HashSet<String> = HashSet::new();

    if !root_path.is_dir() {
        return detected;
    }

    // 1. ModLoader Directory (modloader/*)
    let modloader_dir = root_path.join("modloader");
    if modloader_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&modloader_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if folder_name.starts_with('.') || folder_name.eq_ignore_ascii_case(".data") {
                        continue;
                    }

                    let is_enabled = !folder_name.ends_with(".disabled");
                    let clean_folder = folder_name.trim_end_matches(".disabled").trim();
                    let lower = clean_folder.to_lowercase();

                    // If this folder is an essentials pack or custom collection containing subdirectories, scan each subfolder as its own mod
                    let mut has_subdirs = false;
                    if lower.contains("essential") || clean_folder.starts_with('_') {
                        if let Ok(sub_entries) = fs::read_dir(&path) {
                            for se in sub_entries.flatten() {
                                if se.path().is_dir() {
                                    has_subdirs = true;
                                    break;
                                }
                            }
                        }
                    }

                    if has_subdirs {
                        if let Ok(sub_entries) = fs::read_dir(&path) {
                            for se in sub_entries.flatten() {
                                let sub_path = se.path();
                                if sub_path.is_dir() {
                                    let sub_folder_name = sub_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    if sub_folder_name.starts_with('.') {
                                        continue;
                                    }
                                    let sub_is_enabled = is_enabled && !sub_folder_name.ends_with(".disabled");
                                    let sub_clean = sub_folder_name.trim_end_matches(".disabled").trim();
                                    let mut sub_rel_files = Vec::new();
                                    collect_files_recursive(&sub_path, root_path, &mut sub_rel_files);
                                    if !sub_rel_files.is_empty() {
                                        let sub_lower = sub_clean.to_lowercase();
                                        let (id, name, author, cat) = if sub_lower.contains("silentpatch") {
                                            ("gtainside_silentpatch".to_string(), "SilentPatch for GTA San Andreas".to_string(), "Silent".to_string(), "Plugins / ASI".to_string())
                                        } else if sub_lower.contains("widescreen") {
                                            ("gtainside_widescreen_fix".to_string(), "Widescreen Fix & Aspect Ratio Correction".to_string(), "ThirteenAG".to_string(), "Plugins / ASI".to_string())
                                        } else if sub_lower.contains("framerate") || sub_lower.contains("vigilante") {
                                            ("mixmods_framerate_vigilante".to_string(), "Framerate Vigilante (60FPS Physics Patch)".to_string(), "Junior_Djjr".to_string(), "Plugins / ASI".to_string())
                                        } else if sub_lower.contains("window") {
                                            ("mixmods_windowed_mode".to_string(), "Windowed Mode Borderless (wndmode)".to_string(), "ThirteenAG".to_string(), "Plugins / ASI".to_string())
                                        } else if sub_lower.contains("repairgta") || sub_lower.contains("repair") {
                                            ("local_ml_repairgta".to_string(), "RepairGTA (Crash Prevention & Memory Fix)".to_string(), "Junior_Djjr".to_string(), "Essential Fixes".to_string())
                                        } else if sub_lower.contains("rundll32") {
                                            ("local_ml_rundll32".to_string(), "RunDLL32 Fix (Windows Explorer Fix)".to_string(), "Junior_Djjr".to_string(), "Essential Fixes".to_string())
                                        } else {
                                            let clean_name = sub_clean.replace('_', " ");
                                            (format!("local_ml_sub_{}", clean_slug(sub_clean)), clean_name, "Local / ModLoader".to_string(), "Essential Fixes".to_string())
                                        };

                                        for f in &sub_rel_files {
                                            claimed_files.insert(f.clone());
                                        }

                                        detected.push(DetectedModInfo {
                                            id,
                                            name,
                                            author,
                                            version: "1.0".to_string(),
                                            category: cat,
                                            strategy: "modloader".to_string(),
                                            source_name: "Pre-installed (ModLoader)".to_string(),
                                            relative_files: sub_rel_files,
                                            is_enabled: sub_is_enabled,
                                        });
                                    }
                                }
                            }
                        }
                        continue;
                    }

                    // Standard single modloader directory
                    let mut rel_files = Vec::new();
                    collect_files_recursive(&path, root_path, &mut rel_files);

                    if !rel_files.is_empty() {
                        let (id, name, author, cat) = if lower.contains("silentpatch") {
                            ("gtainside_silentpatch".to_string(), "SilentPatch for GTA San Andreas".to_string(), "Silent".to_string(), "Plugins / ASI".to_string())
                        } else if lower.contains("widescreen") {
                            ("gtainside_widescreen_fix".to_string(), "Widescreen Fix & Aspect Ratio Correction".to_string(), "ThirteenAG".to_string(), "Plugins / ASI".to_string())
                        } else if lower.contains("skygfx") {
                            ("libertycity_skygfx".to_string(), "SkyGfx 4.2b (PS2 Graphics & Visual Pipeline)".to_string(), "aap (The Hero)".to_string(), "Graphics / Plugins".to_string())
                        } else if lower.contains("2dfx") || lower.contains("project2dfx") {
                            ("gtainside_project2dfx".to_string(), "Project 2DFX (Max Draw Distance & LODs)".to_string(), "ThirteenAG & TJDS".to_string(), "Graphics / Plugins".to_string())
                        } else if lower.contains("cleoplus") || lower.contains("cleo+") {
                            ("mixmods_cleo_plus".to_string(), "CLEO+ Extended Opcodes".to_string(), "Junior_Djjr".to_string(), "Tools / Frameworks".to_string())
                        } else if lower.contains("window") {
                            ("mixmods_windowed_mode".to_string(), "Windowed Mode Borderless (wndmode)".to_string(), "ThirteenAG".to_string(), "Plugins / ASI".to_string())
                        } else if lower.contains("firstperson") || lower.contains("first_person") {
                            ("gtainside_first_person".to_string(), "First Person Mod Fixed".to_string(), "BoPoH".to_string(), "Gameplay / Scripts".to_string())
                        } else if lower.contains("ragdoll") {
                            ("gtainside_ragdoll_physics".to_string(), "Ragdoll Physics Engine Mod".to_string(), "MadMan & Ryosuke".to_string(), "Plugins / ASI".to_string())
                        } else if lower.contains("remastered") && lower.contains("gui") {
                            ("gtainside_remastered_gui".to_string(), "Remastered High Definition GUI & Radar".to_string(), "ThirteenAG".to_string(), "Graphics / Textures".to_string())
                        } else if lower.contains("essential") {
                            ("local_ml_essentials".to_string(), "Essentials Pack (ModLoader)".to_string(), "Various Modders".to_string(), "Essential Fixes".to_string())
                        } else if lower.contains("realistic") && lower.contains("vehicle") {
                            ("local_ml_realistic_vehicles".to_string(), "Realistic Vehicles Pack".to_string(), "Community".to_string(), "Vehicles / Cars".to_string())
                        } else {
                            let clean_name = clean_folder.replace('_', " ");
                            (format!("local_ml_{}", clean_slug(clean_folder)), clean_name, "Local / ModLoader".to_string(), "ModLoader Assets".to_string())
                        };

                        for f in &rel_files {
                            claimed_files.insert(f.clone());
                        }

                        detected.push(DetectedModInfo {
                            id,
                            name,
                            author,
                            version: "1.0".to_string(),
                            category: cat,
                            strategy: "modloader".to_string(),
                            source_name: "Pre-installed (ModLoader)".to_string(),
                            relative_files: rel_files,
                            is_enabled,
                        });
                    }
                }
            }
        }
    }

    // 2. Root ASI Plugins (*.asi, *.asi.disabled)
    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let is_disabled = file_name.ends_with(".asi.disabled");
                let is_asi = file_name.ends_with(".asi") || is_disabled;
                if is_asi {
                    let is_enabled = !is_disabled;
                    let lower = file_name.to_lowercase();
                    let clean_stem = if is_disabled {
                        file_name.trim_end_matches(".disabled").trim_end_matches(".asi").to_string()
                    } else {
                        path.file_stem().unwrap_or_default().to_string_lossy().to_string()
                    };

                    let mut rel_files = vec![file_name.clone()];
                    claimed_files.insert(file_name.clone());

                    // Look for companion files (.ini, .toml, .cfg, .dat, .log, .sp, .fp, .dll)
                    for ext_test in ["ini", "toml", "cfg", "dat", "log", "sp", "fp", "dll"] {
                        let comp_name = format!("{}.{}", clean_stem, ext_test);
                        let comp_dis_name = format!("{}.{}.disabled", clean_stem, ext_test);
                        if root_path.join(&comp_name).is_file() && !rel_files.contains(&comp_name) {
                            rel_files.push(comp_name.clone());
                            claimed_files.insert(comp_name);
                        } else if root_path.join(&comp_dis_name).is_file() && !rel_files.contains(&comp_dis_name) {
                            rel_files.push(comp_dis_name.clone());
                            claimed_files.insert(comp_dis_name);
                        }
                    }

                    // Special companion files for SkyGfx (skygfx1.ini, skygfx2.ini, skygfx3.ini)
                    if lower.contains("skygfx") {
                        for idx in 1..=4 {
                            let ini_name = format!("skygfx{}.ini", idx);
                            if root_path.join(&ini_name).is_file() && !rel_files.contains(&ini_name) {
                                rel_files.push(ini_name.clone());
                                claimed_files.insert(ini_name);
                            }
                        }
                    }

                    // Look for matching companion directory in root (e.g. CheatMenuSA/)
                    let companion_dir = root_path.join(&clean_stem);
                    if companion_dir.is_dir() {
                        collect_files_recursive(&companion_dir, root_path, &mut rel_files);
                        for f in &rel_files {
                            claimed_files.insert(f.clone());
                        }
                    }

                    let (id, name, author, cat) = if lower.contains("modloader") {
                        ("mixmods_modloader".to_string(), "ModLoader (Dynamic Asset Injector)".to_string(), "LINK/2012".to_string(), "Tools / Frameworks".to_string())
                    } else if lower.contains("rainbomizer") || lower.contains("randomizer") || lower.contains("randomiser") {
                        ("github_rainbomizer".to_string(), "San Andreas Rainbomizer (Chaos & Randomizer Mod)".to_string(), "Parik27 & 123robot".to_string(), "Gameplay / Scripts".to_string())
                    } else if lower.contains("crashinfo") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "CrashInfo.SA Crash Logger & Analyzer".to_string(), "Junior_Djjr".to_string(), "Essential Fixes".to_string())
                    } else if lower.contains("firstperson") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "First Person Mod Fixed (Root ASI)".to_string(), "BoPoH".to_string(), "Gameplay / Scripts".to_string())
                    } else if lower.contains("gps") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "SA.GPS.REDUX Radar & Map Route".to_string(), "Junior_Djjr".to_string(), "HUD / Radar".to_string())
                    } else if lower.contains("nodep") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "NoDEP Data Execution Prevention Disabler".to_string(), "Silent".to_string(), "Essential Fixes".to_string())
                    } else if lower.contains("silentpatch") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "SilentPatch for GTA San Andreas (Root ASI)".to_string(), "Silent".to_string(), "Plugins / ASI".to_string())
                    } else if lower.contains("widescreen") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Widescreen Fix & Aspect Ratio Correction (Root ASI)".to_string(), "ThirteenAG".to_string(), "Plugins / ASI".to_string())
                    } else if lower.contains("skygfx") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "SkyGfx 4.2b (PS2 Graphics & Visual Pipeline - Root ASI)".to_string(), "aap (The Hero)".to_string(), "Graphics / Plugins".to_string())
                    } else if lower.contains("salodlights") || lower.contains("project2dfx") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Project 2DFX / SALodLights (Root ASI)".to_string(), "ThirteenAG & TJDS".to_string(), "Graphics / Plugins".to_string())
                    } else if lower.contains("wndmode") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Windowed Mode Borderless (Root ASI)".to_string(), "ThirteenAG".to_string(), "Plugins / ASI".to_string())
                    } else if lower.contains("framerate") || lower.contains("vigilante") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Framerate Vigilante (60FPS Physics Fix - Root ASI)".to_string(), "Junior_Djjr".to_string(), "Essential Fixes".to_string())
                    } else if lower.contains("cheatmenu") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Cheat Menu Ultimate GUI (Root ASI)".to_string(), "CheatMenu Team".to_string(), "Gameplay / Scripts".to_string())
                    } else if lower.contains("ginput") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "GInput (Controller Support)".to_string(), "Silent".to_string(), "Plugins / ASI".to_string())
                    } else if lower.contains("cleo") {
                        ("libertycity_cleo4".to_string(), "CLEO 4.4.4 Scripting Library".to_string(), "Seemann & Alien".to_string(), "Tools / Frameworks".to_string())
                    } else if lower.contains("fastman92") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Fastman92 Limit Adjuster".to_string(), "fastman92".to_string(), "Tools / Frameworks".to_string())
                    } else if lower.contains("openlimit") || lower.contains("limitadjuster") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Limit Adjuster (Open/III.VC.SA)".to_string(), "LINK/2012 & ThirteenAG".to_string(), "Tools / Frameworks".to_string())
                    } else if lower.contains("imvehft") || lower.contains("ivf") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "Improved Vehicle Features (IVF)".to_string(), "DK22Pac".to_string(), "Plugins / ASI".to_string())
                    } else if lower.contains("vehfuncs") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "VehFuncs (Dynamic Vehicle Features)".to_string(), "Junior_Djjr".to_string(), "Plugins / ASI".to_string())
                    } else if lower.contains("pedfuncs") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "PedFuncs (Pedestrian Variation)".to_string(), "Junior_Djjr".to_string(), "Plugins / ASI".to_string())
                    } else if lower.contains("mixsets") {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), "MixSets (Engine Tweaker)".to_string(), "Junior_Djjr".to_string(), "Plugins / ASI".to_string())
                    } else {
                        (format!("root_asi_{}", clean_slug(&clean_stem)), format!("{} (Root ASI Plugin)", clean_stem), "Local Plugin".to_string(), "Plugins / ASI".to_string())
                    };

                    if id == "github_rainbomizer" {
                        let rainbomizer_dir = root_path.join("rainbomizer");
                        let rainbomizer_dis = root_path.join("rainbomizer.disabled");
                        if rainbomizer_dir.is_dir() {
                            collect_files_recursive(&rainbomizer_dir, root_path, &mut rel_files);
                        } else if rainbomizer_dis.is_dir() {
                            collect_files_recursive(&rainbomizer_dis, root_path, &mut rel_files);
                        }
                        for f in &rel_files {
                            claimed_files.insert(f.clone());
                        }
                    }

                    detected.push(DetectedModInfo {
                        id,
                        name,
                        author,
                        version: "1.0".to_string(),
                        category: cat,
                        strategy: "asi".to_string(),
                        source_name: "Pre-installed (Root ASI)".to_string(),
                        relative_files: rel_files,
                        is_enabled,
                    });
                }
            }
        }
    }

    // 3. Scripts Directory (scripts/*.asi, scripts/*.cs, *.disabled)
    let scripts_dir = root_path.join("scripts");
    if scripts_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&scripts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                    let is_disabled = file_name.ends_with(".asi.disabled") || file_name.ends_with(".cs.disabled");
                    let is_script = ext == "asi" || ext == "cs" || is_disabled;
                    if is_script {
                        let is_enabled = !is_disabled;
                        let rel_path = format!("scripts/{}", file_name);
                        if claimed_files.contains(&rel_path) {
                            continue;
                        }

                        let mut rel_files = vec![rel_path.clone()];
                        claimed_files.insert(rel_path);

                        let ini_name = path.with_extension("ini");
                        if ini_name.is_file() {
                            let ini_rel = format!("scripts/{}", ini_name.file_name().unwrap().to_string_lossy());
                            rel_files.push(ini_rel.clone());
                            claimed_files.insert(ini_rel);
                        }

                        let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        let lower = stem.to_lowercase();

                        let (id, name, author, cat) = if lower.contains("rainbomizer") || lower.contains("randomizer") || lower.contains("randomiser") {
                            ("github_rainbomizer".to_string(), "San Andreas Rainbomizer (Chaos & Randomizer Mod)".to_string(), "Parik27 & 123robot".to_string(), "Gameplay / Scripts".to_string())
                        } else if lower.contains("crashinfo") {
                            ("mixmods_crashinfo".to_string(), "CrashInfo.SA Crash Logger & Analyzer".to_string(), "Junior_Djjr".to_string(), "Essential Fixes".to_string())
                        } else if lower.contains("firstperson") {
                            ("gtainside_first_person".to_string(), "First Person Mod Fixed".to_string(), "BoPoH".to_string(), "Gameplay / Scripts".to_string())
                        } else if lower.contains("gps") {
                            ("mixmods_gps_redux".to_string(), "SA.GPS.REDUX Radar & Map Route".to_string(), "Junior_Djjr".to_string(), "HUD / Radar".to_string())
                        } else if lower.contains("nodep") {
                            ("mixmods_nodep".to_string(), "NoDEP Data Execution Prevention Disabler".to_string(), "Silent".to_string(), "Essential Fixes".to_string())
                        } else if lower.contains("silentpatch") {
                            ("gtainside_silentpatch".to_string(), "SilentPatch for GTA San Andreas".to_string(), "Silent".to_string(), "Plugins / ASI".to_string())
                        } else if lower.contains("skygfx") {
                            ("libertycity_skygfx".to_string(), "SkyGfx 4.2b (PS2 Graphics & Visual Pipeline)".to_string(), "aap (The Hero)".to_string(), "Graphics / Plugins".to_string())
                        } else if lower.contains("widescreen") {
                            ("gtainside_widescreen_fix".to_string(), "Widescreen Fix & Aspect Ratio Correction".to_string(), "ThirteenAG".to_string(), "Plugins / ASI".to_string())
                        } else if lower.contains("2dfx") || lower.contains("project2dfx") {
                            ("gtainside_project2dfx".to_string(), "Project 2DFX (Max Draw Distance & LODs)".to_string(), "ThirteenAG & TJDS".to_string(), "Graphics / Plugins".to_string())
                        } else {
                            (format!("local_script_{}", clean_slug(&stem)), format!("{} (Scripts Plugin)", stem), "Local Plugin".to_string(), "Plugins / ASI".to_string())
                        };

                        if id == "github_rainbomizer" {
                            let rainbomizer_dir = root_path.join("rainbomizer");
                            let rainbomizer_dis = root_path.join("rainbomizer.disabled");
                            if rainbomizer_dir.is_dir() {
                                collect_files_recursive(&rainbomizer_dir, root_path, &mut rel_files);
                            } else if rainbomizer_dis.is_dir() {
                                collect_files_recursive(&rainbomizer_dis, root_path, &mut rel_files);
                            }
                            for f in &rel_files {
                                claimed_files.insert(f.clone());
                            }
                        }

                        let is_cs = ext == "cs" || file_name.ends_with(".cs.disabled");
                        detected.push(DetectedModInfo {
                            id,
                            name,
                            author,
                            version: "1.0".to_string(),
                            category: cat,
                            strategy: if is_cs { "cleo".to_string() } else { "asi".to_string() },
                            source_name: "Pre-installed (Scripts)".to_string(),
                            relative_files: rel_files,
                            is_enabled,
                        });
                    }
                }
            }
        }
    }

    // 4. CLEO Directory (cleo/*.cs, cleo/*.cleo, *.disabled)
    let cleo_dir = root_path.join("cleo");
    if cleo_dir.is_dir() {
        // Register CLEO 4 library framework if not already added
        if !detected.iter().any(|d| d.id == "libertycity_cleo4") {
            detected.push(DetectedModInfo {
                id: "libertycity_cleo4".to_string(),
                name: "CLEO 4.4.4 Scripting Library".to_string(),
                author: "Seemann & Alien".to_string(),
                version: "4.4.4".to_string(),
                category: "Tools / Frameworks".to_string(),
                strategy: "cleo".to_string(),
                source_name: "Pre-installed (CLEO)".to_string(),
                relative_files: vec!["cleo".to_string()],
                is_enabled: true,
            });
        }

        // Check cleo/CLEO+.cleo
        if cleo_dir.join("CLEO+.cleo").is_file() || cleo_dir.join("cleo+.cleo").is_file() {
            detected.push(DetectedModInfo {
                id: "mixmods_cleo_plus".to_string(),
                name: "CLEO+ Extended Opcodes".to_string(),
                author: "Junior_Djjr".to_string(),
                version: "1.2.0".to_string(),
                category: "Tools / Frameworks".to_string(),
                strategy: "cleo".to_string(),
                source_name: "Pre-installed (CLEO)".to_string(),
                relative_files: vec!["cleo/CLEO+.cleo".to_string()],
                is_enabled: true,
            });
            claimed_files.insert("cleo/CLEO+.cleo".to_string());
        }

        // Scan .cs scripts
        if let Ok(entries) = fs::read_dir(&cleo_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                    let is_disabled = file_name.ends_with(".cs.disabled");
                    let is_cleo = ext == "cs" || is_disabled;
                    if is_cleo {
                        let is_enabled = !is_disabled;
                        let rel_path = format!("cleo/{}", file_name);
                        let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        let clean_stem = if is_disabled {
                            stem.trim_end_matches(".cs").to_string()
                        } else {
                            stem
                        };
                        let lower = clean_stem.to_lowercase();

                        let mut rel_files = vec![rel_path.clone()];
                        claimed_files.insert(rel_path);

                        // Look for .fxt or .ini
                        let ini_path = cleo_dir.join(format!("{}.ini", clean_stem));
                        if ini_path.is_file() {
                            let ini_rel = format!("cleo/{}.ini", clean_stem);
                            rel_files.push(ini_rel.clone());
                            claimed_files.insert(ini_rel);
                        }

                        let fxt_path = cleo_dir.join("cleo_text").join(format!("{}.fxt", clean_stem));
                        if fxt_path.is_file() {
                            let fxt_rel = format!("cleo/cleo_text/{}.fxt", clean_stem);
                            rel_files.push(fxt_rel.clone());
                            claimed_files.insert(fxt_rel);
                        }

                        let (id, name, author) = if lower.contains("firstperson") || lower.contains("first_person") {
                            ("gtainside_first_person".to_string(), "First Person Mod v3.0".to_string(), "BoPoH".to_string())
                        } else if lower.contains("quicksave") {
                            ("local_quicksave".to_string(), "Quicksave Script".to_string(), "ThirteenAG".to_string())
                        } else {
                            (format!("local_cleo_{}", clean_slug(&clean_stem)), format!("{} (CLEO Script)", clean_stem), "Local Script".to_string())
                        };

                        detected.push(DetectedModInfo {
                            id,
                            name,
                            author,
                            version: "1.0".to_string(),
                            category: "Scripts / CLEO".to_string(),
                            strategy: "cleo".to_string(),
                            source_name: "Pre-installed (CLEO)".to_string(),
                            relative_files: rel_files,
                            is_enabled,
                        });
                    }
                }
            }
        }
    }

    // 5. Standalone ASI Loader Hook
    let has_vorbis = root_path.join("vorbisFile.dll").is_file() || root_path.join("vorbisHooked.dll").is_file();
    if has_vorbis && !detected.iter().any(|d| d.id == "local_asi_loader") {
        detected.push(DetectedModInfo {
            id: "local_asi_loader".to_string(),
            name: "Silent's ASI Loader / Vorbis Hook".to_string(),
            author: "Silent & JernejL".to_string(),
            version: "1.3".to_string(),
            category: "Tools / Frameworks".to_string(),
            strategy: "asi".to_string(),
            source_name: "Pre-installed (Loader)".to_string(),
            relative_files: vec!["vorbisFile.dll".to_string()],
            is_enabled: true,
        });
    }

    // 6. ENBSeries
    if root_path.join("enbseries.ini").is_file() {
        detected.push(DetectedModInfo {
            id: "local_enbseries".to_string(),
            name: "ENBSeries Graphics Enhancement".to_string(),
            author: "Boris Vorontsov".to_string(),
            version: "0.076".to_string(),
            category: "Graphics / Plugins".to_string(),
            strategy: "asi".to_string(),
            source_name: "Pre-installed (ENB)".to_string(),
            relative_files: vec!["enbseries.ini".to_string()],
            is_enabled: true,
        });
    }

    // 7. Standalone Rainbomizer Directory (active or disabled)
    let rainbomizer_dir = root_path.join("rainbomizer");
    let rainbomizer_dis = root_path.join("rainbomizer.disabled");
    if (rainbomizer_dir.is_dir() || rainbomizer_dis.is_dir()) && !detected.iter().any(|d| d.id == "github_rainbomizer") {
        let is_disabled = !rainbomizer_dir.is_dir() && rainbomizer_dis.is_dir();
        let target_dir = if is_disabled { &rainbomizer_dis } else { &rainbomizer_dir };
        let mut rel_files = Vec::new();
        collect_files_recursive(target_dir, root_path, &mut rel_files);
        for f in &rel_files {
            claimed_files.insert(f.clone());
        }
        detected.push(DetectedModInfo {
            id: "github_rainbomizer".to_string(),
            name: "San Andreas Rainbomizer (Chaos & Randomizer Mod)".to_string(),
            author: "Parik27 & 123robot".to_string(),
            version: "3.3.1".to_string(),
            category: "Gameplay / Scripts".to_string(),
            strategy: "asi".to_string(),
            source_name: "Pre-installed (Rainbomizer)".to_string(),
            relative_files: rel_files,
            is_enabled: !is_disabled,
        });
    }

    // 8. Loose Root Mod Folders (non-vanilla directories such as [mods]-*, atp, etc.)
    const VANILLA_ROOT_DIRS: &[&str] = &[
        "anim", "audio", "data", "models", "movies", "text", "readme", "samp",
        "modloader", "cleo", "scripts", "$recycle.bin", "system volume information",
    ];

    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let lower = folder_name.to_lowercase();
                if lower.starts_with('.') || lower.contains("readme") || lower.contains("dependencies") || VANILLA_ROOT_DIRS.contains(&lower.as_str()) {
                    continue;
                }

                let mut folder_files = Vec::new();
                collect_files_recursive(&path, root_path, &mut folder_files);
                if folder_files.is_empty() || folder_files.iter().all(|f| claimed_files.contains(f)) {
                    continue;
                }

                let is_enabled = !folder_name.ends_with(".disabled");
                let clean_name = folder_name.trim_end_matches(".disabled");
                let clean_lower = clean_name.to_lowercase();

                let (id, name, cat) = if clean_lower.contains("tsunami") {
                    ("local_loose_tsunami".to_string(), "Tsunami Water Mod (Loose Folder)".to_string(), "Environment / Weather".to_string())
                } else if clean_lower.contains("atp") {
                    ("local_loose_atp".to_string(), "Animated Traffic Police (ATP)".to_string(), "Scripts / Plugins".to_string())
                } else if clean_lower.contains("cheatmenu") {
                    ("local_loose_cheat_menu".to_string(), "Cheat Menu Assets (Loose Folder)".to_string(), "Gameplay / Scripts".to_string())
                } else {
                    let display_title = clean_name
                        .replace(['[', ']', '-', '_'], " ");
                    let title = display_title.split_whitespace().collect::<Vec<_>>().join(" ");
                    (
                        format!("local_loose_{}", clean_slug(clean_name)),
                        format!("{} (Loose Mod Folder)", title),
                        "Custom / Loose Mods".to_string(),
                    )
                };

                for f in &folder_files {
                    claimed_files.insert(f.clone());
                }

                detected.push(DetectedModInfo {
                    id,
                    name,
                    author: "Local Folder".to_string(),
                    version: "1.0".to_string(),
                    category: cat,
                    strategy: "root_loose".to_string(),
                    source_name: "Pre-installed (Loose Root Folder)".to_string(),
                    relative_files: folder_files,
                    is_enabled,
                });
            }
        }
    }

    detected
}

pub fn sync_existing_mods_into_db(
    game: &GameInstallation,
    profile_id: &str,
    db: &Database,
) -> Result<Vec<InstalledMod>, String> {
    let root_path = Path::new(&game.root_path);
    let detected_list = scan_existing_mods(root_path);

    let existing_installed = db.get_installed_mods(&game.id).unwrap_or_default();
    let mut existing_ids: HashSet<String> = existing_installed.iter().map(|m| m.id.clone()).collect();

    // Migrate any legacy unpolished IDs so they get updated to proper catalog IDs with complete file tracking
    let legacy_mappings = [
        ("local_script_sarainbomizer", "github_rainbomizer"),
        ("local_asi_crashinfosa", "mixmods_crashinfo"),
        ("local_asi_sagpsredux", "mixmods_gps_redux"),
        ("local_asi__nodep", "mixmods_nodep"),
        ("local_asi_firstpersonmodfix", "gtainside_first_person"),
        ("local_ml__essentials", "local_ml_essentials"),
    ];
    for (old_id, _new_id) in &legacy_mappings {
        if existing_ids.contains(*old_id) {
            let _ = db.delete_installed_mod(old_id);
            existing_ids.remove(*old_id);
        }
    }

    // Purge legacy monolithic essentials pack so unbundled individual mods are populated
    if detected_list.iter().any(|d| d.id == "mixmods_framerate_vigilante" || d.id == "gtainside_silentpatch" || d.id == "mixmods_windowed_mode") {
        let _ = db.delete_installed_mod("local_ml_essentials");
        existing_ids.remove("local_ml_essentials");
    }

    // Deduplicate any local_ml_ entries that duplicate catalog mods in the same modloader folder
    for m in &existing_installed {
        if m.id.starts_with("local_ml_") {
            let shares_folder_with_catalog = existing_installed.iter().any(|other| {
                !other.id.starts_with("local_ml_") && other.id != m.id && other.files.iter().any(|of| {
                    m.files.iter().any(|mf| {
                        let mf_clean = mf.destination_rel_path.trim_end_matches(".disabled").replace('\\', "/");
                        let of_clean = of.destination_rel_path.trim_end_matches(".disabled").replace('\\', "/");
                        mf_clean.eq_ignore_ascii_case(&of_clean)
                    })
                })
            });
            if shares_folder_with_catalog {
                let _ = db.delete_installed_mod(&m.id);
                existing_ids.remove(&m.id);
            }
        }
    }

    // Clean up ghost preinstalled mods whose files and directories no longer exist on disk
    for m in &existing_installed {
        let is_preinstalled = m.source_name.contains("Pre-installed") || m.source_name.contains("Detected");
        if is_preinstalled && !m.files.is_empty() {
            let dedicated_dirs = crate::installation::get_mod_dedicated_directories(m, root_path);
            let any_dir_exists = dedicated_dirs.iter().any(|d| {
                let d_str = d.to_string_lossy().to_string();
                let clean = PathBuf::from(d_str.trim_end_matches(".disabled"));
                let dis = PathBuf::from(format!("{}.disabled", d_str.trim_end_matches(".disabled")));
                clean.exists() || dis.exists()
            });

            let any_file_exists = m.files.iter().any(|f| {
                let rel = f.destination_rel_path.replace('\\', "/");
                let rel_clean = rel.trim_end_matches(".disabled");
                let p = root_path.join(rel_clean);
                let p_dis = root_path.join(format!("{}.disabled", rel_clean));
                p.exists() || p_dis.exists()
            });

            if !any_dir_exists && !any_file_exists {
                let _ = db.delete_installed_mod(&m.id);
                existing_ids.remove(&m.id);
            }
        }
    }

    let mut synced_mods = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();

    for info in detected_list {
        if existing_ids.contains(&info.id) {
            // Keep DB enabled state in sync with disk state
            if let Ok(mut current_mods) = db.get_installed_mods(&game.id) {
                if let Some(m) = current_mods.iter_mut().find(|m| m.id == info.id) {
                    if m.enabled != info.is_enabled {
                        m.enabled = info.is_enabled;
                        let _ = db.save_installed_mod(m);
                    }
                }
            }
            continue;
        }

        // Avoid creating a duplicate local_ml_ mod if a catalog/managed mod already covers these exact files
        let is_already_owned_by_catalog = existing_installed.iter().any(|m| {
            !m.id.starts_with("local_ml_") && m.files.iter().any(|f| {
                info.relative_files.iter().any(|rf| {
                    let f_clean = f.destination_rel_path.trim_end_matches(".disabled").replace('\\', "/");
                    let rf_clean = rf.trim_end_matches(".disabled").replace('\\', "/");
                    f_clean.eq_ignore_ascii_case(&rf_clean)
                })
            })
        });
        if is_already_owned_by_catalog {
            continue;
        }

        // Build file records
        let mut file_records = Vec::new();
        for rel in &info.relative_files {
            let full = root_path.join(rel);
            let size = if full.is_file() {
                fs::metadata(&full).map(|m| m.len()).unwrap_or(0)
            } else {
                0
            };

            file_records.push(InstalledFileRecord {
                id: uuid::Uuid::new_v4().to_string(),
                installed_mod_id: info.id.clone(),
                destination_rel_path: rel.clone(),
                sha256: String::new(),
                file_size_bytes: size,
                has_backup: false,
                backup_path: None,
            });
        }

        let installed_mod = InstalledMod {
            id: info.id.clone(),
            game_installation_id: game.id.clone(),
            profile_id: profile_id.to_string(),
            name: info.name,
            author: info.author,
            version: info.version,
            source_name: info.source_name,
            source_url: None,
            installed_at: now.clone(),
            strategy: info.strategy,
            enabled: info.is_enabled,
            files: file_records,
            has_conflicts: false,
            conflict_notes: Vec::new(),
        };

        if db.save_installed_mod(&installed_mod).is_ok() {
            // Link to profile
            if let Ok(mut profiles) = db.get_profiles(&game.id) {
                if let Some(p) = profiles.iter_mut().find(|p| p.id == profile_id) {
                    if !p.installed_mod_ids.contains(&installed_mod.id) {
                        p.installed_mod_ids.push(installed_mod.id.clone());
                        let _ = db.save_profile(p);
                    }
                }
            }
            synced_mods.push(installed_mod);
        }
    }

    Ok(synced_mods)
}

fn collect_files_recursive(dir: &Path, root: &Path, acc: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(rel) = path.strip_prefix(root) {
                    acc.push(rel.to_string_lossy().replace('\\', "/"));
                }
            } else if path.is_dir() {
                collect_files_recursive(&path, root, acc);
            }
        }
    }
}

fn clean_slug(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>()
        .to_lowercase()
}

pub fn clean_slug_pub(s: &str) -> String {
    clean_slug(s)
}
