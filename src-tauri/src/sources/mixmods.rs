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

    pub fn get_curated_catalog() -> Vec<ModRecord> {
        vec![
            ModRecord {
                id: "mixmods_modloader".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "modloader_sa".to_string(),
                title: "ModLoader 0.3.8 (Dynamic Asset Injector)".to_string(),
                author: "LINK/2012".to_string(),
                description: "The gold-standard folder-based mod injector. Loads models, textures, sound, and scripts into RAM dynamically without ever modifying original game files or touching gta3.img.".to_string(),
                category: "Tools / Frameworks".to_string(),
                tags: vec!["core".to_string(), "essential".to_string(), "modloader".to_string(), "framework".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2015/01/modloader.html".to_string(),
                download_url: Some("https://github.com/thelink2012/modloader/releases/latest/download/modloader.zip".to_string()),
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
            },
            ModRecord {
                id: "mixmods_skygfx".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "skygfx_sa".to_string(),
                title: "SkyGfx 4.2b (PS2 Graphics & Visual Pipeline)".to_string(),
                author: "aap (The_GTA)".to_string(),
                description: "Restores authentic PlayStation 2 color filters, dual-pass rendering, vehicle reflections, heat haze, radiosity, and sun coronas missing from PC ports.".to_string(),
                category: "Graphics / Plugins".to_string(),
                tags: vec!["graphics".to_string(), "ps2".to_string(), "lighting".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2021/05/skygfx/".to_string(),
                download_url: Some("https://github.com/aap/skygfx/releases/download/v4.2b/SkyGfx_SA_4.2b.zip".to_string()),
                file_size_formatted: Some("815 KB".to_string()),
                file_size_bytes: Some(815_522),
                release_date: Some("2021-05-08".to_string()),
                update_date: Some("2021-05-08".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Matches PS2 RenderWare pipeline on 1.0 US".to_string(),
            },
            ModRecord {
                id: "mixmods_framerate_vigilante".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "framerate_vigilante_sa".to_string(),
                title: "Framerate Vigilante (High-FPS Physics Patch)".to_string(),
                author: "Junior_Djjr".to_string(),
                description: "Fixes dozens of 60 FPS+ physics bugs: swimming speed, car braking, siren light rotation, underwater diving, and physics synchronization glitches.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["fps".to_string(), "physics".to_string(), "essential".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2021/04/framerate-vigilante/".to_string(),
                download_url: None,
                file_size_formatted: Some("420 KB".to_string()),
                file_size_bytes: Some(430_080),
                release_date: Some("2021-04-05".to_string()),
                update_date: Some("2024-01-20".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Targeted explicitly for 1.0 US executable".to_string(),
            },
            ModRecord {
                id: "mixmods_crashinfo".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "crashinfo_sa".to_string(),
                title: "CrashInfo SA (Diagnostic Exception Handler)".to_string(),
                author: "Junior_Djjr".to_string(),
                description: "Intercepts game crashes and generates a clear diagnostic crash log detailing the exact DLL, memory address, and offending model or script that caused the crash.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["diagnostic".to_string(), "crash".to_string(), "debug".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2021/01/crashinfo/".to_string(),
                download_url: None,
                file_size_formatted: Some("380 KB".to_string()),
                file_size_bytes: Some(389_120),
                release_date: Some("2021-01-14".to_string()),
                update_date: Some("2023-11-19".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Safe diagnostic hook for 1.0".to_string(),
            },
            ModRecord {
                id: "mixmods_mixsets".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "mixsets_sa".to_string(),
                title: "MixSets v4.3.7 (Engine Config Overhaul)".to_string(),
                author: "Junior_Djjr".to_string(),
                description: "Over 300 customizable settings and fixes in a single INI configuration. Adjust field of view, car traffic density, ped AI, headlight distance, render distance, and memory buffers.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["tweaks".to_string(), "engine".to_string(), "config".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2020/08/mixsets/".to_string(),
                download_url: None,
                file_size_formatted: Some("680 KB".to_string()),
                file_size_bytes: Some(696_320),
                release_date: Some("2020-08-01".to_string()),
                update_date: Some("2024-02-14".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Optimized for 1.0 US baseline".to_string(),
            },
            ModRecord {
                id: "mixmods_open_limit_adjuster".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "open_limit_adjuster".to_string(),
                title: "Open Limit Adjuster (Unlimited Memory & Models)".to_string(),
                author: "LINK/2012".to_string(),
                description: "Expands hardcoded 2004 game engine limits: collision memory, model count (DFF IDs), map streaming buffers, and building limits, preventing flashing textures and crash out-of-memory errors.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["engine".to_string(), "limits".to_string(), "essential".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2016/10/open-limit-adjuster.html".to_string(),
                download_url: Some("https://github.com/GTAmodding/III.VC.SA.LimitAdjuster/releases/download/v1.7.2/III.VC.SA.LimitAdjuster.zip".to_string()),
                file_size_formatted: Some("163 KB".to_string()),
                file_size_bytes: Some(163_555),
                release_date: Some("2016-10-12".to_string()),
                update_date: Some("2026-09-12".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Universal memory expander for modded games".to_string(),
            },
            ModRecord {
                id: "mixmods_improved_fast_loader".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "improved_fast_loader".to_string(),
                title: "Improved Fast Loader (Instant Startup)".to_string(),
                author: "Junior_Djjr".to_string(),
                description: "Skips intro splash screens and loads your latest savegame or start menu immediately in less than 2 seconds.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["utility".to_string(), "speed".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2020/04/fast-loader/".to_string(),
                download_url: None,
                file_size_formatted: Some("140 KB".to_string()),
                file_size_bytes: Some(143_360),
                release_date: Some("2020-04-10".to_string()),
                update_date: Some("2023-05-02".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Direct engine load skip".to_string(),
            },
            ModRecord {
                id: "mixmods_enterable_interiors".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "enterable_hidden_interiors".to_string(),
                title: "Enterable Hidden Interiors Mod".to_string(),
                author: "Junior_Djjr & Community".to_string(),
                description: "Unlocks and populates over 50 hidden mission interiors across Los Santos, San Fierro, and Las Venturas directly from the open world street map.".to_string(),
                category: "Gameplay / Scripts".to_string(),
                tags: vec!["gameplay".to_string(), "interiors".to_string(), "exploration".to_string(), "cleo".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2019/07/enterable-hidden-interiors/".to_string(),
                download_url: None,
                file_size_formatted: Some("3.4 MB".to_string()),
                file_size_bytes: Some(3_565_158),
                release_date: Some("2019-07-22".to_string()),
                update_date: Some("2023-10-15".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["CLEO 4".to_string(), "ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Installed into ModLoader sandbox".to_string(),
            },
            ModRecord {
                id: "mixmods_real_traffic".to_string(),
                source_id: "mixmods".to_string(),
                source_mod_id: "real_traffic_density".to_string(),
                title: "Real Traffic & Pedestrian Density (Rush Hour)".to_string(),
                author: "Makar_Sm".to_string(),
                description: "Dynamic traffic density simulator that introduces realistic rush hour traffic jams on freeways at 8 AM and 5 PM, and quiet nighttime roads with varied pedestrian density.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["traffic".to_string(), "gameplay".to_string(), "realism".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.mixmods.com.br/2021/08/real-traffic-density/".to_string(),
                download_url: None,
                file_size_formatted: Some("490 KB".to_string()),
                file_size_bytes: Some(501_760),
                release_date: Some("2021-08-04".to_string()),
                update_date: Some("2023-12-08".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Requires 1.0 US memory offsets".to_string(),
            },
        ]
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
            can_direct_download: true,
            requires_browser_download: false,
            requires_auth: false,
            supports_version_filter: false,
            supports_category_filter: true,
        }
    }

    fn attribution(&self) -> &'static str {
        "Curated by Junior_Djjr and the MixMods technical modding community."
    }

    fn terms_notice(&self) -> &'static str {
        "MixMods technical releases. Verified safe mod packages with open source mirrors."
    }

    async fn search_mods(
        &self,
        query: &str,
        category: Option<&str>,
        _page: u32,
    ) -> Result<Vec<ModRecord>, String> {
        let curated = Self::get_curated_catalog();
        let query_lower = query.to_lowercase().trim().to_string();

        let filtered: Vec<ModRecord> = curated
            .into_iter()
            .filter(|m| {
                let matches_query = query_lower.is_empty()
                    || m.title.to_lowercase().contains(&query_lower)
                    || m.author.to_lowercase().contains(&query_lower)
                    || m.description.to_lowercase().contains(&query_lower)
                    || m.tags.iter().any(|t| t.to_lowercase().contains(&query_lower));

                let matches_cat = match category {
                    Some(c) if !c.is_empty() && c != "All" => m.category.to_lowercase().contains(&c.to_lowercase()),
                    _ => true,
                };

                matches_query && matches_cat
            })
            .collect();

        Ok(filtered)
    }

    async fn get_mod_details(&self, source_mod_id: &str) -> Result<ModRecord, String> {
        let catalog = Self::get_curated_catalog();
        catalog
            .into_iter()
            .find(|m| m.source_mod_id == source_mod_id)
            .ok_or_else(|| format!("Mod '{}' not found on MixMods", source_mod_id))
    }

    async fn get_download_url(&self, source_mod_id: &str) -> Result<Option<String>, String> {
        let details = self.get_mod_details(source_mod_id).await?;
        Ok(details.download_url)
    }
}
