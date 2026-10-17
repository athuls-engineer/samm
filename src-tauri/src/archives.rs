use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io;
use std::collections::HashSet;
use zip::ZipArchive;
use crate::models::{ArchiveEntry, ArchiveInspection};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// Security constraints
const MAX_TOTAL_UNCOMPRESSED_BYTES: u64 = 4 * 1024 * 1024 * 1024; // 4 GB
const MAX_FILE_ENTRIES: usize = 15_000;
const MAX_COMPRESSION_RATIO: f64 = 150.0;
const MAX_SINGLE_FILE_BYTES: u64 = 2 * 1024 * 1024 * 1024; // 2 GB

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn inspect_archive(archive_path: &Path) -> Result<ArchiveInspection, String> {
    if !archive_path.exists() {
        return Err(format!("Archive file not found: {:?}", archive_path));
    }

    let ext = archive_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let is_standalone = ["asi", "cs", "cleo", "dll", "ini", "dat"].contains(&ext.as_str());
    if is_standalone {
        let file_name = archive_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let size = std::fs::metadata(archive_path).map(|m| m.len()).unwrap_or(0);
        return Ok(ArchiveInspection {
            archive_path: archive_path.to_string_lossy().to_string(),
            format: ext.clone(),
            total_entries: 1,
            total_uncompressed_size: size,
            compression_ratio: 1.0,
            is_safe: true,
            safety_violations: Vec::new(),
            entries: vec![ArchiveEntry {
                path: file_name,
                size,
                compressed_size: size,
                crc32: "00000000".to_string(),
                is_directory: false,
                is_suspicious: false,
                suspicion_reason: None,
            }],
        });
    }

    let is_tar_candidate = ["7z", "rar", "tar", "gz", "bz2", "xz"].contains(&ext.as_str());

    if is_tar_candidate {
        match inspect_archive_tar(archive_path, &ext) {
            Ok(inspection) => return Ok(inspection),
            Err(tar_err) => {
                // Fallback to zip in case file was misnamed
                if let Ok(inspection) = inspect_archive_zip(archive_path) {
                    return Ok(inspection);
                }
                return Err(tar_err);
            }
        }
    }

    // Default: try zip first
    match inspect_archive_zip(archive_path) {
        Ok(inspection) => Ok(inspection),
        Err(zip_err) => {
            // Fallback to tar in case it's actually 7z/rar disguised as zip or unsupported zip compression
            let fallback_format = if ext.is_empty() { "archive" } else { &ext };
            match inspect_archive_tar(archive_path, fallback_format) {
                Ok(inspection) => Ok(inspection),
                Err(_) => Err(zip_err),
            }
        }
    }
}

fn inspect_archive_zip(archive_path: &Path) -> Result<ArchiveInspection, String> {
    let file = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;
    let mut zip = ZipArchive::new(file).map_err(|e| format!("Invalid or corrupted ZIP archive: {}", e))?;

    let mut entries = Vec::new();
    let mut violations = Vec::new();
    let mut total_uncompressed_size = 0u64;
    let mut total_compressed_size = 0u64;
    let mut seen_paths = HashSet::new();

    let entry_count = zip.len();
    if entry_count > MAX_FILE_ENTRIES {
        violations.push(format!("Archive exceeds maximum entry limit: {} > {}", entry_count, MAX_FILE_ENTRIES));
    }

    for i in 0..entry_count {
        let entry = zip.by_index(i).map_err(|e| format!("Corrupted archive entry at index {}: {}", i, e))?;
        let raw_name = entry.name().to_string();
        let uncompressed = entry.size();
        let compressed = entry.compressed_size();
        let is_dir = entry.is_dir();
        let crc = format!("{:08x}", entry.crc32());

        total_uncompressed_size += uncompressed;
        total_compressed_size += compressed;

        let (suspicious, suspicion_reason) = run_security_checks(
            &raw_name,
            uncompressed,
            is_dir,
            &mut seen_paths,
            &mut violations,
        );

        entries.push(ArchiveEntry {
            path: raw_name,
            size: uncompressed,
            compressed_size: compressed,
            crc32: crc,
            is_directory: is_dir,
            is_suspicious: suspicious,
            suspicion_reason,
        });
    }

    if total_uncompressed_size > MAX_TOTAL_UNCOMPRESSED_BYTES {
        violations.push(format!("Archive total uncompressed size exceeds limit ({} GB > 4 GB)", total_uncompressed_size / (1024 * 1024 * 1024)));
    }

    let ratio = if total_compressed_size > 0 {
        total_uncompressed_size as f64 / total_compressed_size as f64
    } else {
        1.0
    };

    if total_uncompressed_size > 10 * 1024 * 1024 && ratio > MAX_COMPRESSION_RATIO {
        violations.push(format!("Suspicious compression ratio {:.1}:1 (potential Zip bomb attack)", ratio));
    }

    let is_safe = violations.is_empty();

    Ok(ArchiveInspection {
        archive_path: archive_path.to_string_lossy().to_string(),
        format: "zip".to_string(),
        total_entries: entries.len(),
        total_uncompressed_size,
        compression_ratio: (ratio * 10.0).round() / 10.0,
        is_safe,
        safety_violations: violations,
        entries,
    })
}

fn inspect_archive_tar(archive_path: &Path, format_label: &str) -> Result<ArchiveInspection, String> {
    let mut cmd = std::process::Command::new("tar");
    cmd.arg("-tvf").arg(archive_path);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd.output().map_err(|e| format!("Failed to invoke tar for archive inspection: {}", e))?;
    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Archive read failed via tar: {}", err_msg.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let pattern = regex::Regex::new(r"^\s*([drwxls\-]{10})\s+\S+\s+\S+\s+\S+\s+(\d+)\s+\S+\s+\S+(?:\s+\S+)?\s+(.+)$")
        .map_err(|e| e.to_string())?;

    let archive_file_size = fs::metadata(archive_path).map(|m| m.len()).unwrap_or(0);
    let mut entries = Vec::new();
    let mut violations = Vec::new();
    let mut total_uncompressed_size = 0u64;
    let mut seen_paths = HashSet::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (mode, size_val, raw_name) = if let Some(caps) = pattern.captures(line) {
            let m = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            let s = caps.get(2).map(|c| c.as_str()).unwrap_or("0").parse::<u64>().unwrap_or(0);
            let p = caps.get(3).map(|c| c.as_str()).unwrap_or("").to_string();
            (m, s, p)
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let m = parts[0];
                let s = parts[4].parse::<u64>().unwrap_or(0);
                let p = parts[5..].join(" ");
                (m, s, p)
            } else {
                continue;
            }
        };

        let is_dir = mode.starts_with('d') || raw_name.ends_with('/') || raw_name.ends_with('\\');
        total_uncompressed_size += size_val;

        let (suspicious, suspicion_reason) = run_security_checks(
            &raw_name,
            size_val,
            is_dir,
            &mut seen_paths,
            &mut violations,
        );

        entries.push(ArchiveEntry {
            path: raw_name,
            size: size_val,
            compressed_size: 0,
            crc32: "00000000".to_string(),
            is_directory: is_dir,
            is_suspicious: suspicious,
            suspicion_reason,
        });
    }

    if entries.len() > MAX_FILE_ENTRIES {
        violations.push(format!("Archive exceeds maximum entry limit: {} > {}", entries.len(), MAX_FILE_ENTRIES));
    }

    if total_uncompressed_size > MAX_TOTAL_UNCOMPRESSED_BYTES {
        violations.push(format!("Archive total uncompressed size exceeds limit ({} GB > 4 GB)", total_uncompressed_size / (1024 * 1024 * 1024)));
    }

    let ratio = if archive_file_size > 0 {
        total_uncompressed_size as f64 / archive_file_size as f64
    } else {
        1.0
    };

    if total_uncompressed_size > 10 * 1024 * 1024 && ratio > MAX_COMPRESSION_RATIO {
        violations.push(format!("Suspicious compression ratio {:.1}:1 (potential Zip bomb attack)", ratio));
    }

    let is_safe = violations.is_empty();

    Ok(ArchiveInspection {
        archive_path: archive_path.to_string_lossy().to_string(),
        format: format_label.to_string(),
        total_entries: entries.len(),
        total_uncompressed_size,
        compression_ratio: (ratio * 10.0).round() / 10.0,
        is_safe,
        safety_violations: violations,
        entries,
    })
}

fn run_security_checks(
    raw_name: &str,
    uncompressed: u64,
    is_dir: bool,
    seen_paths: &mut HashSet<String>,
    violations: &mut Vec<String>,
) -> (bool, Option<String>) {
    let mut suspicious = false;
    let mut suspicion_reason = None;

    let normalized = raw_name.replace('\\', "/");

    // 1. Check for absolute path (leading slash)
    if normalized.starts_with('/') {
        suspicious = true;
        suspicion_reason = Some("Absolute path not permitted".to_string());
        violations.push(format!("Illegal absolute path in entry: '{}'", raw_name));
    }

    // 2. Check for Windows Drive letter (e.g. C:, D:)
    if normalized.len() >= 2 && normalized.as_bytes()[1] == b':' {
        suspicious = true;
        suspicion_reason = Some("Windows drive path detected".to_string());
        violations.push(format!("Illegal Windows drive path: '{}'", raw_name));
    }

    // 3. Check for UNC network path (e.g. // or \\)
    if raw_name.starts_with(r"\\") || raw_name.starts_with("//") {
        suspicious = true;
        suspicion_reason = Some("UNC path detected".to_string());
        violations.push(format!("Illegal UNC network path: '{}'", raw_name));
    }

    // 4. Check for Directory Traversal (../ or ..\)
    let segments: Vec<&str> = normalized.split('/').collect();
    for seg in &segments {
        if *seg == ".." {
            suspicious = true;
            suspicion_reason = Some("Path traversal sequence '..' detected".to_string());
            violations.push(format!("Path traversal attempt detected in '{}'", raw_name));
            break;
        }
    }

    // 5. Case-insensitive collision detection
    let lower_key = normalized.to_lowercase();
    if !is_dir && seen_paths.contains(&lower_key) {
        suspicious = true;
        suspicion_reason = Some("Case-insensitive duplicate file collision in archive".to_string());
        violations.push(format!("Duplicate or case collision path: '{}'", raw_name));
    } else {
        seen_paths.insert(lower_key);
    }

    // 6. Single file size check
    if uncompressed > MAX_SINGLE_FILE_BYTES {
        suspicious = true;
        suspicion_reason = Some("Single file size exceeds security limit (2 GB)".to_string());
        violations.push(format!("Excessive file size in entry '{}'", raw_name));
    }

    // 7. Dangerous file types (executables / batch scripts / VBS / powershell)
    let ext = Path::new(&normalized).extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    if ["exe", "bat", "cmd", "vbs", "ps1", "scr", "pif", "msi"].contains(&ext.as_str()) {
        suspicious = true;
        suspicion_reason = Some(format!("Contains executable script or binary (.{ext})", ext = ext));
    }

    (suspicious, suspicion_reason)
}

pub fn extract_archive_safely(archive_path: &Path, destination_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let inspection = inspect_archive(archive_path)?;
    if !inspection.is_safe {
        return Err(format!("Archive extraction aborted due to security violations: {}", inspection.safety_violations.join(", ")));
    }

    let ext = archive_path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    if ["asi", "cs", "cleo", "dll", "ini", "dat"].contains(&ext.as_str()) {
        fs::create_dir_all(destination_dir).map_err(|e| format!("Failed to create destination dir: {}", e))?;
        let file_name = archive_path.file_name().unwrap_or_default();
        let outpath = destination_dir.join(file_name);
        fs::copy(archive_path, &outpath).map_err(|e| format!("Failed to copy standalone file: {}", e))?;
        return Ok(vec![outpath]);
    }

    let is_tar = ["7z", "rar", "tar", "gz", "bz2", "xz"].contains(&ext.as_str()) || inspection.format != "zip";

    if is_tar {
        return extract_archive_tar(archive_path, destination_dir);
    }

    match extract_archive_zip(archive_path, destination_dir) {
        Ok(files) => Ok(files),
        Err(zip_err) => {
            // Fallback to tar
            match extract_archive_tar(archive_path, destination_dir) {
                Ok(files) => Ok(files),
                Err(_) => Err(zip_err),
            }
        }
    }
}

fn extract_archive_zip(archive_path: &Path, destination_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let file = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;
    let mut zip = ZipArchive::new(file).map_err(|e| format!("Failed to read archive: {}", e))?;

    fs::create_dir_all(destination_dir).map_err(|e| format!("Failed to create destination dir: {}", e))?;
    let canonical_dest = destination_dir.canonicalize().map_err(|e| format!("Failed to canonicalize destination: {}", e))?;
    let mut extracted_files = Vec::new();

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| format!("Failed to read entry {}: {}", i, e))?;
        let entry_name = entry.name().to_string();

        let safe_rel_path = sanitize_archive_entry_path(&entry_name)?;
        let outpath = canonical_dest.join(&safe_rel_path);

        if !outpath.starts_with(&canonical_dest) {
            return Err(format!("Path traversal attempt blocked during extraction: '{}'", entry_name));
        }

        if entry.is_dir() {
            fs::create_dir_all(&outpath).map_err(|e| format!("Failed to create directory {:?}: {}", outpath, e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| format!("Failed to create parent dir {:?}: {}", p, e))?;
                }
            }

            let mut outfile = File::create(&outpath).map_err(|e| format!("Failed to create file {:?}: {}", outpath, e))?;
            io::copy(&mut entry, &mut outfile).map_err(|e| format!("Failed to extract file {:?}: {}", outpath, e))?;
            extracted_files.push(outpath);
        }
    }

    Ok(extracted_files)
}

fn extract_archive_tar(archive_path: &Path, destination_dir: &Path) -> Result<Vec<PathBuf>, String> {
    fs::create_dir_all(destination_dir).map_err(|e| format!("Failed to create destination dir: {}", e))?;
    let canonical_dest = destination_dir.canonicalize().map_err(|e| format!("Failed to canonicalize destination: {}", e))?;

    let mut cmd = std::process::Command::new("tar");
    cmd.arg("-xf")
        .arg(archive_path)
        .arg("-C")
        .arg(&canonical_dest);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd.output().map_err(|e| format!("Failed to execute tar extraction: {}", e))?;
    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("tar extraction failed: {}", err_msg.trim()));
    }

    let mut extracted_files = Vec::new();
    collect_files_recursive(&canonical_dest, &canonical_dest, &mut extracted_files)?;
    Ok(extracted_files)
}

fn collect_files_recursive(root: &Path, current_dir: &Path, result: &mut Vec<PathBuf>) -> Result<(), String> {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files_recursive(root, &path, result)?;
            } else if path.is_file() {
                if !path.starts_with(root) {
                    return Err(format!("Extracted file escaped root boundary: {:?}", path));
                }
                result.push(path);
            }
        }
    }
    Ok(())
}

pub fn sanitize_archive_entry_path(path_str: &str) -> Result<PathBuf, String> {
    let normalized = path_str.replace('\\', "/");
    let mut components = Vec::new();

    for part in normalized.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            return Err(format!("Path traversal '..' is prohibited in '{}'", path_str));
        }
        if part.contains(':') {
            return Err(format!("Illegal colon in path component '{}'", part));
        }
        components.push(part);
    }

    if components.is_empty() {
        return Err("Empty entry path".to_string());
    }

    let mut result = PathBuf::new();
    for c in components {
        result.push(c);
    }
    Ok(result)
}
