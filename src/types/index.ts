export type ConfidenceLevel = 'confirmed' | 'likely' | 'ambiguous' | 'incompatible' | 'invalid';

export interface GameInstallation {
  id: string;
  name: string;
  executablePath: string;
  rootPath: string;
  detectedEdition: string;
  versionConfidence: ConfidenceLevel;
  executableSha256: string;
  executableSizeBytes: number;
  peTimestamp: number | null;
  evidenceList: string[];
  uncertainties: string[];
  isClassic10: boolean;
  isActive: boolean;
  createdDate: string;
  lastVerifiedDate: string;
}

export interface ModSource {
  id: string;
  name: string;
  baseUrl: string;
  capabilities: {
    canSearch: boolean;
    canDirectDownload: boolean;
    requiresBrowserDownload: boolean;
    requiresAuth: boolean;
    supportsVersionFilter: boolean;
    supportsCategoryFilter: boolean;
  };
  status: 'available' | 'rate_limited' | 'offline' | 'manual_only';
  attribution: string;
  termsNotice: string;
}

export type InstallationStrategyType =
  | 'cleo'
  | 'asi'
  | 'modloader'
  | 'assets'
  | 'audio'
  | 'loose'
  | 'manual'
  | 'guided'
  | 'unknown';

export interface ModRecord {
  id: string;
  sourceId: string;
  sourceModId: string;
  title: string;
  author: string;
  description: string;
  category: string;
  tags: string[];
  thumbnailUrl: string | null;
  pageUrl: string;
  downloadUrl: string | null;
  fileSizeFormatted: string | null;
  fileSizeBytes: number | null;
  releaseDate: string | null;
  updateDate: string | null;
  supportedGameVersions: string[];
  requiredDependencies: string[];
  incompatibilities: string[];
  installationStrategy: InstallationStrategyType;
  confidence: 'confirmed' | 'reported' | 'likely' | 'unknown' | 'incompatible';
  compatibilityReason: string;
}

export interface DownloadJob {
  id: string;
  modId: string;
  modTitle: string;
  sourceId: string;
  sourceName: string;
  url: string;
  tempFilePath: string;
  totalBytes: number;
  downloadedBytes: number;
  speedBytesPerSec: number;
  status: 'pending' | 'downloading' | 'paused' | 'completed' | 'failed' | 'cancelled';
  errorMessage: string | null;
  expectedSha256: string | null;
  computedSha256: string | null;
  sha256Matched: boolean | null;
  startTime: string;
  completedTime: string | null;
}

export interface ArchiveEntry {
  path: string;
  size: number;
  compressedSize: number;
  crc32: string;
  isDirectory: boolean;
  isSuspicious: boolean;
  suspicionReason: string | null;
}

export interface ArchiveInspection {
  archivePath: string;
  format: 'zip' | 'rar' | '7z' | 'unknown';
  totalEntries: number;
  totalUncompressedSize: number;
  compressionRatio: number;
  isSafe: boolean;
  safetyViolations: string[];
  entries: ArchiveEntry[];
}

export interface PlannedFileOperation {
  archiveRelPath: string;
  destinationRelPath: string;
  destinationAbsolutePath: string;
  fileType: string;
  rule: string;
  destinationState: 'new' | 'overwrite' | 'collision';
  backupRequired: boolean;
  fileSizeBytes: number;
}

export interface InstallationPlan {
  modId: string;
  modTitle: string;
  installationType: 'automatic' | 'guided' | 'manual';
  strategy: InstallationStrategyType;
  totalFiles: number;
  newFilesCount: number;
  overwriteFilesCount: number;
  missingPrerequisites: string[];
  warnings: string[];
  fileOperations: PlannedFileOperation[];
  confidence: 'high' | 'medium' | 'low' | 'manual_required';
  manualInstructions: string | null;
}

export interface InstalledFileRecord {
  id: string;
  installedModId: string;
  destinationRelPath: string;
  sha256: string;
  fileSizeBytes: number;
  hasBackup: boolean;
  backupPath: string | null;
}

export interface InstalledMod {
  id: string;
  gameInstallationId: string;
  profileId: string;
  name: string;
  author: string;
  version: string;
  sourceName: string;
  sourceUrl: string | null;
  installedAt: string;
  strategy: string;
  enabled: boolean;
  files: InstalledFileRecord[];
  hasConflicts: boolean;
  conflictNotes: string[];
}

export interface Profile {
  id: string;
  gameInstallationId: string;
  name: string;
  description: string;
  isDefault: boolean;
  isActive: boolean;
  installedModIds: string[];
  createdAt: string;
  updatedAt: string;
}

export interface FileConflict {
  id: string;
  destinationRelPath: string;
  activeModId: string;
  activeModName: string;
  conflictingModId: string;
  conflictingModName: string;
  status: 'detected' | 'resolved_mod_a' | 'resolved_mod_b' | 'ignored';
  resolutionNotes: string | null;
}

export interface BackupRecord {
  id: string;
  gameInstallationId: string;
  modId: string;
  modName: string;
  destinationRelPath: string;
  backupFilePath: string;
  originalSha256: string;
  originalSizeBytes: number;
  timestamp: string;
  restored: boolean;
}

export interface RecoveryJournalEntry {
  id: string;
  operationId: string;
  operationType: 'install' | 'uninstall' | 'profile_switch';
  status: 'in_progress' | 'committed' | 'interrupted' | 'rolled_back';
  targetModId: string;
  targetModName: string;
  timestamp: string;
  stagedBackupPaths: string[];
  affectedFiles: string[];
}

export interface DiagnosticReport {
  appVersion: string;
  osVersion: string;
  rustVersion: string;
  gameDetected: boolean;
  activeGame: GameInstallation | null;
  installedModsCount: number;
  pendingOperations: number;
  sqliteHealth: string;
  recentLogs: string[];
}

export interface AppSettings {
  defaultGameInstallationId: string | null;
  downloadDirectory: string;
  backupDirectory: string;
  maxConcurrentDownloads: number;
  requireInstallConfirmation: boolean;
  autoCheckUpdates: boolean;
  theme: 'dark' | 'light';
  logLevel: 'debug' | 'info' | 'warn' | 'error';
}
