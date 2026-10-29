use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};
use crate::models::{ConfidenceLevel, GameInstallation};

// Verified SHA-256 signatures for GTA: San Andreas Executables
// Sources: GTA modding documentation, game preservation databases, community modloader baselines.
const SHA256_CLASSIC_10_US_HOODLUM: &str = "95503dd8a5cc8438db5d24d204a91eb81ff57beea7cf6b8fdf2ea428dd683645";
const SIZE_CLASSIC_10_US_HOODLUM: u64 = 14_383_616;

const SHA256_CLASSIC_10_US_RETAIL: &str = "170b6131c93a9c6870da3e33cfd9cd785c4943fcf4d42b03ca85a3a1f94d07d1";
#[allow(dead_code)]
const SIZE_CLASSIC_10_US_RETAIL: u64 = 14_383_616;

const SIZE_CLASSIC_10_EU_RETAIL: u64 = 14_387_200;
const SIZE_CLASSIC_101_US: u64 = 14_405_632;
const SIZE_CLASSIC_101_EU: u64 = 15_806_464;

pub fn compute_file_sha256(path: &Path) -> Result<(String, u64), String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut total_bytes = 0u64;

    loop {
        let count = file.read(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;
        if count == 0 {
            break;
        }
        total_bytes += count as u64;
        hasher.update(&buffer[..count]);
    }

    let hash_result = hasher.finalize();
    Ok((hex::encode(hash_result), total_bytes))
}

pub fn inspect_game_installation(target_path: &Path) -> Result<GameInstallation, String> {
    let (root_dir, exe_path) = if target_path.is_file() {
        let parent = target_path.parent().ok_or_else(|| "Invalid file path".to_string())?;
        (parent.to_path_buf(), target_path.to_path_buf())
    } else if target_path.is_dir() {
        // Look for gta_sa.exe, gta-sa.exe, or PlayGTASA.exe
        let candidates = ["gta_sa.exe", "gta-sa.exe", "PlayGTASA.exe", "gta_sa_compact.exe"];
        let mut found_exe = None;
        for c in &candidates {
            let p = target_path.join(c);
            if p.is_file() {
                found_exe = Some(p);
                break;
            }
        }

        let exe = found_exe.unwrap_or_else(|| target_path.join("gta_sa.exe"));
        (target_path.to_path_buf(), exe)
    } else {
        return Err("Path does not exist on disk".to_string());
    };

    let mut evidence = Vec::new();
    let mut uncertainties = Vec::new();

    // Check for Definitive Edition / Unreal Engine indicators
    let is_de = root_dir.join("Gameface").exists() || root_dir.join("Engine").exists();
    if is_de {
        return Ok(GameInstallation {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Grand Theft Auto: San Andreas - The Definitive Edition".to_string(),
            executable_path: exe_path.to_string_lossy().to_string(),
            root_path: root_dir.to_string_lossy().to_string(),
            detected_edition: "Definitive Edition (Unreal Engine)".to_string(),
            version_confidence: ConfidenceLevel::Incompatible,
            executable_sha256: String::new(),
            executable_size_bytes: 0,
            pe_timestamp: None,
            evidence_list: vec![
                "Found Unreal Engine / Gameface directory structure".to_string(),
                "Incompatible with classic 2004 RenderWare engine mods".to_string(),
            ],
            uncertainties: vec!["Definitive Edition requires Unreal Engine mod tools, not classic PC mods".to_string()],
            is_classic_1_0: false,
            is_active: false,
            created_date: chrono::Utc::now().to_rfc3339(),
            last_verified_date: chrono::Utc::now().to_rfc3339(),
        });
    }

    if !exe_path.exists() {
        return Err(format!("Executable '{}' not found in game directory", exe_path.file_name().and_then(|n| n.to_str()).unwrap_or("gta_sa.exe")));
    }

    let (exe_sha256, exe_size) = compute_file_sha256(&exe_path)?;
    evidence.push(format!("Found executable: {} ({} bytes)", exe_path.file_name().unwrap().to_string_lossy(), exe_size));
    evidence.push(format!("Computed SHA-256: {}", exe_sha256));

    // Inspect RenderWare game data directory structure
    let has_data_dir = root_dir.join("data").is_dir();
    let has_models_dir = root_dir.join("models").is_dir();
    let has_audio_dir = root_dir.join("audio").is_dir();
    let has_anim_dir = root_dir.join("anim").is_dir();

    if has_data_dir {
        evidence.push("Verified core 'data/' directory exists".to_string());
        if root_dir.join("data").join("gta.dat").exists() {
            evidence.push("Verified 'data/gta.dat' exists".to_string());
        } else {
            uncertainties.push("Missing standard 'data/gta.dat'".to_string());
        }
    } else {
        uncertainties.push("Missing core 'data/' directory".to_string());
    }

    if has_models_dir {
        evidence.push("Verified core 'models/' directory exists".to_string());
        if root_dir.join("models").join("gta3.img").exists() {
            evidence.push("Verified 'models/gta3.img' asset archive present".to_string());
        } else {
            uncertainties.push("Missing 'models/gta3.img'".to_string());
        }
    } else {
        uncertainties.push("Missing core 'models/' directory".to_string());
    }

    if has_audio_dir {
        evidence.push("Verified 'audio/' sound directory exists".to_string());
    }

    if has_anim_dir {
        evidence.push("Verified 'anim/' animation directory exists".to_string());
    }

    // Check for Modding Frameworks
    if root_dir.join("cleo").is_dir() {
        evidence.push("CLEO mod framework directory detected".to_string());
    }
    if root_dir.join("modloader").is_dir() {
        evidence.push("ModLoader folder-based framework detected".to_string());
    }
    if root_dir.join("vorbisFile.dll").is_file() || root_dir.join("vorbisHooked.dll").is_file() || root_dir.join("d3d9.dll").is_file() {
        evidence.push("ASI Loader plugin component detected".to_string());
    }

    // Check Steam / Rockstar distribution indicators
    let has_steam_api = root_dir.join("steam_api.dll").exists() || root_dir.join("installscript.vdf").exists();
    if has_steam_api {
        evidence.push("Detected Steam distribution metadata".to_string());
    }

    // Evidence-based Version Classification
    let (edition, confidence, is_classic_10) = if exe_sha256.to_lowercase() == SHA256_CLASSIC_10_US_HOODLUM {
        evidence.push("Cryptographic match: Classic PC 1.0 US (Standard Modding Baseline / Hoodlum NOCD)".to_string());
        ("Classic PC 1.0 (US - Verified 14.3MB)".to_string(), ConfidenceLevel::Confirmed, true)
    } else if exe_sha256.to_lowercase() == SHA256_CLASSIC_10_US_RETAIL {
        evidence.push("Cryptographic match: Classic PC 1.0 US Retail Uncracked".to_string());
        ("Classic PC 1.0 (US Retail)".to_string(), ConfidenceLevel::Confirmed, true)
    } else if exe_size == SIZE_CLASSIC_10_EU_RETAIL {
        evidence.push("File size matches Classic PC 1.0 EU Retail baseline (14,387,200 bytes)".to_string());
        ("Classic PC 1.0 (EU Retail)".to_string(), ConfidenceLevel::Likely, true)
    } else if exe_size == SIZE_CLASSIC_10_US_HOODLUM {
        evidence.push("File size matches 14,383,616 bytes (Classic 1.0 US), but checksum differs (likely patched or custom modified 1.0)".to_string());
        ("Likely Classic PC 1.0 (Custom / Patched 14.3MB)".to_string(), ConfidenceLevel::Likely, true)
    } else if exe_size == SIZE_CLASSIC_101_US || exe_size == SIZE_CLASSIC_101_EU {
        evidence.push("File size matches PC 1.01 official patch baseline".to_string());
        uncertainties.push("Version 1.01 requires downgrading or 1.01-compatible ASI loader for certain legacy plugins".to_string());
        ("Classic PC 1.01 (Official Patch)".to_string(), ConfidenceLevel::Confirmed, false)
    } else if has_steam_api {
        evidence.push("Executable size or metadata indicates Steam release".to_string());
        uncertainties.push("Steam release executable has altered memory offsets; many ASI/memory plugins require 1.0 baseline".to_string());
        ("Steam Distribution (Executable altered)".to_string(), ConfidenceLevel::Ambiguous, false)
    } else if has_data_dir && has_models_dir {
        evidence.push("Game files structure is valid RenderWare GTA San Andreas".to_string());
        uncertainties.push(format!("Unknown executable hash ({}). May be third-party patch, no-CD or translation.", exe_sha256));
        ("Custom / Modified GTA SA Executable".to_string(), ConfidenceLevel::Ambiguous, false)
    } else {
        uncertainties.push("Directory is missing essential GTA San Andreas game files".to_string());
        ("Invalid / Incomplete Game Installation".to_string(), ConfidenceLevel::Invalid, false)
    };

    let name = root_dir.file_name().and_then(|n| n.to_str()).unwrap_or("GTA San Andreas").to_string();

    Ok(GameInstallation {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        executable_path: exe_path.to_string_lossy().to_string(),
        root_path: root_dir.to_string_lossy().to_string(),
        detected_edition: edition,
        version_confidence: confidence,
        executable_sha256: exe_sha256,
        executable_size_bytes: exe_size,
        pe_timestamp: None,
        evidence_list: evidence,
        uncertainties,
        is_classic_1_0: is_classic_10,
        is_active: false,
        created_date: chrono::Utc::now().to_rfc3339(),
        last_verified_date: chrono::Utc::now().to_rfc3339(),
    })
}

pub fn discover_known_installations() -> Vec<GameInstallation> {
    let mut found = Vec::new();
    let candidate_paths = vec![
        // Standard Steam library locations
        PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\common\Grand Theft Auto San Andreas"),
        PathBuf::from(r"C:\Steam\steamapps\common\Grand Theft Auto San Andreas"),
        PathBuf::from(r"D:\SteamLibrary\steamapps\common\Grand Theft Auto San Andreas"),
        PathBuf::from(r"E:\SteamLibrary\steamapps\common\Grand Theft Auto San Andreas"),

        // Standard Rockstar Games Launcher locations
        PathBuf::from(r"C:\Program Files\Rockstar Games\Grand Theft Auto San Andreas"),
        PathBuf::from(r"C:\Program Files (x86)\Rockstar Games\Grand Theft Auto San Andreas"),

        // Classic retail install paths
        PathBuf::from(r"C:\Program Files (x86)\Rockstar Games\GTA San Andreas"),
        PathBuf::from(r"C:\Program Files\Rockstar Games\GTA San Andreas"),
        PathBuf::from(r"C:\Games\GTA San Andreas"),
        PathBuf::from(r"D:\Games\GTA San Andreas"),
    ];

    for p in candidate_paths {
        if p.exists() {
            if let Ok(installation) = inspect_game_installation(&p) {
                if installation.version_confidence != ConfidenceLevel::Invalid {
                    found.push(installation);
                }
            }
        }
    }

    found
}
