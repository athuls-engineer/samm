use std::path::Path;
use std::process::Command;
use crate::models::GameInstallation;

pub fn launch_game(
    game: &GameInstallation,
    custom_args: &[String],
) -> Result<u32, String> {
    let exe_path = Path::new(&game.executable_path);
    if !exe_path.exists() {
        return Err(format!("Executable '{}' does not exist", game.executable_path));
    }

    let root_dir = Path::new(&game.root_path);
    if !root_dir.exists() {
        return Err(format!("Game root directory '{}' does not exist", game.root_path));
    }

    // Direct process spawn without shell invocation (prevents shell injection)
    let mut cmd = Command::new(exe_path);
    cmd.current_dir(root_dir);

    for arg_entry in custom_args {
        // Split on whitespace so compound flags and user-entered argument strings are passed as discrete tokens
        for token in arg_entry.split_whitespace() {
            // Sanitize arguments to avoid command injection
            if !token.contains('&') && !token.contains('|') && !token.contains(';') {
                cmd.arg(token);
            }
        }
    }

    let child = cmd.spawn().map_err(|e| format!("Failed to spawn game process: {}", e))?;
    Ok(child.id())
}
