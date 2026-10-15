# Contributing to San Andreas Mod Manager (SAMM)

Thank you for your interest in contributing to SAMM! SAMM is an open-source, local-first mod manager prioritizing safety, transparency, and recoverability.

## 1. Development Guidelines

- **Real functionality over decorative UI**: Do not submit mock or placeholder components that do not perform real filesystem or database operations.
- **Never report success prematurely**: File operations, downloads, checksum verifications, and rollbacks must only report success once verified on disk.
- **Path Traversal & Security**: Never trust archive filenames or source metadata. Always pass entry paths through `archives::sanitize_archive_entry_path`.
- **Honest Capabilities**: When creating a source integration, declare unsupported operations explicitly rather than simulating fake results.

---

## 2. Adding a New Source Adapter

To add support for a new GTA modding repository:

1. Create a new module file in `src-tauri/src/sources/<source_name>.rs`.
2. Implement the `SourceAdapter` trait:
   ```rust
   use async_trait::async_trait;
   use crate::models::{ModRecord, SourceCapabilities};
   use super::SourceAdapter;

   pub struct MySourceAdapter;

   #[async_trait]
   impl SourceAdapter for MySourceAdapter {
       fn id(&self) -> &'static str { "mysource" }
       fn name(&self) -> &'static str { "MySource" }
       fn base_url(&self) -> &'static str { "https://example.com" }
       fn capabilities(&self) -> SourceCapabilities {
           SourceCapabilities {
               can_search: true,
               can_direct_download: false, // Declare honestly
               requires_browser_download: true,
               requires_auth: false,
               supports_version_filter: false,
               supports_category_filter: true,
           }
       }
       fn attribution(&self) -> &'static str { "Content belongs to original authors on example.com." }
       fn terms_notice(&self) -> &'static str { "Public access policy statement." }

       async fn search_mods(&self, query: &str, category: Option<&str>, page: u32) -> Result<Vec<ModRecord>, String> {
           // Implement lawful search endpoint or catalog query
       }

       async fn get_mod_details(&self, source_mod_id: &str) -> Result<ModRecord, String> {
           // Return normalized ModRecord
       }

       async fn get_download_url(&self, source_mod_id: &str) -> Result<Option<String>, String> {
           // Return direct URL or None if browser download is required
       }
   }
   ```
3. Register the new adapter in `src-tauri/src/sources/mod.rs` inside `SourceManager::new()`.
4. Add unit tests verifying parsing against saved fixtures.

---

## 3. Running Tests Before Submitting

Always run the full test suite before submitting a Pull Request:

```bash
# 1. Run Rust tests
cd src-tauri
cargo test --lib
cd ..

# 2. Run TypeScript tests
npx tsx --test src/tests/domain.test.ts

# 3. Verify frontend compilation
npm run build
```
