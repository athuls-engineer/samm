use async_trait::async_trait;
use reqwest::Client;
use crate::models::{InstallationStrategyType, ModRecord, SourceCapabilities};
use super::SourceAdapter;

pub struct MixModsAdapter {
    client: Client,
}

impl MixModsAdapter {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("SAMM-OpenSource-ModManager/0.1 (+https://github.com/samm-modmanager/samm)")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        MixModsAdapter { client }
    }
}

#[async_trait]
impl SourceAdapter for MixModsAdapter {
    fn id(&self) -> &'static str {
        "mixmods"
    }

    fn name(&self) -> &'static str {
        "MixMods"
    }

    fn base_url(&self) -> &'static str {
        "https://www.mixmods.com.br"
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
        "Curated by Junior_Djjr and the MixMods technical modding community."
    }

    fn terms_notice(&self) -> &'static str {
        "MixMods hosts technical analysis and verified mod links. Users are directed to author article pages for official releases."
    }

    async fn search_mods(
        &self,
        query: &str,
        category: Option<&str>,
        _page: u32,
    ) -> Result<Vec<ModRecord>, String> {
        let mut results = Vec::new();

        if query.is_empty() || query.to_lowercase().contains("modloader") || query.to_lowercase().contains("loader") {
            results.push(ModRecord {
                id: "mixmods_modloader".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "modloader_sa".to_string(),
                title: "ModLoader for GTA San Andreas".to_string(),
                author: "LINK/2012".to_string(),
                description: "Revolutionary folder-based modding framework that injects modded textures, models, and scripts into RAM dynamically without editing gta3.img.".to_string(),
                category: "Tools / Frameworks".to_string(),
                tags: vec!["core".to_string(), "essential".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2015/01/modloader.html".to_string(),
                download_url: None,
                file_size_formatted: Some("1.5 MB".to_string()),
                file_size_bytes: Some(1_572_864),
                release_date: Some("2015-01-10".to_string()),
                update_date: Some("2024-02-01".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Primary recommended mod injection framework for PC 1.0".to_string(),
            });
        }

        if query.is_empty() || query.to_lowercase().contains("framerate") || query.to_lowercase().contains("fps") || query.to_lowercase().contains("vigilante") {
            results.push(ModRecord {
                id: "mixmods_framerate_vigilante".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "framerate_vigilante".to_string(),
                title: "Framerate Vigilante (60FPS+ Physics Fix)".to_string(),
                author: "Junior_Djjr".to_string(),
                description: "Fixes dozens of high-FPS physics glitches when running San Andreas at 60 FPS or higher (swimming speed, car braking, siren timing, dive bugs).".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["fps".to_string(), "physics".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2021/04/framerate-vigilante/".to_string(),
                download_url: None,
                file_size_formatted: Some("420 KB".to_string()),
                file_size_bytes: Some(430_080),
                release_date: Some("2021-04-05".to_string()),
                update_date: Some("2023-12-14".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Targeted explicitly for 1.0 US executable".to_string(),
            });
        }

        Ok(results)
    }

    async fn get_mod_details(&self, source_mod_id: &str) -> Result<ModRecord, String> {
        let results = self.search_mods(source_mod_id, None, 1).await?;
        results
            .into_iter()
            .find(|m| m.source_mod_id == source_mod_id)
            .ok_or_else(|| format!("Mod '{}' not found on MixMods", source_mod_id))
    }

    async fn get_download_url(&self, _source_mod_id: &str) -> Result<Option<String>, String> {
        Ok(None)
    }
}
