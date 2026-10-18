import { invoke } from '@tauri-apps/api/core';
import {
  GameInstallation,
  ModRecord,
  ModSource,
  DownloadJob,
  ArchiveInspection,
  InstallationPlan,
  InstalledMod,
  Profile,
  FileConflict,
  BackupRecord,
  DiagnosticReport,
  AppSettings,
  DownloadedArchiveItem,
  ConflictResolutionResult,
  CacheCleanupResult,
  DuplicateModGroup,
  DuplicateResolutionResult,
} from '../types';

export const api = {
  // Game Installations
  getInstallations: async (): Promise<GameInstallation[]> => {
    return await invoke('get_installations');
  },

  inspectGamePath: async (path: string): Promise<GameInstallation> => {
    return await invoke('inspect_game_path', { path });
  },

  saveGameInstallation: async (game: GameInstallation): Promise<void> => {
    return await invoke('save_game_installation', { game });
  },

  setActiveInstallation: async (id: string): Promise<void> => {
    return await invoke('set_active_installation', { id });
  },

  deleteGameInstallation: async (id: string): Promise<void> => {
    return await invoke('delete_game_installation', { id });
  },

  discoverInstallations: async (): Promise<GameInstallation[]> => {
    return await invoke('discover_installations');
  },

  // Sources & Discovery
  listSources: async (): Promise<ModSource[]> => {
    return await invoke('list_sources');
  },

  searchMods: async (query: string, category?: string, sourceId?: string): Promise<ModRecord[]> => {
    return await invoke('search_mods', {
      query,
      category: category || null,
      sourceId: sourceId || null,
    });
  },

  getModDetails: async (sourceId: string, sourceModId: string): Promise<ModRecord> => {
    return await invoke('get_mod_details', { sourceId, sourceModId });
  },

  // Archives & Installation
  inspectArchive: async (archivePath: string): Promise<ArchiveInspection> => {
    return await invoke('inspect_archive', { archivePath });
  },

  generateInstallationPlan: async (
    archivePath: string,
    gameId: string,
    modId: string,
    modTitle: string
  ): Promise<InstallationPlan> => {
    return await invoke('generate_installation_plan', {
      archivePath,
      gameId,
      modId,
      modTitle,
    });
  },

  installMod: async (
    archivePath: string,
    plan: InstallationPlan,
    gameId: string,
    profileId: string
  ): Promise<InstalledMod> => {
    return await invoke('install_mod', {
      archivePath,
      plan,
      gameId,
      profileId,
    });
  },

  uninstallMod: async (modId: string, gameId: string): Promise<void> => {
    return await invoke('uninstall_mod', { modId, gameId });
  },

  getInstalledMods: async (gameId: string): Promise<InstalledMod[]> => {
    return await invoke('get_installed_mods', { gameId });
  },

  toggleModState: async (modId: string, enabled: boolean, gameId: string): Promise<void> => {
    return await invoke('toggle_mod_state', { modId, enabled, gameId });
  },

  // Profiles
  getProfiles: async (gameId: string): Promise<Profile[]> => {
    return await invoke('get_profiles', { gameId });
  },

  createProfile: async (gameId: string, name: string, description: string): Promise<Profile> => {
    return await invoke('create_profile', { gameId, name, description });
  },

  setActiveProfile: async (profileId: string, gameId: string): Promise<void> => {
    return await invoke('set_active_profile', { profileId, gameId });
  },

  deleteProfile: async (profileId: string): Promise<void> => {
    return await invoke('delete_profile', { profileId });
  },

  exportProfile: async (profileId: string, gameId: string): Promise<string> => {
    return await invoke('export_profile', { profileId, gameId });
  },

  importProfile: async (manifestJson: string, gameId: string): Promise<Profile> => {
    return await invoke('import_profile', { manifestJson, gameId });
  },

  // Conflicts & Backups
  getConflicts: async (gameId: string): Promise<FileConflict[]> => {
    return await invoke('get_conflicts', { gameId });
  },

  checkForConflicts: async (gameId: string): Promise<FileConflict[]> => {
    return await invoke('check_for_conflicts', { gameId });
  },

  resolveConflict: async (
    gameId: string,
    destinationRelPath: string,
    keepModId: string,
    yieldModId: string
  ): Promise<void> => {
    return await invoke('resolve_conflict', {
      gameId,
      destinationRelPath,
      keepModId,
      yieldModId,
    });
  },

  autoResolveConflicts: async (gameId: string): Promise<ConflictResolutionResult> => {
    return await invoke('auto_resolve_conflicts', { gameId });
  },

  getBackups: async (gameId: string): Promise<BackupRecord[]> => {
    return await invoke('get_backups', { gameId });
  },

  // Downloads
  startDownload: async (
    modId: string,
    modTitle: string,
    sourceId: string,
    sourceName: string,
    url: string,
    expectedSha256?: string
  ): Promise<DownloadJob> => {
    return await invoke('start_download', {
      modId,
      modTitle,
      sourceId,
      sourceName,
      url,
      expectedSha256: expectedSha256 || null,
    });
  },

  getDownloadJobs: async (): Promise<DownloadJob[]> => {
    return await invoke('get_download_jobs');
  },

  cancelDownload: async (jobId: string): Promise<void> => {
    return await invoke('cancel_download', { jobId });
  },

  deleteDownload: async (jobId: string): Promise<void> => {
    return await invoke('delete_download', { jobId });
  },

  clearCompletedDownloads: async (): Promise<void> => {
    return await invoke('clear_completed_downloads');
  },

  // Launch & Diagnostics
  launchGame: async (gameId: string, customArgs: string[]): Promise<number> => {
    return await invoke('launch_game', { gameId, customArgs });
  },

  getDiagnosticReport: async (): Promise<DiagnosticReport> => {
    return await invoke('get_diagnostic_report');
  },

  getSettings: async (): Promise<AppSettings> => {
    return await invoke('get_settings');
  },

  saveSettings: async (settings: AppSettings): Promise<void> => {
    return await invoke('save_settings', { settings });
  },

  clearDownloadCache: async (downloadDir?: string): Promise<CacheCleanupResult> => {
    return await invoke('clear_download_cache', { downloadDir: downloadDir || null });
  },

  purgeStaleBackups: async (gameId?: string): Promise<CacheCleanupResult> => {
    return await invoke('purge_stale_backups', { gameId: gameId || null });
  },

  syncExistingMods: async (gameId: string): Promise<InstalledMod[]> => {
    return await invoke('sync_existing_mods', { gameId });
  },

  pickFolder: async (): Promise<string | null> => {
    return await invoke('pick_folder');
  },

  pickArchiveFile: async (): Promise<string | null> => {
    return await invoke('pick_archive_file');
  },

  openPathInExplorer: async (targetPath: string): Promise<void> => {
    return await invoke('open_path_in_explorer', { targetPath });
  },

  openUrl: async (url: string): Promise<void> => {
    return await invoke('open_url_in_browser', { url });
  },

  isGameProcessRunning: async (gameId: string): Promise<boolean> => {
    return await invoke('is_game_process_running', { gameId });
  },

  killGameProcess: async (gameId: string): Promise<void> => {
    return await invoke('kill_game_process', { gameId });
  },

  batchToggleMods: async (gameId: string, modIds: string[], enabled: boolean): Promise<void> => {
    return await invoke('batch_toggle_mods', { gameId, modIds, enabled });
  },

  batchUninstallMods: async (gameId: string, modIds: string[]): Promise<void> => {
    return await invoke('batch_uninstall_mods', { gameId, modIds });
  },

  getRecentDownloadedArchives: async (): Promise<DownloadedArchiveItem[]> => {
    return await invoke('get_recent_downloaded_archives');
  },

  getDuplicateMods: async (gameId: string): Promise<DuplicateModGroup[]> => {
    return await invoke('get_duplicate_mods', { gameId });
  },

  removeDuplicateMod: async (gameId: string, targetId: string): Promise<string> => {
    return await invoke('remove_duplicate_mod', { gameId, targetId });
  },

  removeAllDuplicates: async (gameId: string): Promise<DuplicateResolutionResult> => {
    return await invoke('remove_all_duplicates', { gameId });
  },
};
