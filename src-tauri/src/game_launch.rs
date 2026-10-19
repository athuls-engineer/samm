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

    // 1. Isolate CheatMenuSA.asi plugin if present
    // CheatMenuSA.asi triggers an immediate unhandled exception (0xC0000005) and stack overflow (0xC00000FD)
    // on modern Windows 11 due to Direct3D9 vtable collisions with radar/HUD hooks.
    let root_cheat = root_dir.join("CheatMenuSA.asi");
    if root_cheat.is_file() {
        let _ = std::fs::rename(&root_cheat, root_dir.join("CheatMenuSA.asi.disabled"));
    }
    let ml_cheat = modloader_dir
        .join("Cheat_Menu_Ultimate_In-Game_Trainer_GUI")
        .join("CheatMenuSA.asi");
    if ml_cheat.is_file() {
        let _ = std::fs::rename(
            &ml_cheat,
            modloader_dir
                .join("Cheat_Menu_Ultimate_In-Game_Trainer_GUI")
                .join("CheatMenuSA.asi.disabled"),
        );
    }

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
