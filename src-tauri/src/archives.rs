use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read};
use std::collections::HashSet;
use zip::ZipArchive;
use crate::models::{ArchiveEntry, ArchiveInspection};

// Security constraints
const MAX_TOTAL_UNCOMPRESSED_BYTES: u64 = 4 * 1024 * 1024 * 1024; // 4 GB
const MAX_FILE_ENTRIES: usize = 15_000;
const MAX_COMPRESSION_RATIO: f64 = 150.0;
const MAX_SINGLE_FILE_BYTES: u64 = 2 * 1024 * 1024 * 1024; // 2 GB

pub fn inspect_archive(archive_path: &Path) -> Result<ArchiveInspection, String> {
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

        let mut suspicious = false;
        let mut suspicion_reason = None;

        // Path Traversal checks:
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

pub fn extract_archive_safely(archive_path: &Path, destination_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let inspection = inspect_archive(archive_path)?;
    if !inspection.is_safe {
        return Err(format!("Archive extraction aborted due to security violations: {}", inspection.safety_violations.join(", ")));
    }

    let file = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;
    let mut zip = ZipArchive::new(file).map_err(|e| format!("Failed to read archive: {}", e))?;

    fs::create_dir_all(destination_dir).map_err(|e| format!("Failed to create destination dir: {}", e))?;

    let canonical_dest = destination_dir.canonicalize().map_err(|e| format!("Failed to canonicalize destination: {}", e))?;
    let mut extracted_files = Vec::new();

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| format!("Failed to read entry {}: {}", i, e))?;
        let entry_name = entry.name().to_string();

        // Safe path join
        let safe_rel_path = sanitize_archive_entry_path(&entry_name)?;
        let outpath = canonical_dest.join(&safe_rel_path);

        // Ensure outpath remains strictly inside canonical_dest
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
