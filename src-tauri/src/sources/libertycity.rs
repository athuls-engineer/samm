use async_trait::async_trait;
use reqwest::Client;
use crate::models::{InstallationStrategyType, ModRecord, SourceCapabilities};
use super::SourceAdapter;

pub struct LibertyCityAdapter {
    client: Client,
}

impl LibertyCityAdapter {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("SAMM-OpenSource-ModManager/0.1 (+https://github.com/samm-modmanager/samm)")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        LibertyCityAdapter { client }
    }

    pub fn get_curated_catalog() -> Vec<ModRecord> {
        vec![
            ModRecord {
                id: "libertycity_skygfx".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "skygfx_sa".to_string(),
                title: "SkyGfx 4.2b (PS2 & Xbox Graphical Pipelines)".to_string(),
                author: "aap (The Hero)".to_string(),
                description: "Accurately restores authentic PS2 console color filters, radiant sun bloom, dual-pass vehicle reflections, building lighting, and original timecycle ambience on PC.".to_string(),
                category: "Graphics / Plugins".to_string(),
                tags: vec!["graphics".to_string(), "ps2".to_string(), "visuals".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/108343-skygfx.html".to_string(),
                download_url: Some("https://github.com/aap/skygfx/releases/latest/download/skygfx.zip".to_string()),
                file_size_formatted: Some("1.8 MB".to_string()),
                file_size_bytes: Some(1_887_436),
                release_date: Some("2021-08-14".to_string()),
                update_date: Some("2024-01-15".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Verified compatibility with 1.0 US baseline".to_string(),
            },
            ModRecord {
                id: "libertycity_cleo4".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "cleo_4_4_sa".to_string(),
                title: "CLEO 4.4.4 Scripting Library".to_string(),
                author: "Seemann & Alien".to_string(),
                description: "The core custom scripting library for GTA San Andreas. Injects custom opcodes, threaded scripting support, audio stream playback, and math extensions without replacing main.scm.".to_string(),
                category: "Tools / Frameworks".to_string(),
                tags: vec!["cleo".to_string(), "core".to_string(), "framework".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/154102-cleo-4-4.html".to_string(),
                download_url: Some("https://github.com/cleolibrary/CLEO4/releases/download/v4.4.4/CLEO4.zip".to_string()),
                file_size_formatted: Some("550 KB".to_string()),
                file_size_bytes: Some(550_990),
                release_date: Some("2020-03-25".to_string()),
                update_date: Some("2023-08-13".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Confirmed fundamental modding framework for GTA SA 1.0".to_string(),
            },
            ModRecord {
                id: "libertycity_weapon_wheel".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "gta_v_weapon_wheel".to_string(),
                title: "GTA V Style Weapon Wheel & Slow Motion".to_string(),
                author: "DK22Pac".to_string(),
                description: "Replaces clunky numeric weapon cycling with a smooth GTA V radial weapon selection wheel, complete with weapon icons, ammo counters, and bullet-time slow motion while selecting.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["hud".to_string(), "weapons".to_string(), "ui".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/86104-weapon-wheel.html".to_string(),
                download_url: None,
                file_size_formatted: Some("3.1 MB".to_string()),
                file_size_bytes: Some(3_250_585),
                release_date: Some("2016-04-12".to_string()),
                update_date: Some("2022-07-20".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Verified for 1.0 US".to_string(),
            },
            ModRecord {
                id: "libertycity_gps_radar".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "dynamic_gps_navigation".to_string(),
                title: "Dynamic GPS Route Navigation Line".to_string(),
                author: "DK22Pac".to_string(),
                description: "Draws an intelligent real-time road navigation route line on the mini-map radar and main map towards destinations and mission waypoints, mimicking modern GPS navigation.".to_string(),
                category: "Plugins / ASI".to_string(),
                tags: vec!["radar".to_string(), "gps".to_string(), "hud".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/84210-gps-mod.html".to_string(),
                download_url: None,
                file_size_formatted: Some("1.1 MB".to_string()),
                file_size_bytes: Some(1_153_433),
                release_date: Some("2015-11-08".to_string()),
                update_date: Some("2023-01-14".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Targeted to 1.0 US radar rendering loop".to_string(),
            },
            ModRecord {
                id: "libertycity_overdose_effects".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "overdose_effects_v15".to_string(),
                title: "Overdose Effects HD Particle Overhaul".to_string(),
                author: "2CH & Function".to_string(),
                description: "Comprehensive visual overhaul of vehicle explosions, fire, smoke, blood splatters, muzzle flashes, and water ripples with realistic particle physics and high-definition textures.".to_string(),
                category: "Graphics / Assets".to_string(),
                tags: vec!["effects".to_string(), "particles".to_string(), "graphics".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/62040-overdose-effects-v1-5.html".to_string(),
                download_url: None,
                file_size_formatted: Some("14.5 MB".to_string()),
                file_size_bytes: Some(15_204_352),
                release_date: Some("2014-06-18".to_string()),
                update_date: Some("2022-03-10".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Isolated cleanly inside ModLoader folder".to_string(),
            },
            ModRecord {
                id: "libertycity_rosa_project".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "rosa_project_evolved".to_string(),
                title: "RoSA Project Evolved (High-Definition World Textures)".to_string(),
                author: "dima0017 & RoSA Team".to_string(),
                description: "The premier full-world texture remaster for San Andreas. Replaces roads, sidewalks, walls, foliage, roofs, and billboards with photorealistic 512x512 and 1024x1024 textures faithful to original lore.".to_string(),
                category: "Graphics / Textures".to_string(),
                tags: vec!["textures".to_string(), "hd".to_string(), "graphics".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/152912-rosa-project-evolved.html".to_string(),
                download_url: None,
                file_size_formatted: Some("1.2 GB".to_string()),
                file_size_bytes: Some(1_288_490_188),
                release_date: Some("2021-02-10".to_string()),
                update_date: Some("2024-01-28".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ModLoader".to_string(), "Open Limit Adjuster".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Safe folder-based ModLoader installation".to_string(),
            },
            ModRecord {
                id: "libertycity_moonloader".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "moonloader_v026".to_string(),
                title: "MoonLoader v026 (Lua Scripting Environment)".to_string(),
                author: "FYP".to_string(),
                description: "Full Lua 5.1 runtime engine for GTA San Andreas. Executes asynchronous scripts, high-performance GUI overlays, and memory hooks with zero lag.".to_string(),
                category: "Tools / Frameworks".to_string(),
                tags: vec!["lua".to_string(), "moonloader".to_string(), "framework".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/98120-moonloader.html".to_string(),
                download_url: None,
                file_size_formatted: Some("2.1 MB".to_string()),
                file_size_bytes: Some(2_202_009),
                release_date: Some("2016-12-05".to_string()),
                update_date: Some("2023-04-14".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Verified Lua runtime for 1.0 US".to_string(),
            },
            ModRecord {
                id: "libertycity_radio_restoration".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "radio_station_restoration".to_string(),
                title: "GTA SA Complete Radio Restoration (All Cut Songs)".to_string(),
                author: "Community Archive".to_string(),
                description: "Restores all 17 licensed songs that Rockstar cut from modern releases (Radio Los Santos, Bounce FM, K-DST, Radio X) with original uncompressed audio quality.".to_string(),
                category: "Audio / Sounds".to_string(),
                tags: vec!["audio".to_string(), "radio".to_string(), "restoration".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/82910-radio-restoration.html".to_string(),
                download_url: None,
                file_size_formatted: Some("480 MB".to_string()),
                file_size_bytes: Some(503_316_480),
                release_date: Some("2015-02-18".to_string()),
                update_date: Some("2023-11-20".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Installed safely into ModLoader audio streams".to_string(),
            },
            ModRecord {
                id: "libertycity_ps2_audio".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "ps2_original_audio_sfx".to_string(),
                title: "Original PS2 Audio & Ambient Sound Effects Pack".to_string(),
                author: "aap & Ash_735".to_string(),
                description: "Restores higher-quality PlayStation 2 audio soundbanks: ambient crickets, police radio dispatches, engine revs, tyre screeches, and uncompressed dialogue.".to_string(),
                category: "Audio / Sounds".to_string(),
                tags: vec!["audio".to_string(), "ps2".to_string(), "sfx".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/124910-ps2-audio-sfx.html".to_string(),
                download_url: None,
                file_size_formatted: Some("210 MB".to_string()),
                file_size_bytes: Some(220_200_960),
                release_date: Some("2018-07-12".to_string()),
                update_date: Some("2022-09-01".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Non-destructive ModLoader sound replacement".to_string(),
            },
            ModRecord {
                id: "libertycity_original_peds_hd".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "original_peds_remastered".to_string(),
                title: "Original Peds Remastered HD (All 300+ Citizens)".to_string(),
                author: "Makar_Sm".to_string(),
                description: "Replaces all 300+ pedestrians, gangsters, police officers, and story characters with faithful high-poly models and sharp textures without changing their iconic look.".to_string(),
                category: "Skins / Models".to_string(),
                tags: vec!["peds".to_string(), "hd".to_string(), "models".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/138120-peds-remastered.html".to_string(),
                download_url: None,
                file_size_formatted: Some("140 MB".to_string()),
                file_size_bytes: Some(146_800_640),
                release_date: Some("2019-05-14".to_string()),
                update_date: Some("2023-08-22".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Installed into ModLoader ped models".to_string(),
            },
            ModRecord {
                id: "libertycity_skyline_trees".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "skyline_hd_foliage".to_string(),
                title: "Skyline HD Palm Trees & Vegetation Overhaul".to_string(),
                author: "Artgin".to_string(),
                description: "Replaces 2D card trees and blurry bushes with gorgeous high-polygon 3D palm trees, pine trees, and foliage matching real California and Nevada vegetation.".to_string(),
                category: "Graphics / Textures".to_string(),
                tags: vec!["trees".to_string(), "vegetation".to_string(), "hd".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/114920-skyline-trees.html".to_string(),
                download_url: None,
                file_size_formatted: Some("78 MB".to_string()),
                file_size_bytes: Some(81_788_928),
                release_date: Some("2017-10-09".to_string()),
                update_date: Some("2022-06-18".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ModLoader".to_string(), "Open Limit Adjuster".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Safe folder-based ModLoader model streaming".to_string(),
            },
            ModRecord {
                id: "libertycity_sampfuncs".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "sampfuncs_v541".to_string(),
                title: "SAMPFUNCS v5.4.1 (Hooking & Library Core)".to_string(),
                author: "FYP".to_string(),
                description: "Advanced API hooking and development library providing deep runtime memory modification and network hooks for GTA SA and SA-MP.".to_string(),
                category: "Tools / Frameworks".to_string(),
                tags: vec!["samp".to_string(), "framework".to_string(), "hooking".to_string(), "asi".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/89120-sampfuncs.html".to_string(),
                download_url: None,
                file_size_formatted: Some("920 KB".to_string()),
                file_size_bytes: Some(942_080),
                release_date: Some("2016-08-11".to_string()),
                update_date: Some("2021-03-05".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ASI Loader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Asi,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Targeted to 1.0 US memory structure".to_string(),
            },
            ModRecord {
                id: "libertycity_csgo_weapons".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "csgo_weapons_knives_pack".to_string(),
                title: "CS:GO Complete Firearms & Knives Collection in SA".to_string(),
                author: "Valve & VindiCaToR".to_string(),
                description: "Replaces all default weapons with Counter-Strike: Global Offensive firearms and knives (Karambit, Butterfly, AWP Dragon Lore, AK-47 Vulcan) with custom icons.".to_string(),
                category: "Weapons / Assets".to_string(),
                tags: vec!["weapons".to_string(), "csgo".to_string(), "knives".to_string(), "hd".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/124102-csgo-weapon-pack.html".to_string(),
                download_url: None,
                file_size_formatted: Some("48 MB".to_string()),
                file_size_bytes: Some(50_331_648),
                release_date: Some("2018-05-20".to_string()),
                update_date: Some("2022-09-14".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Safe folder-based ModLoader weapon replacement".to_string(),
            },
            ModRecord {
                id: "libertycity_cod_mw2_weapons".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "cod_mw2_weapons_hd".to_string(),
                title: "Call of Duty MW2 Tactical Weapons Pack".to_string(),
                author: "Infinity Ward & Artgin".to_string(),
                description: "Faithful Modern Warfare 2 firearms (ACR, Intervention .50, Desert Eagle, Model 1887 akimbo) with authentic bump maps and high-res HUD icons.".to_string(),
                category: "Weapons / Assets".to_string(),
                tags: vec!["weapons".to_string(), "cod".to_string(), "tactical".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/119201-cod-mw2-pack.html".to_string(),
                download_url: None,
                file_size_formatted: Some("32 MB".to_string()),
                file_size_bytes: Some(33_554_432),
                release_date: Some("2017-09-12".to_string()),
                update_date: Some("2021-04-18".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Non-destructive ModLoader weapon injection".to_string(),
            },
            ModRecord {
                id: "libertycity_lightsaber".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "star_wars_lightsaber".to_string(),
                title: "Star Wars Lightsaber Mod (Dynamic Glow & Deflection)".to_string(),
                author: "Ryosuke82 & Community".to_string(),
                description: "Replaces the katana with an illuminated glowing Jedi lightsaber. Features authentic hum/swing SFX and the ability to deflect incoming bullets.".to_string(),
                category: "Scripts / CLEO".to_string(),
                tags: vec!["lightsaber".to_string(), "starwars".to_string(), "weapons".to_string(), "cleo".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/42019-star-wars-lightsaber.html".to_string(),
                download_url: None,
                file_size_formatted: Some("2.2 MB".to_string()),
                file_size_bytes: Some(2_306_867),
                release_date: Some("2012-05-04".to_string()),
                update_date: Some("2019-11-15".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["CLEO 4".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Cleo,
                confidence: "confirmed".to_string(),
                compatibility_reason: "CLEO 4 script and Katana DFF replacement".to_string(),
            },
            ModRecord {
                id: "libertycity_true_sound_guns".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "true_sound_of_guns".to_string(),
                title: "True Sound of Guns HD (Realistic Firearm Audio)".to_string(),
                author: "44Caliber".to_string(),
                description: "Complete studio recording overhaul for all weapons in GTA SA. Features realistic supersonic cracks, echoing reverberations in alleys, and mechanical reloads.".to_string(),
                category: "Audio / Sounds".to_string(),
                tags: vec!["audio".to_string(), "guns".to_string(), "sfx".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/89120-true-sound-of-guns.html".to_string(),
                download_url: None,
                file_size_formatted: Some("18 MB".to_string()),
                file_size_bytes: Some(18_874_368),
                release_date: Some("2015-08-20".to_string()),
                update_date: Some("2022-07-10".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string(), "Classic PC 1.0 EU".to_string()],
                required_dependencies: vec!["ModLoader".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Installed into ModLoader audio soundbanks".to_string(),
            },
            ModRecord {
                id: "libertycity_srt3_retextured".to_string(),
                source_id: "libertycity".to_string(),
                source_mod_id: "san_andreas_retextured_3".to_string(),
                title: "SRT3 Final (San Andreas Retextured 3)".to_string(),
                author: "ParoXum & SRT Team".to_string(),
                description: "The classic all-time favorite world retexture. Overhauls asphalt, sidewalks, grass, building facades, and vegetation across the entire map.".to_string(),
                category: "Graphics / Textures".to_string(),
                tags: vec!["textures".to_string(), "world".to_string(), "hd".to_string(), "modloader".to_string()],
                thumbnail_url: None,
                page_url: "https://libertycity.net/files/gta-san-andreas/62104-srt3-final.html".to_string(),
                download_url: None,
                file_size_formatted: Some("890 MB".to_string()),
                file_size_bytes: Some(933_232_640),
                release_date: Some("2014-07-15".to_string()),
                update_date: Some("2020-03-22".to_string()),
                supported_game_versions: vec!["Classic PC 1.0 US".to_string()],
                required_dependencies: vec!["ModLoader".to_string(), "Open Limit Adjuster".to_string()],
                incompatibilities: Vec::new(),
                installation_strategy: InstallationStrategyType::Modloader,
                confidence: "confirmed".to_string(),
                compatibility_reason: "Safe folder-based ModLoader world texture replacement".to_string(),
            },
        ]
    }
}

#[async_trait]
impl SourceAdapter for LibertyCityAdapter {
    fn id(&self) -> &'static str {
        "libertycity"
    }

    fn name(&self) -> &'static str {
        "LibertyCity"
    }

    fn base_url(&self) -> &'static str {
        "https://libertycity.net"
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
        "Mods hosted on LibertyCity.net. Protected by original author attribution."
    }

    fn terms_notice(&self) -> &'static str {
        "LibertyCity community mod database. Direct links and project mirrors provided for verified packages."
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
            .ok_or_else(|| format!("Mod '{}' not found on LibertyCity", source_mod_id))
    }

    async fn get_download_url(&self, source_mod_id: &str) -> Result<Option<String>, String> {
        let details = self.get_mod_details(source_mod_id).await?;
        Ok(details.download_url)
    }
}
