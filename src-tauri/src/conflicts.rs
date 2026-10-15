use std::collections::HashMap;
use crate::models::{FileConflict, InstalledMod, PlannedFileOperation};

pub fn detect_installation_conflicts(
    installed_mods: &[InstalledMod],
    proposed_operations: &[PlannedFileOperation],
    proposing_mod_id: &str,
    proposing_mod_name: &str,
) -> Vec<FileConflict> {
    let mut conflicts = Vec::new();

    // Map: destination_rel_path -> (mod_id, mod_name)
    let mut ownership: HashMap<String, (String, String)> = HashMap::new();

    for installed in installed_mods {
        if !installed.enabled {
            continue;
        }
        for file in &installed.files {
            ownership.insert(
                file.destination_rel_path.to_lowercase(),
                (installed.id.clone(), installed.name.clone()),
            );
        }
    }

    for op in proposed_operations {
        let key = op.destination_rel_path.to_lowercase();
        if let Some((owner_id, owner_name)) = ownership.get(&key) {
            if owner_id != proposing_mod_id {
                conflicts.push(FileConflict {
                    id: uuid::Uuid::new_v4().to_string(),
                    destination_rel_path: op.destination_rel_path.clone(),
                    active_mod_id: owner_id.clone(),
                    active_mod_name: owner_name.clone(),
                    conflicting_mod_id: proposing_mod_id.to_string(),
                    conflicting_mod_name: proposing_mod_name.to_string(),
                    status: "detected".to_string(),
                    resolution_notes: Some(format!(
                        "File collision at '{}': active mod '{}' also provides this file.",
                        op.destination_rel_path, owner_name
                    )),
                });
            }
        }
    }

    conflicts
}

pub fn scan_existing_installed_conflicts(installed_mods: &[InstalledMod]) -> Vec<FileConflict> {
    let mut conflicts = Vec::new();
    let mut seen: HashMap<String, (String, String)> = HashMap::new();

    for installed in installed_mods {
        if !installed.enabled {
            continue;
        }
        for file in &installed.files {
            let key = file.destination_rel_path.to_lowercase();
            if let Some((prev_id, prev_name)) = seen.get(&key) {
                if prev_id != &installed.id {
                    conflicts.push(FileConflict {
                        id: uuid::Uuid::new_v4().to_string(),
                        destination_rel_path: file.destination_rel_path.clone(),
                        active_mod_id: prev_id.clone(),
                        active_mod_name: prev_name.clone(),
                        conflicting_mod_id: installed.id.clone(),
                        conflicting_mod_name: installed.name.clone(),
                        status: "detected".to_string(),
                        resolution_notes: Some(format!(
                            "Multiple mods claim '{}': '{}' and '{}'",
                            file.destination_rel_path, prev_name, installed.name
                        )),
                    });
                }
            } else {
                seen.insert(key, (installed.id.clone(), installed.name.clone()));
            }
        }
    }

    conflicts
}
