use std::path::Path;
use std::process::Command;
use crate::models::GameInstallation;

#[cfg(target_os = "windows")]
mod windows_launch {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::{null, null_mut};
    use std::path::Path;

    #[repr(C)]
    struct SHELLEXECUTEINFOW {
        cb_size: u32,
        f_mask: u32,
        hwnd: *mut std::ffi::c_void,
        lp_verb: *const u16,
        lp_file: *const u16,
        lp_parameters: *const u16,
        lp_directory: *const u16,
        n_show: i32,
        h_inst_app: *mut std::ffi::c_void,
        lp_id_list: *mut std::ffi::c_void,
        lp_class: *const u16,
        hkey_class: *mut std::ffi::c_void,
        dw_hot_key: u32,
        h_icon_or_monitor: *mut std::ffi::c_void,
        h_process: *mut std::ffi::c_void,
    }

    #[link(name = "shell32")]
    extern "system" {
        fn ShellExecuteExW(p_exec_info: *mut SHELLEXECUTEINFOW) -> i32;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetProcessId(h_process: *mut std::ffi::c_void) -> u32;
        fn WaitForSingleObject(h_handle: *mut std::ffi::c_void, dw_milliseconds: u32) -> u32;
        fn GetExitCodeProcess(h_process: *mut std::ffi::c_void, lp_exit_code: *mut u32) -> i32;
        fn CloseHandle(h_object: *mut std::ffi::c_void) -> i32;
    }

    const SEE_MASK_NOCLOSEPROCESS: u32 = 0x00000040;
    const SW_SHOWNORMAL: i32 = 1;
    const WAIT_TIMEOUT: u32 = 0x00000102;
    const STILL_ACTIVE: u32 = 259;

    pub fn launch_via_shell(
        exe_path: &Path,
        root_dir: &Path,
        args_str: &str,
    ) -> Result<u32, String> {
        let to_wide = |s: &OsStr| -> Vec<u16> {
            s.encode_wide().chain(std::iter::once(0)).collect()
        };

        let file_wide = to_wide(exe_path.as_os_str());
        let dir_wide = to_wide(root_dir.as_os_str());
        let params_wide = if !args_str.trim().is_empty() {
            Some(to_wide(OsStr::new(args_str.trim())))
        } else {
            None
        };

        let mut sei = SHELLEXECUTEINFOW {
            cb_size: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
            f_mask: SEE_MASK_NOCLOSEPROCESS,
            hwnd: null_mut(),
            lp_verb: null(),
            lp_file: file_wide.as_ptr(),
            lp_parameters: params_wide.as_ref().map(|v| v.as_ptr()).unwrap_or(null()),
            lp_directory: dir_wide.as_ptr(),
            n_show: SW_SHOWNORMAL,
            h_inst_app: null_mut(),
            lp_id_list: null_mut(),
            lp_class: null(),
            hkey_class: null_mut(),
            dw_hot_key: 0,
            h_icon_or_monitor: null_mut(),
            h_process: null_mut(),
        };

        let success = unsafe { ShellExecuteExW(&mut sei) };
        if success == 0 || sei.h_process.is_null() {
            return Err("Failed to launch game process via ShellExecuteExW".to_string());
        }

        let pid = unsafe { GetProcessId(sei.h_process) };

        // Check if the process crashed or exited immediately on boot
        // Wait up to 2000ms to verify game engine window creation and D3D device setup
        let wait_result = unsafe { WaitForSingleObject(sei.h_process, 2000) };
        if wait_result != WAIT_TIMEOUT {
            let mut exit_code: u32 = 0;
            unsafe {
                GetExitCodeProcess(sei.h_process, &mut exit_code);
                CloseHandle(sei.h_process);
            }
            if exit_code != STILL_ACTIVE {
                return Err(format!(
                    "Game process terminated immediately upon launch (exit code: 0x{:X}). If the game closed silently, your 'gta_sa.set' configuration may be corrupted or an incompatible DLL hooked the process.",
                    exit_code
                ));
            }
        } else {
            unsafe {
                CloseHandle(sei.h_process);
            }
        }

        Ok(pid)
    }
}

/// Automatically auto-heal common mod file positioning prerequisites in the game folder
fn auto_heal_companion_prerequisites(root_dir: &Path) {
    let modloader_dir = root_dir.join("modloader");
    if !modloader_dir.is_dir() {
        return;
    }

    // 1. Isolate Cheat Menu mod entirely
    // CheatMenuSA.asi triggers D3D9 vtable collisions, and its internal CheatMenuSA/clothes.txd and vehicles.txd
    // intercept ModLoader's streaming and replace vanilla clothing/vehicles with thumbnail icons,
    // which triggers an immediate 0x007F39FB crash in RwTexDictionaryFindNamedTexture during character init.
    let root_cheat = root_dir.join("CheatMenuSA.asi");
    if root_cheat.is_file() {
        let _ = std::fs::rename(&root_cheat, root_dir.join("CheatMenuSA.asi.disabled"));
    }
    let ml_cheat_dir = modloader_dir.join("Cheat_Menu_Ultimate_In-Game_Trainer_GUI");
    if ml_cheat_dir.is_dir() {
        let ml_asi = ml_cheat_dir.join("CheatMenuSA.asi");
        if ml_asi.is_file() {
            let _ = std::fs::rename(&ml_asi, ml_cheat_dir.join("CheatMenuSA.asi.disabled"));
        }
    }

    // 2. Scan and neutralize any rogue clothes.txd or hud.txd in modloader that replace core game assets
    fn sanitize_rogue_txds(dir: &Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let folder_name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if !folder_name.ends_with(".disabled") {
                        sanitize_rogue_txds(&p);
                    }
                } else if p.is_file() {
                    let file_name = p.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                    if file_name == "clothes.txd" {
                        let path_str = p.to_string_lossy().to_lowercase();
                        if path_str.contains("cheat") || path_str.contains("menu") || path_str.contains("trainer") {
                            let disabled_p = p.with_file_name(format!("{}.disabled", p.file_name().unwrap_or_default().to_string_lossy()));
                            let _ = std::fs::rename(&p, &disabled_p);
                        }
                    }
                }
            }
        }
    }
    sanitize_rogue_txds(&modloader_dir);

    // 3. Fix V-HUD folder link in root directory
    let vhud_src = modloader_dir
        .join("GTA_V_Style_High-Definition_Radar__Minimap")
        .join("VHud");
    let vhud_dst = root_dir.join("VHud");
    if vhud_src.is_dir() && !vhud_dst.exists() {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("powershell")
                .args(&[
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    &format!(
                        "New-Item -ItemType Junction -Path '{}' -Target '{}' -ErrorAction SilentlyContinue",
                        vhud_dst.to_string_lossy().replace('\'', "''"),
                        vhud_src.to_string_lossy().replace('\'', "''")
                    ),
                ])
                .output();
        }
    }

    // 4. Fix bass.dll in game root if older or missing
    let vhud_bass = modloader_dir
        .join("GTA_V_Style_High-Definition_Radar__Minimap")
        .join("bass.dll");
    let root_bass = root_dir.join("bass.dll");
    if vhud_bass.is_file() {
        let needs_copy = if let Ok(meta) = std::fs::metadata(&root_bass) {
            meta.len() < 100_000 // Old 2010 BASS.dll is 92KB, modern is 129KB
        } else {
            true
        };
        if needs_copy {
            let _ = std::fs::copy(&vhud_bass, &root_bass);
        }
    }

    // 5. Fix msvcr100d.dll runtime in game root if companion needs it
    let fp_msvcr = modloader_dir
        .join("First_Person_Mod_v3_0_FIXED")
        .join("msvcr100d.dll");
    let root_msvcr = root_dir.join("msvcr100d.dll");
    if fp_msvcr.is_file() && !root_msvcr.exists() {
        let _ = std::fs::copy(&fp_msvcr, &root_msvcr);
    }

    // 6. Detect and resolve duplicate ASI loaders in game root
    let dinput8 = root_dir.join("dinput8.dll");
    let vorbis = root_dir.join("vorbisFile.dll");
    if dinput8.is_file() && vorbis.is_file() {
        // Check if dinput8 is an Ultimate ASI Loader duplicate
        if let Ok(content) = std::fs::read(&dinput8) {
            let content_str = String::from_utf8_lossy(&content);
            if content_str.contains("Ultimate-ASI-Loader") || content_str.contains("Ultimate ASI Loader") {
                let backup_path = root_dir.join("dinput8.dll.duplicate_bak");
                let _ = std::fs::rename(&dinput8, &backup_path);
            }
        }
    }

    // 7. Detect and isolate third-party SA-MP hooks that intercept and abort singleplayer
    let winmm = root_dir.join("winmm.dll");
    if winmm.is_file() {
        if let Ok(content) = std::fs::read(&winmm) {
            let content_str = String::from_utf8_lossy(&content);
            if content_str.contains("sa-mp") || content_str.contains("samp") || content_str.contains("masterlist") {
                let backup_path = root_dir.join("winmm.dll.samp_bak");
                let _ = std::fs::rename(&winmm, &backup_path);
            }
        }
    }

    // 8. Disable legacy DirectShow intro movies on Windows 11 to avoid video focus abort
    let movies_dir = root_dir.join("movies");
    if movies_dir.is_dir() {
        for movie_name in &["Logo.mpg", "GTAtitles.mpg"] {
            let movie_file = movies_dir.join(movie_name);
            if movie_file.is_file() {
                let disabled_file = movies_dir.join(format!("{}.disabled", movie_name));
                let _ = std::fs::rename(&movie_file, &disabled_file);
            }
        }
    }

    // 9. Centrally manage essentials in scripts/ and neutralize duplicates in root and modloader
    // SilentPatch, WidescreenFix, WindowedMode, and FramerateVigilante MUST load early via scripts/
    // so they hook Direct3D9 device creation before ModLoader initializes.
    let scripts_dir = root_dir.join("scripts");
    let essentials_dir = modloader_dir.join("_ESSENTIALS");
    let _ = std::fs::create_dir_all(&scripts_dir);

    let essential_names = [
        "SilentPatchSA.asi",
        "GTASA.WidescreenFix.asi",
        "III.VC.SA.WindowedMode.asi",
        "FramerateVigilante.SA.asi",
    ];

    for asi_name in &essential_names {
        let script_target = scripts_dir.join(asi_name);
        if !script_target.is_file() {
            // Restore from duplicate_bak or modloader
            let candidates = [
                scripts_dir.join(format!("{}.duplicate_bak", asi_name)),
                scripts_dir.join(format!("{}.disabled", asi_name)),
                root_dir.join(asi_name),
                root_dir.join(format!("{}.duplicate_bak", asi_name)),
            ];
            for cand in &candidates {
                if cand.is_file() {
                    let _ = std::fs::copy(cand, &script_target);
                    break;
                }
            }
        }

        // Neutralize root copy so only scripts/ loads it
        let root_copy = root_dir.join(asi_name);
        if root_copy.is_file() {
            let backup_path = root_dir.join(format!("{}.duplicate_bak", asi_name));
            let _ = std::fs::rename(&root_copy, &backup_path);
        }
    }

    // Disable duplicate essential ASIs inside modloader/_ESSENTIALS so ModLoader doesn't load them twice
    if essentials_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&essentials_dir) {
            for entry in entries.flatten() {
                let sub_path = entry.path();
                if sub_path.is_dir() {
                    if let Ok(sub_entries) = std::fs::read_dir(&sub_path) {
                        for se in sub_entries.flatten() {
                            let p = se.path();
                            if p.is_file() {
                                if let Some(ext) = p.extension() {
                                    if ext.eq_ignore_ascii_case("asi") {
                                        let disabled_path = p.with_file_name(format!("{}.disabled", p.file_name().unwrap_or_default().to_string_lossy()));
                                        let _ = std::fs::rename(&p, &disabled_path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 10. Auto-repair Skimmer vehicle definition in data/vehicles.ide (Windows 11 24H2 crash fix)
    let vehicles_ide = root_dir.join("data").join("vehicles.ide");
    if vehicles_ide.is_file() {
        if let Ok(content) = std::fs::read_to_string(&vehicles_ide) {
            if content.contains("460, \tskimmer,\tskimmer, \tplane,\t\tSEAPLANE,\tSKIMMER,\tnull,\tignore,\t\t5,\t0,\t0\r\n")
                || content.contains("460, \tskimmer,\tskimmer, \tplane,\t\tSEAPLANE,\tSKIMMER,\tnull,\tignore,\t\t5,\t0,\t0\n")
            {
                let repaired = content.replace(
                    "460, \tskimmer,\tskimmer, \tplane,\t\tSEAPLANE,\tSKIMMER,\tnull,\tignore,\t\t5,\t0,\t0",
                    "460, \tskimmer,\tskimmer, \tplane,\t\tSEAPLANE,\tSKIMMER,\tnull,\tignore,\t\t5,\t0,\t0,\t\t-1, 0.7, 0.7,\t\t-1",
                );
                let _ = std::fs::write(&vehicles_ide, repaired);
            }
        }
    }

    // 11. Ensure Windows 11 DPI awareness manifest and AppCompat flags
    let manifest_path = root_dir.join("gta_sa.exe.manifest");
    if !manifest_path.is_file() {
        let manifest_content = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2,PerMonitor</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>"#;
        let _ = std::fs::write(&manifest_path, manifest_content);
    }

    #[cfg(target_os = "windows")]
    {
        let exe_path = root_dir.join("gta_sa.exe");
        let exe_path_str = exe_path.to_string_lossy().to_string();
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let _ = std::process::Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &format!(
                    "Set-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers' -Name '{}' -Value '~ HIGHDPIAWARE $ IgnoreFreeLibrary<vhud>' -ErrorAction SilentlyContinue",
                    exe_path_str.replace('\'', "''")
                ),
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
    }

    // 12. Quarantine SkyGfx and Project 2DFX to eliminate black shader anomalies on Intel UHD / DXVK
    let skygfx_dir = modloader_dir.join("SkyGfx_42b_PS2__Xbox_Graphical_Pipelines");
    if skygfx_dir.is_dir() {
        let _ = std::fs::rename(&skygfx_dir, modloader_dir.join("SkyGfx_42b_PS2__Xbox_Graphical_Pipelines.disabled"));
    }
    let p2dfx_dir = modloader_dir.join("Project_2DFX_Max_Draw_Distance__LODs");
    if p2dfx_dir.is_dir() {
        let _ = std::fs::rename(&p2dfx_dir, modloader_dir.join("Project_2DFX_Max_Draw_Distance__LODs.disabled"));
    }

    // 13. Ensure VHud.asi is disabled to prevent d3dx9_43 shader compiler access violations with DXVK
    let vhud_asi = modloader_dir
        .join("GTA_V_Style_High-Definition_Radar__Minimap")
        .join("VHud.asi");
    if vhud_asi.is_file() {
        let _ = std::fs::rename(&vhud_asi, modloader_dir.join("GTA_V_Style_High-Definition_Radar__Minimap").join("VHud.asi.disabled"));
    }

    // 14. Ensure DXVK configuration for Intel UHD / modern Windows display adapter compatibility
    // ModeCountCompatibility must NEVER be true: it truncates reported display modes to 16, dropping 1920x1080!
    let dxvk_conf = root_dir.join("dxvk.conf");
    let dxvk_required = "d3d9.forceAspectRatio = 16:9\ndxvk.allowFse = False\n";
    let _ = std::fs::write(&dxvk_conf, dxvk_required);

    // 15. Quarantine First Person Mod to prevent cutscene camera entrapment and black interiors
    let fp_dir = modloader_dir.join("First_Person_Mod_v3_0_FIXED");
    if fp_dir.is_dir() {
        let _ = std::fs::rename(&fp_dir, modloader_dir.join("First_Person_Mod_v3_0_FIXED.disabled"));
    }
}

fn update_or_append_ini_key(content: &mut String, key: &str, value: &str) {
    let prefix = format!("{} =", key);
    let prefix_space = format!("{}=", key);
    let mut replaced = false;
    let mut new_lines = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&prefix) || trimmed.starts_with(&prefix_space) {
            new_lines.push(format!("{} = {}", key, value));
            replaced = true;
        } else {
            new_lines.push(line.to_string());
        }
    }
    if !replaced {
        if !new_lines.iter().any(|l| l.trim().starts_with("[MAIN]") || l.trim().starts_with("[")) {
            new_lines.insert(0, "[MAIN]".to_string());
        }
        new_lines.push(format!("{} = {}", key, value));
    }
    *content = new_lines.join("\r\n");
}

fn scan_and_disable_oversized_loadscs(dir: &Path) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_and_disable_oversized_loadscs(&path);
            } else if path.is_file() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                if file_name.eq_ignore_ascii_case("LOADSCS.txd") {
                    if let Ok(meta) = path.metadata() {
                        if meta.len() > 10_000_000 {
                            let disabled_path = path.with_file_name(format!("{}.disabled", file_name));
                            let _ = std::fs::rename(&path, &disabled_path);
                        }
                    }
                }
            }
        }
    }
}

pub fn configure_widescreen_and_display(root_dir: &Path, width: u32, height: u32, windowed: bool) {
    let scripts_dir = root_dir.join("scripts");
    let modloader_dir = root_dir.join("modloader");
    let _ = std::fs::create_dir_all(&scripts_dir);

    // 1. Ensure scripts/GTASA.WidescreenFix.asi is active
    let script_ws_asi = scripts_dir.join("GTASA.WidescreenFix.asi");
    if !script_ws_asi.is_file() {
        let candidates = [
            scripts_dir.join("GTASA.WidescreenFix.asi.duplicate_bak"),
            root_dir.join("GTASA.WidescreenFix.asi"),
            root_dir.join("GTASA.WidescreenFix.asi.duplicate_bak"),
            modloader_dir.join("_ESSENTIALS").join("Widescreen Fix by ThirteenAG").join("GTASA.WidescreenFix.asi"),
            modloader_dir.join("_ESSENTIALS").join("Widescreen Fix by ThirteenAG").join("GTASA.WidescreenFix.asi.disabled"),
        ];
        for cand in &candidates {
            if cand.is_file() {
                let _ = std::fs::copy(cand, &script_ws_asi);
                break;
            }
        }
    }

    // 2. Ensure scripts/III.VC.SA.WindowedMode.asi is active
    let script_wm_asi = scripts_dir.join("III.VC.SA.WindowedMode.asi");
    if !script_wm_asi.is_file() {
        let candidates = [
            scripts_dir.join("III.VC.SA.WindowedMode.asi.duplicate_bak"),
            root_dir.join("III.VC.SA.WindowedMode.asi"),
            root_dir.join("III.VC.SA.WindowedMode.asi.duplicate_bak"),
            modloader_dir.join("_ESSENTIALS").join("Windowed Mode").join("III.VC.SA.WindowedMode.asi"),
            modloader_dir.join("_ESSENTIALS").join("Windowed Mode").join("III.VC.SA.WindowedMode.asi.disabled"),
        ];
        for cand in &candidates {
            if cand.is_file() {
                let _ = std::fs::copy(cand, &script_wm_asi);
                break;
            }
        }
    }

    // 3. Neutralize duplicates outside scripts/
    let root_wm = root_dir.join("III.VC.SA.WindowedMode.asi");
    if root_wm.is_file() {
        let _ = std::fs::rename(&root_wm, root_dir.join("III.VC.SA.WindowedMode.asi.duplicate_bak"));
    }
    let ml_wm = modloader_dir.join("_ESSENTIALS").join("Windowed Mode").join("III.VC.SA.WindowedMode.asi");
    if ml_wm.is_file() {
        let _ = std::fs::rename(&ml_wm, modloader_dir.join("_ESSENTIALS").join("Windowed Mode").join("III.VC.SA.WindowedMode.asi.disabled"));
    }

    let root_ws = root_dir.join("GTASA.WidescreenFix.asi");
    if root_ws.is_file() {
        let _ = std::fs::rename(&root_ws, root_dir.join("GTASA.WidescreenFix.asi.duplicate_bak"));
    }
    let ml_ws = modloader_dir.join("_ESSENTIALS").join("Widescreen Fix by ThirteenAG").join("GTASA.WidescreenFix.asi");
    if ml_ws.is_file() {
        let _ = std::fs::rename(&ml_ws, modloader_dir.join("_ESSENTIALS").join("Widescreen Fix by ThirteenAG").join("GTASA.WidescreenFix.asi.disabled"));
    }

    // 4. Update GTASA.WidescreenFix.ini across scripts, root, and modloader
    let ws_ini_targets = [
        scripts_dir.join("GTASA.WidescreenFix.ini"),
        root_dir.join("GTASA.WidescreenFix.ini"),
        modloader_dir.join("_ESSENTIALS").join("Widescreen Fix by ThirteenAG").join("GTASA.WidescreenFix.ini"),
    ];
    let res_x = if width == 0 || width == 1536 { "0".to_string() } else { width.to_string() };
    let res_y = if height == 0 || height == 864 { "0".to_string() } else { height.to_string() };
    for ini_path in &ws_ini_targets {
        let mut content = std::fs::read_to_string(ini_path).unwrap_or_else(|_| "[MAIN]\r\n".to_string());
        update_or_append_ini_key(&mut content, "ResX", &res_x);
        update_or_append_ini_key(&mut content, "ResY", &res_y);
        update_or_append_ini_key(&mut content, "ForceAspectRatio", "auto");
        update_or_append_ini_key(&mut content, "FrontendAspectRatio", "auto");
        update_or_append_ini_key(&mut content, "AllowAltTabbingWithoutPausing", "1");
        update_or_append_ini_key(&mut content, "SmartCutsceneBorders", "0");
        let _ = std::fs::write(ini_path, content);
    }

    // 5. Update III.VC.SA.WindowedMode.ini across scripts, root, and modloader
    // Mode 2 provides borderless fullscreen with automatic display resolution
    let wm_ini_content = format!(
        "[window]\r\nmode = {}\r\nmaximized = 1\r\npositionX = 0\r\npositionY = 0\r\nresolutionX = 0\r\nresolutionY = 0\r\n\r\n[game]\r\nmenuFPS = 0\r\nautoPause = 0\r\nautoResume = 1\r\n\r\n[MAIN]\r\nShowBorder = {}\r\nShowMenu = 0\r\n",
        if windowed { 1 } else { 2 },
        if windowed { "1" } else { "0" }
    );
    let wm_ini_targets = [
        scripts_dir.join("III.VC.SA.WindowedMode.ini"),
        root_dir.join("III.VC.SA.WindowedMode.ini"),
        modloader_dir.join("_ESSENTIALS").join("Windowed Mode").join("III.VC.SA.WindowedMode.ini"),
    ];
    for ini_path in &wm_ini_targets {
        let _ = std::fs::write(ini_path, &wm_ini_content);
    }

    // 6. Ensure dxvk.conf disallows exclusive fullscreen mode-switch crashes
    let dxvk_conf = root_dir.join("dxvk.conf");
    let dxvk_content = "d3d9.forceAspectRatio = 16:9\ndxvk.allowFse = False\n";
    let _ = std::fs::write(&dxvk_conf, dxvk_content);

    // 7. Ensure modloader.ini ignores .disabled mods
    let ml_ini = modloader_dir.join("modloader.ini");
    if ml_ini.is_file() {
        if let Ok(mut text) = std::fs::read_to_string(&ml_ini) {
            if !text.contains("*.disabled") {
                if text.contains("_ignore") {
                    text = text.replace("_ignore", "_ignore\r\n*.disabled\r\n*.disabled*");
                } else if text.contains("[Profiles.Default.IgnoreMods]") {
                    text = text.replace("[Profiles.Default.IgnoreMods]", "[Profiles.Default.IgnoreMods]\r\n*.disabled\r\n*.disabled*");
                }
                let _ = std::fs::write(&ml_ini, text);
            }
        }
    }

    // 8. Neutralize oversized LOADSCS.txd in modloader (> 10MB) to restore instant loading
    if modloader_dir.is_dir() {
        scan_and_disable_oversized_loadscs(&modloader_dir);
    }
}

pub fn launch_game(
    game: &GameInstallation,
    custom_args: &[String],
) -> Result<u32, String> {
    let root_dir = Path::new(&game.root_path);
    if !root_dir.exists() {
        return Err(format!("Game root directory '{}' does not exist", game.root_path));
    }

    // Auto-heal mod companion dependencies before launch
    auto_heal_companion_prerequisites(root_dir);

    // Extract target display parameters from custom args
    let mut width: u32 = 1920;
    let mut height: u32 = 1080;
    let mut is_windowed = false;

    #[cfg(target_os = "windows")]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetSystemMetrics(nIndex: i32) -> i32;
        }
        let screen_w = unsafe { GetSystemMetrics(0) }; // SM_CXSCREEN
        let screen_h = unsafe { GetSystemMetrics(1) }; // SM_CYSCREEN
        if screen_w > 0 && screen_h > 0 {
            width = screen_w as u32;
            height = screen_h as u32;
        }
    }

    let mut i = 0;
    while i < custom_args.len() {
        let arg = &custom_args[i];
        if arg == "-window" || arg == "-w" {
            is_windowed = true;
        } else if arg == "-width" && i + 1 < custom_args.len() {
            if let Ok(w) = custom_args[i + 1].parse::<u32>() {
                width = w;
            }
            i += 1;
        } else if arg == "-height" && i + 1 < custom_args.len() {
            if let Ok(h) = custom_args[i + 1].parse::<u32>() {
                height = h;
            }
            i += 1;
        }
        i += 1;
    }

    // Configure widescreen fix, windowed mode, and display engine
    configure_widescreen_and_display(root_dir, width, height, is_windowed);

    let exe_path = Path::new(&game.executable_path);
    if !exe_path.exists() {
        return Err(format!("Executable '{}' does not exist", game.executable_path));
    }

    // Build sanitized parameter list
    let mut sanitized_tokens = Vec::new();
    for arg_entry in custom_args {
        for token in arg_entry.split_whitespace() {
            if !token.contains('&') && !token.contains('|') && !token.contains(';') {
                sanitized_tokens.push(token.to_string());
            }
        }
    }

    let args_str = sanitized_tokens.join(" ");

    // Clean up any stale headless background process of gta_sa.exe to prevent single-instance mutex blockage
    #[cfg(target_os = "windows")]
    {
        let exe_name = exe_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "gta_sa.exe".to_string());
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let _ = std::process::Command::new("taskkill")
            .args(&["/F", "/IM", &exe_name])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        // Give OS a moment to release mutex
        std::thread::sleep(std::time::Duration::from_millis(150));
    }

    // Try primary launch via Windows ShellExecuteEx
    #[cfg(target_os = "windows")]
    {
        match windows_launch::launch_via_shell(exe_path, root_dir, &args_str) {
            Ok(pid) => return Ok(pid),
            Err(e) => {
                // If the game closed immediately, auto-recover corrupted gta_sa.set
                if let Some(doc_dir) = dirs::document_dir() {
                    let set_file = doc_dir.join("GTA San Andreas User Files").join("gta_sa.set");
                    if set_file.is_file() {
                        let bak_file = doc_dir.join("GTA San Andreas User Files").join("gta_sa.set.corrupt_bak");
                        let _ = std::fs::rename(&set_file, &bak_file);
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        if let Ok(pid) = windows_launch::launch_via_shell(exe_path, root_dir, &args_str) {
                            return Ok(pid);
                        }
                    }
                }
                if e.contains("terminated immediately") {
                    return Err(e);
                }
            }
        }
    }

    // Cross-platform Command fallback with detached process creation
    let mut cmd = Command::new(exe_path);
    cmd.current_dir(root_dir);

    for token in sanitized_tokens {
        cmd.arg(token);
    }

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x00000008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }

    let child = cmd.spawn().map_err(|e| format!("Failed to spawn game process: {}", e))?;
    Ok(child.id())
}
