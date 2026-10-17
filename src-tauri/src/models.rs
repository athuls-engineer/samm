use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
    Confirmed,
    Likely,
    Ambiguous,
    Incompatible,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInstallation {
    pub id: String,
    pub name: String,
    pub executable_path: String,
    pub root_path: String,
    pub detected_edition: String,
    pub version_confidence: ConfidenceLevel,
    pub executable_sha256: String,
    pub executable_size_bytes: u64,
    pub pe_timestamp: Option<u64>,
    pub evidence_list: Vec<String>,
    pub uncertainties: Vec<String>,
    pub is_classic_1_0: boolean_compat::BoolCompat,
    pub is_active: boolean_compat::BoolCompat,
    pub created_date: String,
    pub last_verified_date: String,
}

pub mod boolean_compat {
    pub type BoolCompat = bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceCapabilities {
    pub can_search: bool,
    pub can_direct_download: bool,
    pub requires_browser_download: bool,
    pub requires_auth: bool,
    pub supports_version_filter: bool,
    pub supports_category_filter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModSource {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub capabilities: SourceCapabilities,
    pub status: String,
    pub attribution: String,
    pub terms_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstallationStrategyType {
    Cleo,
    Asi,
    Modloader,
    Assets,
    Audio,
    Loose,
    Manual,
    Guided,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModRecord {
    pub id: String,
    pub source_id: String,
    pub source_mod_id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub thumbnail_url: Option<String>,
    pub page_url: String,
    pub download_url: Option<String>,
    pub file_size_formatted: Option<String>,
    pub file_size_bytes: Option<u64>,
    pub release_date: Option<String>,
    pub update_date: Option<String>,
    pub supported_game_versions: Vec<String>,
    pub required_dependencies: Vec<String>,
    pub incompatibilities: Vec<String>,
    pub installation_strategy: InstallationStrategyType,
    pub confidence: String,
    pub compatibility_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadJob {
    pub id: String,
    pub mod_id: String,
    pub mod_title: String,
    pub source_id: String,
    pub source_name: String,
    pub url: String,
    pub temp_file_path: String,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub speed_bytes_per_sec: u64,
    pub status: String,
    pub error_message: Option<String>,
    pub expected_sha256: Option<String>,
    pub computed_sha256: Option<String>,
    pub sha256_matched: Option<bool>,
    pub start_time: String,
    pub completed_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveEntry {
    pub path: String,
    pub size: u64,
    pub compressed_size: u64,
    pub crc32: String,
    pub is_directory: bool,
    pub is_suspicious: bool,
    pub suspicion_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveInspection {
    pub archive_path: String,
    pub format: String,
    pub total_entries: usize,
    pub total_uncompressed_size: u64,
    pub compression_ratio: f64,
    pub is_safe: bool,
    pub safety_violations: Vec<String>,
    pub entries: Vec<ArchiveEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedFileOperation {
    pub archive_rel_path: String,
    pub destination_rel_path: String,
    pub destination_absolute_path: String,
    pub file_type: String,
    pub rule: String,
    pub destination_state: String, // "new", "overwrite", "collision"
    pub backup_required: bool,
    pub file_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationPlan {
    pub mod_id: String,
    pub mod_title: String,
    pub installation_type: String, // "automatic", "guided", "manual"
    pub strategy: InstallationStrategyType,
    pub total_files: usize,
    pub new_files_count: usize,
    pub overwrite_files_count: usize,
    pub missing_prerequisites: Vec<String>,
    pub warnings: Vec<String>,
    pub file_operations: Vec<PlannedFileOperation>,
    pub confidence: String,
    pub manual_instructions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledFileRecord {
    pub id: String,
    pub installed_mod_id: String,
    pub destination_rel_path: String,
    pub sha256: String,
    pub file_size_bytes: u64,
    pub has_backup: bool,
    pub backup_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledMod {
    pub id: String,
    pub game_installation_id: String,
    pub profile_id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub source_name: String,
    pub source_url: Option<String>,
    pub installed_at: String,
    pub strategy: String,
    pub enabled: bool,
    pub files: Vec<InstalledFileRecord>,
    pub has_conflicts: bool,
    pub conflict_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub game_installation_id: String,
    pub name: String,
    pub description: String,
    pub is_default: bool,
    pub is_active: bool,
    pub installed_mod_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileConflict {
    pub id: String,
    pub destination_rel_path: String,
    pub active_mod_id: String,
    pub active_mod_name: String,
    pub conflicting_mod_id: String,
    pub conflicting_mod_name: String,
    pub status: String,
    pub resolution_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictResolutionResult {
    pub resolved_count: usize,
    pub actions_taken: Vec<String>,
    pub remaining_conflicts: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupRecord {
    pub id: String,
    pub game_installation_id: String,
    pub mod_id: String,
    pub mod_name: String,
    pub destination_rel_path: String,
    pub backup_file_path: String,
    pub original_sha256: String,
    pub original_size_bytes: u64,
    pub timestamp: String,
    pub restored: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoveryJournalEntry {
    pub id: String,
    pub operation_id: String,
    pub operation_type: String, // "install", "uninstall", "profile_switch"
    pub status: String, // "in_progress", "committed", "interrupted", "rolled_back"
    pub target_mod_id: String,
    pub target_mod_name: String,
    pub timestamp: String,
    pub staged_backup_paths: Vec<String>,
    pub affected_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    pub app_version: String,
    pub os_version: String,
    pub rust_version: String,
    pub game_detected: bool,
    pub active_game: Option<GameInstallation>,
    pub installed_mods_count: usize,
    pub pending_operations: usize,
    pub sqlite_health: String,
    pub recent_logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheCleanupResult {
    pub files_removed: usize,
    pub bytes_freed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub default_game_installation_id: Option<String>,
    pub download_directory: String,
    pub backup_directory: String,
    pub max_concurrent_downloads: u32,
    pub require_install_confirmation: bool,
    pub auto_check_updates: bool,
    pub theme: String,
    pub log_level: String,
    pub default_launch_args: Option<String>,
    pub default_aspect_ratio: Option<String>,
    pub auto_resolve_conflicts: Option<bool>,
    pub auto_detect_game_on_startup: Option<bool>,
}

