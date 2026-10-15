use std::path::Path;
use async_trait::async_trait;
use crate::models::{InstallationStrategyType, ModRecord, SourceCapabilities};
use crate::archives::inspect_archive;
use super::SourceAdapter;

pub struct ManualImportAdapter {}

impl ManualImportAdapter {
    pub fn new() -> Self {
        ManualImportAdapter {}
    }

    pub fn inspect_local_archive(&self, archive_path: &Path) -> Result<ModRecord, String> {
        let inspection = inspect_archive(archive_path)?;
        let file_name = archive_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Imported_Mod.zip");

        let title = file_name.replace(".zip", "").replace(".rar", "").replace(".7z", "").replace('_', " ");

        let mut has_cs = false;
        let mut has_asi = false;
        let mut has_exe = false;

        for entry in &inspection.entries {
            let ext = Path::new(&entry.path)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            if ext == "cs" || ext == "fxt" {
                has_cs = true;
            } else if ext == "asi" {
                has_asi = true;
            } else if ["exe", "bat", "msi"].contains(&ext.as_str()) {
                has_exe = true;
            }
        }

        let strategy = if has_exe {
            InstallationStrategyType::Manual
        } else if has_cs {
            InstallationStrategyType::Cleo
        } else if has_asi {
            InstallationStrategyType::Asi
        } else {
            InstallationStrategyType::Modloader
        };

        let (sha256, size) = crate::game_detection::compute_file_sha256(archive_path)?;
        let size_mb = (size as f64) / (1024.0 * 1024.0);

        Ok(ModRecord {
            id: format!("local_{}", uuid::Uuid::new_v4().simple()),
            source_id: "manual_import".to_string(),
            source_mod_id: sha256.clone(),
            title,
            author: "Local Import (User Provided)".to_string(),
            description: format!(
                "Locally imported mod archive with {} files ({:.2} MB uncompressed). SHA-256: {}",
                inspection.total_entries,
                (inspection.total_uncompressed_size as f64) / (1024.0 * 1024.0),
                sha256
            ),
            category: "Local Archive".to_string(),
            tags: vec!["local".to_string(), "custom".to_string()],
            thumbnail_url: None,
            page_url: archive_path.to_string_lossy().to_string(),
            download_url: Some(archive_path.to_string_lossy().to_string()),
            file_size_formatted: Some(format!("{:.2} MB", size_mb)),
            file_size_bytes: Some(size),
            release_date: None,
            update_date: Some(chrono::Utc::now().format("%Y-%m-%d").to_string()),
            supported_game_versions: vec!["Classic PC 1.0".to_string()],
            required_dependencies: if has_cs { vec!["CLEO 4".to_string()] } else { Vec::new() },
            incompatibilities: Vec::new(),
            installation_strategy: strategy,
            confidence: "confirmed".to_string(),
            compatibility_reason: "Locally inspected file with verified archive integrity".to_string(),
        })
    }
}

#[async_trait]
impl SourceAdapter for ManualImportAdapter {
    fn id(&self) -> &'static str {
        "manual_import"
    }

    fn name(&self) -> &'static str {
        "Manual Archive Import"
    }

    fn base_url(&self) -> &'static str {
        "file://"
    }

    fn capabilities(&self) -> SourceCapabilities {
        SourceCapabilities {
            can_search: false,
            can_direct_download: false,
            requires_browser_download: false,
            requires_auth: false,
            supports_version_filter: false,
            supports_category_filter: false,
        }
    }

    fn attribution(&self) -> &'static str {
        "User-supplied local archives and community packages."
    }

    fn terms_notice(&self) -> &'static str {
        "Ensure you have lawful rights and permissions to use and install imported third-party mod files."
    }

    async fn search_mods(
        &self,
        _query: &str,
        _category: Option<&str>,
        _page: u32,
    ) -> Result<Vec<ModRecord>, String> {
        Ok(Vec::new())
    }

    async fn get_mod_details(&self, source_mod_id: &str) -> Result<ModRecord, String> {
        let p = Path::new(source_mod_id);
        if p.exists() {
            self.inspect_local_archive(p)
        } else {
            Err(format!("Local archive file '{}' not found", source_mod_id))
        }
    }

    async fn get_download_url(&self, source_mod_id: &str) -> Result<Option<String>, String> {
        Ok(Some(source_mod_id.to_string()))
    }
}
