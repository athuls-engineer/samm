use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::models::{GameInstallation, DuplicateModGroup, DuplicateModItem, DuplicateResolutionResult};
use crate::database::Database;

pub fn normalize_mod_family(raw: &str) -> String {
    let lower = raw.to_lowercase();
    let cleaned = lower
        .replace(['_', '-', '.', '[', ']', '(', ')'], " ");

    if cleaned.contains("cheat") && cleaned.contains("menu") {
        return "cheat_menu".to_string();
    }
    if cleaned.contains("first") && cleaned.contains("person") {
        return "first_person".to_string();
    }
    if cleaned.contains("skygfx") {
        return "skygfx".to_string();
    }
    if cleaned.contains("2dfx") || cleaned.contains("salodlights") || cleaned.contains("project2dfx") {
        return "project_2dfx".to_string();
    }
    if cleaned.contains("silentpatch") {
        return "silentpatch".to_string();
    }
    if cleaned.contains("widescreen") {
        return "widescreen_fix".to_string();
    }
    if cleaned.contains("framerate") || cleaned.contains("vigilante") {
        return "framerate_vigilante".to_string();
    }
    if cleaned.contains("window") || cleaned.contains("wndmode") {
        return "windowed_mode".to_string();
    }
    if cleaned.contains("repairgta") || cleaned.contains("repair") {
        return "repair_gta".to_string();
    }
    if cleaned.contains("rundll32") {
        return "rundll32_fix".to_string();
    }
    if cleaned.contains("remastered") && cleaned.contains("gui") {
        return "remastered_gui".to_string();
    }
    if cleaned.contains("loadscreen") || cleaned.contains("loadscs") {
        return "loadscreens".to_string();
    }
    if cleaned.contains("cleoplus") || cleaned.contains("cleo+") {
        return "cleo_plus".to_string();
    }
    if cleaned.contains("fastman92") || (cleaned.contains("limit") && cleaned.contains("adjuster")) {
        return "limit_adjuster".to_string();
    }
    if cleaned.contains("rainbomizer") || cleaned.contains("randomizer") {
        return "rainbomizer".to_string();
    }
    if cleaned.contains("gps") && cleaned.contains("redux") {
        return "gps_redux".to_string();
    }
    if cleaned.contains("nodep") {
        return "nodep".to_string();
    }
    if cleaned.contains("crashinfo") {
        return "crash_info".to_string();
    }
    if cleaned.contains("customskins") || cleaned.contains("custom_skins") {
        return "custom_skins".to_string();
    }
    if cleaned.contains("tsunami") {
        return "tsunami_water".to_string();
    }
    if cleaned.contains("atp") || (cleaned.contains("traffic") && cleaned.contains("police")) {
        return "atp_police".to_string();
    }
    if cleaned.contains("ginput") {
        return "ginput".to_string();
    }
    if cleaned.contains("ragdoll") {
        return "ragdoll_physics".to_string();
    }

    let tokens: Vec<&str> = cleaned
        .split_whitespace()
        .filter(|w| {
            w.len() > 2
                && *w != "the"
                && *w != "for"
                && *w != "and"
                && *w != "mod"
                && *w != "fix"
                && *w != "asi"
                && *w != "gta"
                && *w != "san"
                && *w != "andreas"
                && *w != "plugin"
                && *w != "script"
        })
        .collect();

    if tokens.is_empty() {
        cleaned.trim().replace(' ', "_")
    } else {
        tokens[0].to_string()
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes > 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes > 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes > 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn scan_all_duplicates(game: &GameInstallation, db: &Database) -> Vec<DuplicateModGroup> {
    let root_path = Path::new(&game.root_path);
    if !root_path.is_dir() {
        return Vec::new();
    }

    // Refresh database with live disk scan first
    let profiles = db.get_profiles(&game.id).unwrap_or_default();
    let active_profile_id = profiles.iter().find(|p| p.is_active).map(|p| p.id.clone())
        .unwrap_or_else(|| profiles.first().map(|p| p.id.clone()).unwrap_or_else(|| "default_profile".to_string()));
    let _ = crate::existing_mods::sync_existing_mods_into_db(game, &active_profile_id, db);

    let installed_mods = db.get_installed_mods(&game.id).unwrap_or_default();

    // Map: family_key -> Vec<DuplicateModItem>
    let mut groups_map: HashMap<String, Vec<DuplicateModItem>> = HashMap::new();
    let mut group_titles: HashMap<String, String> = HashMap::new();

    // 1. Process all registered installed mods
    for m in &installed_mods {
        let is_ml = m.strategy == "modloader" || m.files.iter().any(|f| f.destination_rel_path.starts_with("modloader"));
        let location_type = if is_ml {
            "modloader".to_string()
        } else if m.strategy == "cleo" {
            "cleo".to_string()
        } else if m.strategy == "asi" {
            "root_asi".to_string()
        } else {
            "loose_folder".to_string()
        };

        let primary_path = if !m.files.is_empty() {
            m.files[0].destination_rel_path.clone()
        } else {
            m.name.clone()
        };

        let size_bytes: u64 = m.files.iter().map(|f| f.file_size_bytes).sum();
        let family = normalize_mod_family(&format!("{} {}", m.name, primary_path));

        let friendly_title = match family.as_str() {
            "cheat_menu" => "Cheat Menu Ultimate (In-Game Trainer)".to_string(),
            "first_person" => "First Person Mod Fixed".to_string(),
            "skygfx" => "SkyGfx 4.2b Graphics Pipeline".to_string(),
            "project_2dfx" => "Project 2DFX & SALodLights".to_string(),
            "silentpatch" => "SilentPatch for GTA San Andreas".to_string(),
            "widescreen_fix" => "Widescreen Fix & Aspect Ratio".to_string(),
            "framerate_vigilante" => "Framerate Vigilante (60FPS Fix)".to_string(),
            "windowed_mode" => "Windowed Mode Borderless (wndmode)".to_string(),
            "repair_gta" => "RepairGTA Memory Fix".to_string(),
            "rundll32_fix" => "RunDLL32 Explorer Fix".to_string(),
            "loadscreens" => "4K Definitive Loading Screens".to_string(),
            "tsunami_water" => "Tsunami Water Mod".to_string(),
            "atp_police" => "Animated Traffic Police (ATP)".to_string(),
            "ginput" => "GInput (Controller Support)".to_string(),
            "ragdoll_physics" => "Ragdoll Physics Engine".to_string(),
            "limit_adjuster" => "Fastman92 / Open Limit Adjuster".to_string(),
            _ => m.name.clone(),
        };

        group_titles.entry(family.clone()).or_insert(friendly_title);

        let item = DuplicateModItem {
            id: m.id.clone(),
            name: m.name.clone(),
            location_type,
            primary_path,
            file_count: m.files.len(),
            size_bytes,
            size_formatted: format_bytes(size_bytes),
            is_active: m.enabled,
            is_recommended_keep: is_ml, // ModLoader folders are cleanly isolated and preferred
        };

        groups_map.entry(family).or_default().push(item);
    }

    // 2. Direct filesystem scan for unmanaged loose Root ASIs conflicting with ModLoader
    let modloader_dir = root_path.join("modloader");
    if modloader_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    let fname = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let lower = fname.to_lowercase();
                    if lower.ends_with(".asi") || lower.ends_with(".asi.disabled") {
                        let family = normalize_mod_family(&fname);

                        // If we already have a ModLoader copy of this family, verify if this root file is registered
                        if let Some(existing_items) = groups_map.get_mut(&family) {
                            let already_in_group = existing_items.iter().any(|it| it.primary_path.eq_ignore_ascii_case(&fname));
                            if !already_in_group {
                                let size_bytes = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                                let clean_stem = fname.trim_end_matches(".disabled");
                                let is_active = !fname.ends_with(".disabled");

                                existing_items.push(DuplicateModItem {
                                    id: format!("root_loose_{}", crate::existing_mods::clean_slug_pub(&fname)),
                                    name: format!("{} (Root Game Directory)", clean_stem),
                                    location_type: "root_asi".to_string(),
                                    primary_path: fname.clone(),
                                    file_count: 1,
                                    size_bytes,
                                    size_formatted: format_bytes(size_bytes),
                                    is_active,
                                    is_recommended_keep: false, // Root copy is redundant
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Assemble filtered list of duplicate groups (groups with >= 2 items)
    let mut duplicate_groups = Vec::new();

    for (family, mut items) in groups_map {
        if items.len() < 2 {
            continue;
        }

        // Sort items so preferred copy is first:
        // 1. ModLoader over non-ModLoader (isolated directory over root placement)
        // 2. Active over disabled
        // 3. Highest file count
        // 4. Largest size
        items.sort_by(|a, b| {
            let a_is_ml = a.location_type == "modloader";
            let b_is_ml = b.location_type == "modloader";
            if a_is_ml != b_is_ml {
                return b_is_ml.cmp(&a_is_ml);
            }
            if a.is_active != b.is_active {
                return b.is_active.cmp(&a.is_active);
            }
            if a.file_count != b.file_count {
                return b.file_count.cmp(&a.file_count);
            }
            b.size_bytes.cmp(&a.size_bytes)
        });

        // INVARIANT: Exactly ONE item is marked recommended keep (idx == 0).
        // ALL other items in the group are marked redundant duplicates (is_recommended_keep = false).
        for (idx, item) in items.iter_mut().enumerate() {
            item.is_recommended_keep = idx == 0;
        }

        let recommended_keep_id = items[0].id.clone();
        let duplicates_to_remove: Vec<String> = items.iter().skip(1).map(|i| i.id.clone()).collect();

        let mod_title = group_titles.get(&family).cloned().unwrap_or_else(|| family.clone());

        let reason = if items.iter().any(|i| i.location_type == "root_asi") && items.iter().any(|i| i.location_type == "modloader") {
            "Duplicate installation detected across root and ModLoader. The root plugin bypasses mod isolation and collides with ModLoader assets, causing UI textures or menus to fail to load.".to_string()
        } else {
            format!(
                "Multiple overlapping copies of '{}' exist in your GTA SA installation. Running duplicate versions wastes memory and creates file hook collisions.",
                mod_title
            )
        };

        duplicate_groups.push(DuplicateModGroup {
            group_id: family,
            mod_title,
            reason,
            recommended_keep_id,
            duplicates_to_remove,
            items,
        });
    }

    duplicate_groups.sort_by(|a, b| a.mod_title.cmp(&b.mod_title));
    duplicate_groups
}

pub fn remove_duplicate_mod(
    game: &GameInstallation,
    target_id_or_path: &str,
    db: &Database,
) -> Result<String, String> {
    let root_path = Path::new(&game.root_path);
    if !root_path.is_dir() {
        return Err("Game root directory does not exist".to_string());
    }

    let mut removed_desc = String::new();

    // 1. Check if target is a registered InstalledMod in DB
    if let Ok(installed_mods) = db.get_installed_mods(&game.id) {
        if let Some(m) = installed_mods.iter().find(|m| m.id == target_id_or_path) {
            let family = normalize_mod_family(&format!("{} {}", m.name, target_id_or_path));
            let modloader_dir = root_path.join("modloader");

            // If this is a root ASI or loose mod, check if a companion ModLoader directory lacks the ASI
            if modloader_dir.is_dir() {
                for f in &m.files {
                    let rel_clean = f.destination_rel_path.trim_end_matches(".disabled");
                    if rel_clean.ends_with(".asi") {
                        let full_src = root_path.join(&f.destination_rel_path);
                        let file_name = full_src.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let clean_asi_name = file_name.trim_end_matches(".disabled");

                        if let Ok(entries) = fs::read_dir(&modloader_dir) {
                            for entry in entries.flatten() {
                                let dir = entry.path();
                                if dir.is_dir() {
                                    let dname = dir.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    if normalize_mod_family(&dname) == family {
                                        let dest_asi = dir.join(clean_asi_name);
                                        if !dest_asi.exists() && full_src.is_file() {
                                            let _ = fs::copy(&full_src, &dest_asi);
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Delete dedicated mod directories and files (safeguarding shared files/directories)
            let other_installed = installed_mods.iter().filter(|o| o.id != m.id).collect::<Vec<_>>();
            let dedicated_dirs = crate::installation::get_mod_dedicated_directories(m, root_path);
            for d in dedicated_dirs {
                let is_dir_shared = other_installed.iter().any(|other| {
                    other.files.iter().any(|f| {
                        let f_path = root_path.join(&f.destination_rel_path);
                        f_path.starts_with(&d)
                    })
                });
                if !is_dir_shared {
                    if d.is_dir() {
                        let _ = fs::remove_dir_all(&d);
                    }
                    let dis = PathBuf::from(format!("{}.disabled", d.to_string_lossy().trim_end_matches(".disabled")));
                    if dis.is_dir() {
                        let _ = fs::remove_dir_all(&dis);
                    }
                }
            }

            for f in &m.files {
                let p = root_path.join(&f.destination_rel_path);
                let is_file_shared = other_installed.iter().any(|other| {
                    other.files.iter().any(|of| of.destination_rel_path.eq_ignore_ascii_case(&f.destination_rel_path))
                });
                if !is_file_shared {
                    if p.is_file() {
                        let _ = fs::remove_file(&p);
                    }
                    let p_dis = PathBuf::from(format!("{}.disabled", p.to_string_lossy().trim_end_matches(".disabled")));
                    if p_dis.is_file() {
                        let _ = fs::remove_file(&p_dis);
                    }
                }
            }

            // If loose folder, clean up root folder if now empty or completely deleted
            if m.strategy == "root_loose" {
                for f in &m.files {
                    if let Some(first_segment) = Path::new(&f.destination_rel_path).iter().next() {
                        let top_dir = root_path.join(first_segment);
                        if top_dir.is_dir() {
                            let is_empty = fs::read_dir(&top_dir).map(|mut it| it.next().is_none()).unwrap_or(false);
                            if is_empty {
                                let _ = fs::remove_dir_all(&top_dir);
                            }
                        }
                    }
                }
            }

            let _ = db.delete_installed_mod(&m.id);
            removed_desc = format!("Removed duplicate mod '{}' ({})", m.name, m.id);
        }
    }

    // 2. Check if target is a physical root file or path
    let candidate_path = root_path.join(target_id_or_path.trim_start_matches("root_loose_"));
    if candidate_path.is_file() {
        let fname = candidate_path.file_name().unwrap_or_default().to_string_lossy().to_string();

        // Check if there is a matching ModLoader directory that lacks this ASI, copy it over before deletion
        let family = normalize_mod_family(&fname);
        let modloader_dir = root_path.join("modloader");
        if modloader_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&modloader_dir) {
                for entry in entries.flatten() {
                    let dir = entry.path();
                    if dir.is_dir() {
                        let dname = dir.file_name().unwrap_or_default().to_string_lossy().to_string();
                        if normalize_mod_family(&dname) == family {
                            let dest_asi = dir.join(&fname);
                            if !dest_asi.exists() {
                                let _ = fs::copy(&candidate_path, &dest_asi);
                            }
                            break;
                        }
                    }
                }
            }
        }

        let _ = fs::remove_file(&candidate_path);

        // Also clean up matching .ini
        let ini = candidate_path.with_extension("ini");
        if ini.is_file() {
            let _ = fs::remove_file(&ini);
        }

        removed_desc = format!("Deleted redundant root plugin '{}'", fname);
    } else if target_id_or_path.ends_with(".asi") || target_id_or_path.ends_with(".cs") {
        let p = root_path.join(target_id_or_path);
        if p.is_file() {
            let _ = fs::remove_file(&p);
            removed_desc = format!("Deleted root file '{}'", target_id_or_path);
        }
    }

    if removed_desc.is_empty() {
        // Fallback: search for file in root
        if let Ok(entries) = fs::read_dir(root_path) {
            for entry in entries.flatten() {
                let p = entry.path();
                let fname = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                if target_id_or_path.contains(&crate::existing_mods::clean_slug_pub(&fname)) {
                    let _ = fs::remove_file(&p);
                    removed_desc = format!("Deleted root plugin '{}'", fname);
                    break;
                }
            }
        }
    }

    // Refresh database
    let profiles = db.get_profiles(&game.id).unwrap_or_default();
    let active_profile_id = profiles.iter().find(|p| p.is_active).map(|p| p.id.clone())
        .unwrap_or_else(|| "default_profile".to_string());
    let _ = crate::existing_mods::sync_existing_mods_into_db(game, &active_profile_id, db);

    if removed_desc.is_empty() {
        Ok(format!("Uninstalled duplicate '{}'", target_id_or_path))
    } else {
        Ok(removed_desc)
    }
}

pub fn remove_all_duplicates(
    game: &GameInstallation,
    db: &Database,
) -> Result<DuplicateResolutionResult, String> {
    let groups = scan_all_duplicates(game, db);
    let mut removed_count = 0;
    let mut bytes_freed: u64 = 0;
    let mut removed_items = Vec::new();

    for group in groups {
        for dup_id in group.duplicates_to_remove {
            if let Some(item) = group.items.iter().find(|i| i.id == dup_id) {
                bytes_freed += item.size_bytes;
                removed_items.push(item.name.clone());
            }
            if remove_duplicate_mod(game, &dup_id, db).is_ok() {
                removed_count += 1;
            }
        }
    }

    Ok(DuplicateResolutionResult {
        removed_count,
        bytes_freed,
        removed_items,
    })
}
