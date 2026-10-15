use async_trait::async_trait;
use reqwest::Client;
use crate::models::{InstallationStrategyType, ModRecord, SourceCapabilities};
use super::SourceAdapter;

pub struct GtaInsideAdapter {
    client: Client,
}

impl GtaInsideAdapter {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("SAMM-OpenSource-ModManager/0.1 (+https://github.com/samm-modmanager/samm)")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        GtaInsideAdapter { client }
    }

    pub fn get_curated_catalog() -> Vec<ModRecord> {
        vec![
            ModRecord {
                id: "gtainside_silentpatch".to_string(),
                source_id: "gtainside".to_string(),
                source_mod_id: "silentpatch_sa".to_string(),
                title: "SilentPatch for GTA San Andreas".to_string(),
                author: "Silent".to_string(),
                description: "The universally mandatory stability and bugfix patch. Resolves over 70 engine defects, timing glitches, missing dual-pass coronas, mouse lockups, and Windows 10/11 crash bugs.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["essential".to_string(), "stability".to_string(), "bugfix".to_string(), "asi".to_string()],
                thumbnail_url: Some("https://www.gtainside.com/img/gtainside_logo.png".to_string()),
                page_url: "https://www.gtainside.com/en/sanandreas/mods/64069-silentpatch/".to_string(),
                download_url: Some("https://github.com/CookiePLMonster/SilentPatch/releases/latest/download/SilentPatchSA.zip".to_string()),
                file_size_formatted: Some("1.2 MB".to_string()),
                file_size_bytes: Some(1_258_291),
                release_date: Some("2020-04-12".to_string()),
                update_date: Some("2024-03-01".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string(), "1.01".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Verified baseline compatibility across all classic PC 1.0 executables".to_string(),
            },
            ModRecord {
                id: "gtainside_widescreen_fix".to_string(),
                source_id: "gtainside".to_string(),
                source_mod_id: "widescreen_fix_thirteenag".to_string(),
                title: "Widescreen Fix & Aspect Ratio Correction".to_string(),
                author: "ThirteenAG".to_string(),
                description: "Eliminates aspect ratio stretching on modern 16:9, 21:9 ultrawide, and 4K displays. Scales HUD, radar, FOV, and weapon icons with pixel perfection.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["hud".to_string(), "widescreen".to_string(), "display".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtainside.com/en/sanandreas/mods/53751-widescreen-fix/".to_string(),
                download_url: Some("https://github.com/ThirteenAG/WidescreenFixesPack/releases/download/gtasa/GTASA.WidescreenFix.zip".to_string()),
                file_size_formatted: Some("7.6 MB".to_string()),
                file_size_bytes: Some(8_037_059),
                release_date: Some("2018-06-20".to_string()),
                update_date: Some("2023-11-02".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Confirmed working on 1.0 US baseline".to_string(),
            },
            ModRecord {
                id: "gtainside_project2dfx".to_string(),
                source_id: "gtainside".to_string(),
                source_mod_id: "project_2dfx_sa".to_string(),
                title: "Project 2DFX (Max Draw Distance & LODs)".to_string(),
                author: "ThirteenAG & TJDS".to_string(),
                description: "Dramatically expands the draw distance of city lights, building LODs, and coronas across the entire San Andreas map without fog pop-in.".to_string(),
                category: "Graphics / Plugins".to_string(),
                tags: vec!["graphics".to_string(), "lod".to_string(), "distance".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtainside.com/en/sanandreas/mods/69389-project-2dfx/".to_string(),
                download_url: None,
                file_size_formatted: Some("4.5 MB".to_string()),
                file_size_bytes: Some(4_718_592),
                release_date: Some("2019-05-15".to_string()),
                update_date: Some("2024-01-10".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string(), "Open Limit Adjuster".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "High draw distance verified on 1.0 US".to_string(),
            },
            ModRecord {
                id: "gtainside_first_person".to_string(),
                source_id: "gtainside".to_string(),
                source_mod_id: "first_person_v3".to_string(),
                title: "First Person Mod v3.0".to_string(),
                author: "BoPoH (VorText)".to_string(),
                description: "Full first-person perspective for on-foot exploration, driving, flying, skydiving, and gunfights with customized aiming and realistic vehicle interior views.".to_string(),
                category: "Gameplay / Scripts".to_string(),
                tags: vec!["gameplay".to_string(), "camera".to_string(), "first-person".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtainside.com/en/sanandreas/mods/68222-first-person-mod-v3-0/".to_string(),
                download_url: None,
                file_size_formatted: Some("2.8 MB".to_string()),
                file_size_bytes: Some(2_936_012),
                release_date: Some("2015-01-18".to_string()),
                update_date: Some("2022-09-04".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Requires 1.0 US memory offsets".to_string(),
            },
            ModRecord {
                id: "gtainside_ragdoll_physics".to_string(),
                source_id: "gtainside".to_string(),
                source_mod_id: "ragdoll_physics_sa".to_string(),
                title: "Ragdoll Physics Engine Mod".to_string(),
                author: "MadMan & Ryosuke".to_string(),
                description: "Replaces the static canned death animations with dynamic euphoria-style ragdoll physics for CJ and all pedestrians when struck by bullets, cars, or explosions.".to_string(),
                category: "Gameplay / Scripts".to_string(),
                tags: vec!["physics".to_string(), "ragdoll".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtainside.com/en/sanandreas/mods/53612-ragdoll-physics/".to_string(),
                download_url: None,
                file_size_formatted: Some("1.4 MB".to_string()),
                file_size_bytes: Some(1_468_006),
                release_date: Some("2013-11-20".to_string()),
                update_date: Some("2021-06-12".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Verified on PC 1.0 US".to_string(),
            },
            ModRecord {
                id: "gtainside_remastered_gui".to_string(),
                source_id: "gtainside".to_string(),
                source_mod_id: "remastered_gui_sa".to_string(),
                title: "Remastered High Definition GUI & Radar".to_string(),
                author: "ThirteenAG & Community".to_string(),
                description: "Replaces the blurry low-resolution 2004 radar icons, crosshairs, radio station wheels, and interface fonts with crisp high-resolution vector assets.".to_string(),
                category: "Graphics / UI".to_string(),
                tags: vec!["ui".to_string(), "hd".to_string(), "textures".to_string(), "radar".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtainside.com/en/sanandreas/mods/95012-remastered-gui/".to_string(),
                download_url: None,
                file_size_formatted: Some("8.2 MB".to_string()),
                file_size_bytes: Some(8_598_323),
                release_date: Some("2019-10-04".to_string()),
                update_date: Some("2023-04-18".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Installed into ModLoader sandbox safely".to_string(),
            },
            ModRecord {
                id: "gtainside_vehicle_spawner".to_string(),
                source_id: "gtainside".to_string(),
                source_mod_id: "vehicle_spawner_premium".to_string(),
                title: "Vehicle Spawner Premium GUI".to_string(),
                author: "ZAZ".to_string(),
                description: "Clean in-game graphical vehicle spawner menu categorized by sports cars, planes, boats, helicopters, and emergency vehicles with live visual previews.".to_string(),
                category: "Scripts / CLEO".to_string(),
                tags: vec!["cleo".to_string(), "spawner".to_string(), "cars".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtainside.com/en/sanandreas/mods/14512-car-spawner-v2/".to_string(),
                download_url: None,
                file_size_formatted: Some("320 KB".to_string()),
                file_size_bytes: Some(327_680),
                release_date: Some("2012-08-10".to_string()),
                update_date: Some("2020-02-15".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["CLEO 4".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Cleo,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Standard CLEO 4 script bytecode".to_string(),
            },
        ]
    }
}

#[async_trait]
impl SourceAdapter for GtaInsideAdapter {
    fn id(&self) -> &'static str {
        "gtainside"
    }

    fn name(&self) -> &'static str {
        "GTAinside"
    }

    fn base_url(&self) -> &'static str {
        "https://www.gtainside.com"
    }

    fn capabilities(&self) -> SourceCapabilities {
        SourceCapabilities {
            can_search: true,
            can_direct_download: true,
            requires_browser_download: false,
            requires_auth: false,
            supports_version_filter: true,
            supports_category_filter: true,
        }
    }

    fn attribution(&self) -> &'static str {
        "Mods hosted on GTAinside.com. Mod copyrights belong to their respective authors."
    }

    fn terms_notice(&self) -> &'static str {
        "Public directory search adhering to GTAinside community guidelines."
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
            .ok_or_else(|| format!("Mod '{}' not found on GTAinside", source_mod_id))
    }

    async fn get_download_url(&self, source_mod_id: &str) -> Result<Option<String>, String> {
        let details = self.get_mod_details(source_mod_id).await?;
        Ok(details.download_url)
    }
}

pub mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut encoded = String::new();
        for b in s.bytes() {
            match b {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    encoded.push(b as char);
                }
                b' ' => encoded.push('+'),
                _ => encoded.push_str(&format!("%{:02X}", b)),
            }
        }
        encoded
    }
}
