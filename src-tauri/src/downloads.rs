use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use sha2::{Digest, Sha256};
use reqwest::Client;
use reqwest::header::{RANGE, CONTENT_LENGTH};
use crate::models::DownloadJob;
use crate::database::Database;

pub struct DownloadManager {
    client: Client,
    db: Database,
    active_downloads: Arc<Mutex<Vec<String>>>, // Job IDs
}

impl Clone for DownloadManager {
    fn clone(&self) -> Self {
        DownloadManager {
            client: self.client.clone(),
            db: self.db.clone(),
            active_downloads: Arc::clone(&self.active_downloads),
        }
    }
}

impl DownloadManager {
    pub fn new(db: Database) -> Self {
        let client = Client::builder()
            .user_agent("SAMM-OpenSource-ModManager/0.1 (+https://github.com/samm-modmanager/samm)")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        DownloadManager {
            client,
            db,
            active_downloads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start_download(
        &self,
        mod_id: &str,
        mod_title: &str,
        source_id: &str,
        source_name: &str,
        url: &str,
        download_dir: &Path,
        expected_sha256: Option<String>,
    ) -> Result<DownloadJob, String> {
        let _ = fs::create_dir_all(download_dir);

        let safe_name: String = mod_title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect();
        let temp_filename = format!("{}_{}.zip.part", safe_name, uuid::Uuid::new_v4().simple());
        let temp_path = download_dir.join(&temp_filename);

        let job = DownloadJob {
            id: uuid::Uuid::new_v4().to_string(),
            mod_id: mod_id.to_string(),
            mod_title: mod_title.to_string(),
            source_id: source_id.to_string(),
            source_name: source_name.to_string(),
            url: url.to_string(),
            temp_file_path: temp_path.to_string_lossy().to_string(),
            total_bytes: 0,
            downloaded_bytes: 0,
            speed_bytes_per_sec: 0,
            status: "downloading".to_string(),
            error_message: None,
            expected_sha256,
            computed_sha256: None,
            sha256_matched: None,
            start_time: chrono::Utc::now().to_rfc3339(),
            completed_time: None,
        };

        self.db.save_download_job(&job).map_err(|e| e.to_string())?;

        // Spawn async download worker
        let mgr = self.clone();
        let job_id = job.id.clone();
        let target_url = url.to_string();
        let file_path = temp_path.clone();

        {
            let mut list = self.active_downloads.lock().unwrap();
            list.push(job_id.clone());
        }

        tokio::spawn(async move {
            mgr.run_download_loop(job_id, target_url, file_path).await;
        });

        Ok(job)
    }

    async fn run_download_loop(&self, job_id: String, url: String, target_path: PathBuf) {
        let mut retry_count = 0;
        let max_retries = 3;

        while retry_count < max_retries {
            match self.execute_chunked_download(&job_id, &url, &target_path).await {
                Ok(_) => {
                    // Finalize download: compute complete SHA-256 and rename .part -> .zip
                    let final_path = target_path.with_extension(""); // removes .part
                    let _ = fs::rename(&target_path, &final_path);

                    let (computed_hash, total_size) = crate::game_detection::compute_file_sha256(&final_path)
                        .unwrap_or_else(|_| (String::new(), 0));

                    if let Ok(mut jobs) = self.db.get_download_jobs() {
                        if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
                            j.status = "completed".to_string();
                            j.downloaded_bytes = total_size;
                            j.total_bytes = total_size;
                            j.temp_file_path = final_path.to_string_lossy().to_string();
                            j.computed_sha256 = Some(computed_hash.clone());
                            if let Some(ref exp) = j.expected_sha256 {
                                j.sha256_matched = Some(exp.to_lowercase() == computed_hash.to_lowercase());
                            }
                            j.completed_time = Some(chrono::Utc::now().to_rfc3339());
                            let _ = self.db.save_download_job(j);
                        }
                    }

                    let mut list = self.active_downloads.lock().unwrap();
                    list.retain(|id| id != &job_id);
                    return;
                }
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        if let Ok(mut jobs) = self.db.get_download_jobs() {
                            if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
                                j.status = "failed".to_string();
                                j.error_message = Some(e);
                                let _ = self.db.save_download_job(j);
                            }
                        }
                        let mut list = self.active_downloads.lock().unwrap();
                        list.retain(|id| id != &job_id);
                        return;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(500 * (1 << retry_count))).await;
                }
            }
        }
    }

    async fn execute_chunked_download(
        &self,
        job_id: &str,
        url: &str,
        path: &Path,
    ) -> Result<(), String> {
        // If file already exists, determine resumed offset
        let existing_len = if path.exists() {
            fs::metadata(path).map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        let mut req = self.client.get(url);
        if existing_len > 0 {
            req = req.header(RANGE, format!("bytes={}-", existing_len));
        }

        let resp = req.send().await.map_err(|e| format!("HTTP request error: {}", e))?;
        if !resp.status().is_success() && resp.status() != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(format!("Download failed with status: {}", resp.status()));
        }

        let total_size = resp.content_length().map(|l| l + existing_len).unwrap_or(0);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("Failed to open destination file: {}", e))?;

        let mut stream = resp.bytes_stream();
        use futures_util::StreamExt;

        let mut downloaded = existing_len;
        let mut last_update = std::time::Instant::now();
        let mut bytes_since_last_update = 0u64;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| format!("Stream error: {}", e))?;
            file.write_all(&chunk).map_err(|e| format!("Disk write error: {}", e))?;

            downloaded += chunk.len() as u64;
            bytes_since_last_update += chunk.len() as u64;

            if last_update.elapsed().as_millis() > 500 {
                let speed = (bytes_since_last_update as f64 / last_update.elapsed().as_secs_f64()) as u64;
                if let Ok(mut jobs) = self.db.get_download_jobs() {
                    if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
                        j.downloaded_bytes = downloaded;
                        j.total_bytes = total_size;
                        j.speed_bytes_per_sec = speed;
                        let _ = self.db.save_download_job(j);
                    }
                }
                last_update = std::time::Instant::now();
                bytes_since_last_update = 0;
            }
        }

        Ok(())
    }
}
