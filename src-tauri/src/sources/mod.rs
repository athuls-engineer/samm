pub mod gtainside;
pub mod libertycity;
pub mod mixmods;
pub mod gtagarage;
pub mod manual_import;

use async_trait::async_trait;
use crate::models::{ModRecord, ModSource, SourceCapabilities};

#[async_trait]
pub trait SourceAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn base_url(&self) -> &'static str;
    fn capabilities(&self) -> SourceCapabilities;
    fn attribution(&self) -> &'static str;
    fn terms_notice(&self) -> &'static str;

    async fn search_mods(
        &self,
        query: &str,
        category: Option<&str>,
        page: u32,
    ) -> Result<Vec<ModRecord>, String>;

    async fn get_mod_details(&self, source_mod_id: &str) -> Result<ModRecord, String>;

    async fn get_download_url(&self, source_mod_id: &str) -> Result<Option<String>, String>;
}

pub struct SourceManager {
    adapters: Vec<Box<dyn SourceAdapter>>,
}

impl SourceManager {
    pub fn new() -> Self {
        let mut adapters: Vec<Box<dyn SourceAdapter>> = Vec::new();
        adapters.push(Box::new(gtainside::GtaInsideAdapter::new()));
        adapters.push(Box::new(libertycity::LibertyCityAdapter::new()));
        adapters.push(Box::new(mixmods::MixModsAdapter::new()));
        adapters.push(Box::new(gtagarage::GtaGarageAdapter::new()));
        adapters.push(Box::new(manual_import::ManualImportAdapter::new()));

        SourceManager { adapters }
    }

    pub fn list_sources(&self) -> Vec<ModSource> {
        self.adapters
            .iter()
            .map(|a| ModSource {
                id: a.id().to_string(),
                name: a.name().to_string(),
                base_url: a.base_url().to_string(),
                capabilities: a.capabilities(),
                status: "available".to_string(),
                attribution: a.attribution().to_string(),
                terms_notice: a.terms_notice().to_string(),
            })
            .collect()
    }

    pub async fn search_all(
        &self,
        query: &str,
        category: Option<&str>,
        source_id: Option<&str>,
    ) -> Vec<ModRecord> {
        let mut results = Vec::new();

        for adapter in &self.adapters {
            if let Some(target) = source_id {
                if !target.is_empty() && target != "all" && adapter.id() != target {
                    continue;
                }
            }

            match adapter.search_mods(query, category, 1).await {
                Ok(mut mods) => results.append(&mut mods),
                Err(e) => {
                    tracing::warn!("Search failed on source '{}': {}", adapter.id(), e);
                }
            }
        }

        // Deduplicate results to ensure mod uniqueness across catalog and sources
        Self::deduplicate_mods(results)
    }

    pub fn deduplicate_mods(mods: Vec<ModRecord>) -> Vec<ModRecord> {
        let mut unique: Vec<ModRecord> = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();
        let mut seen_normalized_titles = std::collections::HashSet::new();

        let mut seen_urls = std::collections::HashSet::new();

        for m in mods {
            if seen_ids.contains(&m.id) {
                continue;
            }

            if let Some(ref url) = m.download_url {
                if seen_urls.contains(url) {
                    continue;
                }
            }

            let norm_title: String = m
                .title
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect();

            if !norm_title.is_empty() && seen_normalized_titles.contains(&norm_title) {
                // If the incoming mod has a 1-click download_url while previous did not, upgrade to 1-click
                if m.download_url.is_some() {
                    if let Some(pos) = unique.iter().position(|existing| {
                        let ex_norm: String = existing
                            .title
                            .to_lowercase()
                            .chars()
                            .filter(|c| c.is_alphanumeric())
                            .collect();
                        ex_norm == norm_title && existing.download_url.is_none()
                    }) {
                        seen_ids.remove(&unique[pos].id);
                        unique[pos] = m.clone();
                        seen_ids.insert(m.id.clone());
                        if let Some(ref u) = m.download_url {
                            seen_urls.insert(u.clone());
                        }
                    }
                }
                continue;
            }

            seen_ids.insert(m.id.clone());
            if let Some(ref u) = m.download_url {
                seen_urls.insert(u.clone());
            }
            if !norm_title.is_empty() {
                seen_normalized_titles.insert(norm_title);
            }
            unique.push(m);
        }

        unique
    }

    pub async fn get_details(&self, source_id: &str, source_mod_id: &str) -> Result<ModRecord, String> {
        for adapter in &self.adapters {
            if adapter.id() == source_id {
                return adapter.get_mod_details(source_mod_id).await;
            }
        }
        Err(format!("Source adapter '{}' not found", source_id))
    }

    pub async fn get_download_url(&self, source_id: &str, source_mod_id: &str) -> Result<Option<String>, String> {
        for adapter in &self.adapters {
            if adapter.id() == source_id {
                return adapter.get_download_url(source_mod_id).await;
            }
        }
        Err(format!("Source adapter '{}' not found", source_id))
    }
}
