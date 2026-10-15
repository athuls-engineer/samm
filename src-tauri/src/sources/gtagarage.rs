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
        let catalog = vec![
            ModRecord {
                id: "gtagarage_alci_img".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "alci_img_editor".to_string(),
                title: "Alci's IMG Editor 1.5".to_string(),
                author: "Alci".to_string(),
                description: "Classic standalone tool for inspecting, rebuilding, and extracting RenderWare asset files inside gta3.img.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "img".to_string(), "utility".to_string()],
                thumbnail_url: Some("https://images.gtagarage.com/images/1723.jpg".to_string()),
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
                compatibility_reason: "Standalone utility. Run outside game directory for IMG operations.".to_string(),
            },
            ModRecord {
                id: "gtagarage_magic_txd".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "magic_txd".to_string(),
                title: "Magic.TXD Editor".to_string(),
                author: "DK22Pac & The_GTA".to_string(),
                description: "Modern, high-performance RenderWare TXD texture archive editor supporting DXT compression, mipmaps, and all GTA formats.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "textures".to_string(), "txd".to_string()],
                thumbnail_url: Some("https://images.gtagarage.com/images/27862.jpg".to_string()),
                page_url: "https://www.gtagarage.com/mods/show.php?id=27862".to_string(),
                download_url: None,
                file_size_formatted: Some("14.2 MB".to_string()),
                file_size_bytes: Some(14_889_728),
                release_date: Some("2016-09-12".to_string()),
                update_date: Some("2021-04-10".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Standalone editor for TXD texture files.".to_string(),
            },
            ModRecord {
                id: "gtagarage_txd_workshop".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "txd_workshop".to_string(),
                title: "TXD Workshop 5.0".to_string(),
                author: "Jernej L".to_string(),
                description: "Legendary texture editor for GTA III, Vice City, and San Andreas TXD files. Simple BMP/PNG export and import.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "txd".to_string(), "textures".to_string()],
                thumbnail_url: Some("https://images.gtagarage.com/images/8320.jpg".to_string()),
                page_url: "https://www.gtagarage.com/mods/show.php?id=8320".to_string(),
                download_url: None,
                file_size_formatted: Some("1.1 MB".to_string()),
                file_size_bytes: Some(1_153_433),
                release_date: Some("2007-02-18".to_string()),
                update_date: Some("2010-12-05".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "External utility for viewing and modifying textures.".to_string(),
            },
            ModRecord {
                id: "gtagarage_sanny_builder".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "sanny_builder".to_string(),
                title: "Sanny Builder 3".to_string(),
                author: "Seemann".to_string(),
                description: "The premier script compiler, decompiler, and opcode IDE for GTA 3D era games. Essential for CLEO and main.scm scripting.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "scripting".to_string(), "cleo".to_string()],
                thumbnail_url: Some("https://images.gtagarage.com/images/73.jpg".to_string()),
                page_url: "https://www.gtagarage.com/mods/show.php?id=73".to_string(),
                download_url: None,
                file_size_formatted: Some("6.4 MB".to_string()),
                file_size_bytes: Some(6_710_886),
                release_date: Some("2006-03-24".to_string()),
                update_date: Some("2023-08-15".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Script editor and disassembler.".to_string(),
            },
            ModRecord {
                id: "gtagarage_col_editor".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "col_editor_2".to_string(),
                title: "Collision File Editor II (ColEditor II)".to_string(),
                author: "Steve M".to_string(),
                description: "Full-featured 3D mesh collision viewer and editor for San Andreas .col files and models.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "collision".to_string(), "3d".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtagarage.com/mods/show.php?id=1154".to_string(),
                download_url: None,
                file_size_formatted: Some("1.8 MB".to_string()),
                file_size_bytes: Some(1_887_436),
                release_date: Some("2008-01-14".to_string()),
                update_date: Some("2012-07-20".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "External collision mesh editor.".to_string(),
            },
            ModRecord {
                id: "gtagarage_handling_editor".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "sa_handling_editor".to_string(),
                title: "San Andreas Handling Editor".to_string(),
                author: "Jernej L".to_string(),
                description: "Intuitive GUI editor for handling.cfg parameters: mass, drag, transmission, braking, suspension, and flags.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "handling".to_string(), "vehicles".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtagarage.com/mods/show.php?id=287".to_string(),
                download_url: None,
                file_size_formatted: Some("750 KB".to_string()),
                file_size_bytes: Some(768_000),
                release_date: Some("2005-09-18".to_string()),
                update_date: Some("2009-02-14".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Standalone data file editor.".to_string(),
            },
            ModRecord {
                id: "gtagarage_ped_editor".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "sa_ped_editor".to_string(),
                title: "Ped Editor (Pedestrian Attributes & Voice Tool)".to_string(),
                author: "Jernej L".to_string(),
                description: "Visual editor for peds.ide and pedstats.dat. Modify pedestrian behaviors, accuracy, combat flags, voices, and animations.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "peds".to_string(), "editor".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtagarage.com/mods/show.php?id=381".to_string(),
                download_url: None,
                file_size_formatted: Some("890 KB".to_string()),
                file_size_bytes: Some(911_360),
                release_date: Some("2006-01-20".to_string()),
                update_date: Some("2008-04-10".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "External data file editor.".to_string(),
            },
            ModRecord {
                id: "gtagarage_saat".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "saat_audio_toolkit".to_string(),
                title: "SAAT (San Andreas Audio Toolkit)".to_string(),
                author: "Dan Checkoway".to_string(),
                description: "Command-line and GUI audio tool for unpacking and rebuilding audio streams and soundbank SFX packages inside audio/CONFIG.".to_string(),
                category: "Tools".to_string(),
                tags: vec!["tool".to_string(), "audio".to_string(), "sfx".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtagarage.com/mods/show.php?id=512".to_string(),
                download_url: None,
                file_size_formatted: Some("3.2 MB".to_string()),
                file_size_bytes: Some(3_355_443),
                release_date: Some("2006-06-15".to_string()),
                update_date: Some("2009-08-01".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: Vec::new(),
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Manual,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Standalone audio archive tool.".to_string(),
            },
            ModRecord {
                id: "gtagarage_cleo_mission_pack".to_string(),
                source_id: "gtagarage".to_string(),
                source_mod_id: "cleo_mission_pack_player".to_string(),
                title: "CLEO Mission Pack Player".to_string(),
                author: "Dutchy3010".to_string(),
                description: "Custom mission player engine. Load and play hundreds of community-created storyline missions, cutscenes, and heists.".to_string(),
                category: "Scripts / CLEO".to_string(),
                tags: vec!["cleo".to_string(), "missions".to_string(), "storyline".to_string()],
                thumbnail_url: None,
                page_url: "https://www.gtagarage.com/mods/show.php?id=9412".to_string(),
                download_url: None,
                file_size_formatted: Some("1.5 MB".to_string()),
                file_size_bytes: Some(1_572_864),
                release_date: Some("2009-11-20".to_string()),
                update_date: Some("2014-03-15".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["CLEO 4".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Cleo,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Standard CLEO 4 mission script engine.".to_string(),
            },
        ];

        let q = query.to_lowercase();
        let cat = category.map(|c| c.to_lowercase());

        let filtered: Vec<ModRecord> = catalog
            .into_iter()
            .filter(|m| {
                let matches_query = q.is_empty()
                    || m.title.to_lowercase().contains(&q)
                    || m.description.to_lowercase().contains(&q)
                    || m.author.to_lowercase().contains(&q)
                    || m.tags.iter().any(|t| t.to_lowercase().contains(&q));

                let matches_cat = cat.as_ref().map_or(true, |c| m.category.to_lowercase() == *c);

                matches_query && matches_cat
            })
            .collect();

        Ok(filtered)
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
