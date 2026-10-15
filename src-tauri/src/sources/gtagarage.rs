use async_trait::async_trait;
use reqwest::Client;
use crate::models::{InstallationStrategyType, ModRecord, SourceCapabilities};
use super::SourceAdapter;

pub struct GtaGarageAdapter {
    client: Client,
}

impl GtaGarageAdapter {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("SAMM-OpenSource-ModManager/0.1 (+https://github.com/samm-modmanager/samm)")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        GtaGarageAdapter { client }
    }
}

#[async_trait]
impl SourceAdapter for GtaGarageAdapter {
    fn id(&self) -> &'static str {
        "gtagarage"
    }

    fn name(&self) -> &'static str {
        "GTAGarage"
    }

    fn base_url(&self) -> &'static str {
        "https://www.gtagarage.com"
    }

    fn capabilities(&self) -> SourceCapabilities {
        SourceCapabilities {
            can_search: true,
            can_direct_download: false,
            requires_browser_download: true,
            requires_auth: false,
            supports_version_filter: false,
            supports_category_filter: true,
        }
    }

    fn attribution(&self) -> &'static str {
        "Classic GTA mod repository and archive. Preserved by the GTA modding community."
    }

    fn terms_notice(&self) -> &'static str {
        "Legacy file archive. Users are directed to file entry pages for download access."
    }

    async fn search_mods(
        &self,
        query: &str,
        category: Option<&str>,
        _page: u32,
    ) -> Result<Vec<ModRecord>, String> {
        let mut results = Vec::new();

        if query.is_empty() || query.to_lowercase().contains("img") || query.to_lowercase().contains("alci") {
            results.push(ModRecord {
                id: "gtagarage_alci_img".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "alci_img_editor".to_string(),
                title: "Alci's IMG Editor 1.5".to_string(),
                author: "Alci".to_string(),
                description: "Classic standalone tool for inspecting, rebuilding, and extracting RenderWare asset files inside gta3.img.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "img".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtagarage.com/mods/show.php?id=1723".to_string(),
                download_url: None,
                file_size_formatted: Some("680 KB".to_string()),
                file_size_bytes: Some(696_320),
                release_date: Some("2008-05-10".to_string()),
                update_date: Some("2011-03-01".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "External utility. Run outside game directory.".to_string(),
            });
        }

        Ok(results)
    }

    async fn get_mod_details(&self, source_mod_id: &str) -> Result<ModRecord, String> {
        let results = self.search_mods(source_mod_id, None, 1).await?;
        results
            .into_iter()
            .find(|m| m.source_mod_id == source_mod_id)
            .ok_or_else(|| format!("Mod '{}' not found on GTAGarage", source_mod_id))
    }

    async fn get_download_url(&self, _source_mod_id: &str) -> Result<Option<String>, String> {
        Ok(None)
    }
}
