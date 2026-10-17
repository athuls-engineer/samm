use crate::models::Profile;
use crate::database::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileExportManifest {
    pub format_version: String,
    pub profile_name: String,
    pub description: String,
    pub exported_at: String,
    pub mods: Vec<ProfileModExportItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileModExportItem {
    pub name: String,
    pub author: String,
    pub version: String,
    pub strategy: String,
    pub source_name: String,
    pub source_url: Option<String>,
}

pub fn create_default_profiles(game_id: &str, db: &Database) -> Result<Profile, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let default_profile = Profile {
        id: uuid::Uuid::new_v4().to_string(),
        game_installation_id: game_id.to_string(),
        name: "Default (Essential Fixes & Baseline)".to_string(),
        description: "Primary active profile for core enhancements, widescreen fix, and SilentPatch.".to_string(),
        is_default: true,
        is_active: true,
        installed_mod_ids: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    };

    db.save_profile(&default_profile).map_err(|e| e.to_string())?;
    Ok(default_profile)
}

pub fn export_profile_manifest(profile_id: &str, game_id: &str, db: &Database) -> Result<String, String> {
    let profiles = db.get_profiles(game_id).map_err(|e| e.to_string())?;
    let profile = profiles.into_iter().find(|p| p.id == profile_id)
        .ok_or_else(|| format!("Profile '{}' not found", profile_id))?;

    let installed_mods = db.get_installed_mods(game_id).map_err(|e| e.to_string())?;

    let mut mod_items = Vec::new();
    for mod_id in &profile.installed_mod_ids {
        if let Some(m) = installed_mods.iter().find(|im| &im.id == mod_id) {
            mod_items.push(ProfileModExportItem {
                name: m.name.clone(),
                author: m.author.clone(),
                version: m.version.clone(),
                strategy: m.strategy.clone(),
                source_name: m.source_name.clone(),
                source_url: m.source_url.clone(),
            });
        }
    }

    let manifest = ProfileExportManifest {
        format_version: "1.0".to_string(),
        profile_name: profile.name,
        description: profile.description,
        exported_at: chrono::Utc::now().to_rfc3339(),
        mods: mod_items,
    };

    serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())
}

pub fn import_profile_manifest(
    manifest_json: &str,
    game_id: &str,
    db: &Database,
) -> Result<Profile, String> {
    let manifest: ProfileExportManifest = serde_json::from_str(manifest_json)
        .map_err(|e| format!("Invalid profile manifest JSON: {}", e))?;

    let now = chrono::Utc::now().to_rfc3339();
    let new_profile = Profile {
        id: uuid::Uuid::new_v4().to_string(),
        game_installation_id: game_id.to_string(),
        name: format!("{} (Imported)", manifest.profile_name),
        description: manifest.description,
        is_default: false,
        is_active: false,
        installed_mod_ids: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    };

    db.save_profile(&new_profile).map_err(|e| e.to_string())?;
    Ok(new_profile)
}
