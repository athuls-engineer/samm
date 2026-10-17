import React, { useState, useEffect } from 'react';
import { AppSettings, CacheCleanupResult } from '../types';
import { api } from '../api/client';
import {
  Settings as SettingsIcon,
  Save,
  Check,
  Folder,
  FolderOpen,
  Trash2,
  Palette,
  Gamepad2,
  HardDrive,
  Shield,
  ExternalLink,
  Layers,
  Sparkles,
  Info,
} from 'lucide-react';

interface ThemeOption {
  id: 'charcoal' | 'midnight' | 'slate' | 'cyberpunk' | 'emerald';
  name: string;
  desc: string;
  previewBg: string;
  previewCard: string;
  previewAccent: string;
  previewBorder: string;
}

const THEMES: ThemeOption[] = [
  {
    id: 'charcoal',
    name: 'Charcoal Dark',
    desc: 'Default industrial slate with amber highlight',
    previewBg: '#0f1115',
    previewCard: '#1c1f26',
    previewAccent: '#f59e0b',
    previewBorder: '#383e4d',
  },
  {
    id: 'midnight',
    name: 'Midnight OLED',
    desc: 'Pure deep black with high-contrast gold',
    previewBg: '#000000',
    previewCard: '#101014',
    previewAccent: '#fbbf24',
    previewBorder: '#2c2c36',
  },
  {
    id: 'slate',
    name: 'Nordic Slate',
    desc: 'Subtle ocean navy with crisp sky blue accent',
    previewBg: '#0b0f19',
    previewCard: '#1e293b',
    previewAccent: '#38bdf8',
    previewBorder: '#334155',
  },
  {
    id: 'cyberpunk',
    name: 'San Andreas Gold',
    desc: 'Warm stone dark with vintage Las Venturas gold',
    previewBg: '#0c0a09',
    previewCard: '#292524',
    previewAccent: '#eab308',
    previewBorder: '#44403c',
  },
  {
    id: 'emerald',
    name: 'Grove Green',
    desc: 'Classic Grove Street dark green palette',
    previewBg: '#04120c',
    previewCard: '#0d2c1d',
    previewAccent: '#10b981',
    previewBorder: '#1c4d34',
  },
];

export const Settings: React.FC = () => {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [saved, setSaved] = useState(false);
  const [loading, setLoading] = useState(false);
  const [cleaningDownloads, setCleaningDownloads] = useState(false);
  const [cleaningBackups, setCleaningBackups] = useState(false);
  const [cleanupMessage, setCleanupMessage] = useState<string | null>(null);

  useEffect(() => {
    api.getSettings().then((s) => {
      setSettings(s);
      if (s.theme) {
        document.documentElement.className = 'theme-' + s.theme;
      }
    }).catch(console.error);
  }, []);

  const handleSave = async () => {
    if (!settings) return;
    setLoading(true);
    try {
      await api.saveSettings(settings);
      setSaved(true);
      setTimeout(() => setSaved(false), 2500);
    } catch (e) {
      console.error('Failed to save settings:', e);
    } finally {
      setLoading(false);
    }
  };

  const handleThemeChange = (newTheme: AppSettings['theme']) => {
    if (!settings) return;
    setSettings({ ...settings, theme: newTheme });
    document.documentElement.className = 'theme-' + newTheme;
  };

  const handleBrowseDownloadDir = async () => {
    try {
      const selected = await api.pickFolder();
      if (selected && settings) {
        setSettings({ ...settings, downloadDirectory: selected });
      }
    } catch (e) {
      console.error('Failed to pick download folder:', e);
    }
  };

  const handleBrowseBackupDir = async () => {
    try {
      const selected = await api.pickFolder();
      if (selected && settings) {
        setSettings({ ...settings, backupDirectory: selected });
      }
    } catch (e) {
      console.error('Failed to pick backup folder:', e);
    }
  };

  const handleCleanDownloadCache = async () => {
    if (!settings) return;
    setCleaningDownloads(true);
    setCleanupMessage(null);
    try {
      const res: CacheCleanupResult = await api.clearDownloadCache(settings.downloadDirectory);
      const mbFreed = (res.bytesFreed / (1024 * 1024)).toFixed(1);
      setCleanupMessage(`Removed ${res.filesRemoved} temporary archive(s), freeing ${mbFreed} MB of disk space.`);
      setTimeout(() => setCleanupMessage(null), 5000);
    } catch (e: any) {
      setCleanupMessage(`Failed to clean download cache: ${e?.message || e}`);
    } finally {
      setCleaningDownloads(false);
    }
  };

  const handlePurgeStaleBackups = async () => {
    setCleaningBackups(true);
    setCleanupMessage(null);
    try {
      const res: CacheCleanupResult = await api.purgeStaleBackups();
      const mbFreed = (res.bytesFreed / (1024 * 1024)).toFixed(1);
      setCleanupMessage(`Purged ${res.filesRemoved} obsolete backup file(s), freeing ${mbFreed} MB.`);
      setTimeout(() => setCleanupMessage(null), 5000);
    } catch (e: any) {
      setCleanupMessage(`Failed to purge backups: ${e?.message || e}`);
    } finally {
      setCleaningBackups(false);
    }
  };

  const addLaunchArg = (arg: string) => {
    if (!settings) return;
    const current = settings.defaultLaunchArgs || '';
    if (!current.includes(arg)) {
      const updated = current.trim() ? `${current.trim()} ${arg}` : arg;
      setSettings({ ...settings, defaultLaunchArgs: updated });
    }
  };

  if (!settings) {
    return (
      <div className="p-8 max-w-5xl mx-auto flex items-center justify-center min-h-[400px]">
        <div className="text-xs text-neutral-400 font-medium animate-pulse">Loading application settings...</div>
      </div>
    );
  }

  return (
    <div className="p-6 md:p-8 space-y-6 max-w-5xl mx-auto">
      {/* Header and Save Action */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border-subtle pb-5">
        <div>
          <div className="flex items-center gap-2">
            <SettingsIcon className="w-5 h-5 text-accent" />
            <h1 className="text-xl font-bold tracking-tight text-white">Application Settings</h1>
          </div>
          <p className="text-xs text-neutral-400 mt-1">
            Configure game launch environments, visual themes, storage paths, and transactional conflict handling.
          </p>
        </div>

        <button
          onClick={handleSave}
          disabled={loading}
          className={`h-9 px-5 rounded-md font-bold text-xs flex items-center gap-2 transition-all shadow-md self-start sm:self-auto cursor-pointer ${
            saved
              ? 'bg-emerald-600 text-white'
              : 'bg-accent hover:bg-accent-hover text-bg-main active:scale-[0.98]'
          }`}
        >
          {saved ? <Check className="w-4 h-4" /> : <Save className="w-4 h-4" />}
          <span>{saved ? 'Settings Saved' : 'Save Settings'}</span>
        </button>
      </div>

      {cleanupMessage && (
        <div className="p-3 bg-accent/15 border border-accent/40 rounded-md text-xs text-amber-200 flex items-center gap-2 animate-in fade-in">
          <Info className="w-4 h-4 text-accent flex-shrink-0" />
          <span>{cleanupMessage}</span>
        </div>
      )}

      {/* Grid of Settings Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Card 1: Game Launch & Display Engine */}
        <div className="bg-bg-card border border-border rounded-lg p-5 space-y-4 flex flex-col justify-between">
          <div className="space-y-4">
            <div className="flex items-center gap-2 pb-2 border-b border-border-subtle">
              <Gamepad2 className="w-4 h-4 text-accent" />
              <h2 className="text-xs font-bold uppercase tracking-wider text-neutral-200">
                Game Engine & Launch Configuration
              </h2>
            </div>

            <div>
              <label className="block text-xs font-semibold text-neutral-300 mb-1">
                Default Launch Arguments
              </label>
              <input
                type="text"
                value={settings.defaultLaunchArgs || ''}
                onChange={(e) => setSettings({ ...settings, defaultLaunchArgs: e.target.value })}
                placeholder="e.g. -novid -window"
                className="w-full h-9 bg-bg-main border border-border rounded px-3 text-xs text-white font-mono focus:outline-none focus:border-accent"
              />
              <div className="flex items-center gap-1.5 mt-2 flex-wrap text-[11px]">
                <span className="text-neutral-500 font-mono text-[10px]">Quick Flags:</span>
                <button
                  type="button"
                  onClick={() => addLaunchArg('-novid')}
                  className="px-2 py-0.5 rounded bg-bg-main hover:bg-neutral-800 border border-border text-neutral-300 text-[10px] font-mono transition-colors"
                >
                  + -novid (Skip Movies)
                </button>
                <button
                  type="button"
                  onClick={() => addLaunchArg('-window')}
                  className="px-2 py-0.5 rounded bg-bg-main hover:bg-neutral-800 border border-border text-neutral-300 text-[10px] font-mono transition-colors"
                >
                  + -window (Windowed)
                </button>
              </div>
            </div>

            <div>
              <label className="block text-xs font-semibold text-neutral-300 mb-1">
                Widescreen Fix Aspect Ratio Default
              </label>
              <select
                value={settings.defaultAspectRatio || '16:9'}
                onChange={(e) => setSettings({ ...settings, defaultAspectRatio: e.target.value })}
                className="w-full h-9 bg-bg-main border border-border rounded px-3 text-xs text-white focus:outline-none focus:border-accent cursor-pointer"
              >
                <option value="16:9">16:9 (Standard Modern Widescreen - Recommended)</option>
                <option value="16:10">16:10 (PC Laptop / Monitor)</option>
                <option value="21:9">21:9 (Ultrawide Monitor)</option>
                <option value="32:9">32:9 (Super Ultrawide)</option>
                <option value="4:3">4:3 (Classic CRT Aspect Ratio)</option>
              </select>
              <p className="text-[11px] text-neutral-500 mt-1">
                Ensures GTA: San Andreas GUI and loading screens match your native monitor geometry without stretching.
              </p>
            </div>

            <div className="pt-2">
              <label className="flex items-center gap-2.5 cursor-pointer text-xs text-neutral-300">
                <input
                  type="checkbox"
                  checked={settings.autoDetectGameOnStartup ?? true}
                  onChange={(e) =>
                    setSettings({ ...settings, autoDetectGameOnStartup: e.target.checked })
                  }
                  className="accent-amber-500 rounded"
                />
                <span>Auto-scan standard Steam and Rockstar paths on startup</span>
              </label>
            </div>
          </div>
        </div>

        {/* Card 2: Modding & Conflict Prevention */}
        <div className="bg-bg-card border border-border rounded-lg p-5 space-y-4 flex flex-col justify-between">
          <div className="space-y-4">
            <div className="flex items-center gap-2 pb-2 border-b border-border-subtle">
              <Shield className="w-4 h-4 text-accent" />
              <h2 className="text-xs font-bold uppercase tracking-wider text-neutral-200">
                Installation & Conflict Safety
              </h2>
            </div>

            <div className="space-y-3 pt-1">
              <label className="flex items-start gap-2.5 cursor-pointer text-xs text-neutral-300">
                <input
                  type="checkbox"
                  checked={settings.requireInstallConfirmation}
                  onChange={(e) =>
                    setSettings({ ...settings, requireInstallConfirmation: e.target.checked })
                  }
                  className="accent-amber-500 rounded mt-0.5"
                />
                <div>
                  <span className="font-semibold text-white block">Pre-Installation Manifest Inspection</span>
                  <span className="text-[11px] text-neutral-400 block mt-0.5">
                    Displays an interactive file routing plan before placing files into the game directory or ModLoader.
                  </span>
                </div>
              </label>

              <label className="flex items-start gap-2.5 cursor-pointer text-xs text-neutral-300">
                <input
                  type="checkbox"
                  checked={settings.autoResolveConflicts ?? true}
                  onChange={(e) =>
                    setSettings({ ...settings, autoResolveConflicts: e.target.checked })
                  }
                  className="accent-amber-500 rounded mt-0.5"
                />
                <div>
                  <span className="font-semibold text-white block">Automatic Duplicate Conflict Resolution</span>
                  <span className="text-[11px] text-neutral-400 block mt-0.5">
                    Automatically isolates duplicate ASI files (e.g. root vs ModLoader WidescreenFix) to prevent crashing on game boot.
                  </span>
                </div>
              </label>
            </div>

            <div className="pt-2 border-t border-border-subtle">
              <label className="block text-xs font-semibold text-neutral-300 mb-1">
                Maximum Concurrent Downloads
              </label>
              <div className="flex items-center gap-3">
                <input
                  type="number"
                  min="1"
                  max="10"
                  value={settings.maxConcurrentDownloads}
                  onChange={(e) =>
                    setSettings({ ...settings, maxConcurrentDownloads: Math.max(1, Math.min(10, parseInt(e.target.value) || 3)) })
                  }
                  className="w-24 h-9 bg-bg-main border border-border rounded px-3 text-xs text-white focus:outline-none focus:border-accent font-mono"
                />
                <span className="text-[11px] text-neutral-400">
                  Worker threads for parallel mod downloads (1 to 10).
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Card 3: Storage & Directory Repositories */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-5">
        <div className="flex items-center gap-2 pb-2 border-b border-border-subtle">
          <HardDrive className="w-4 h-4 text-accent" />
          <h2 className="text-xs font-bold uppercase tracking-wider text-neutral-200">
            Storage Repositories & Cache Maintenance
          </h2>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
          {/* Downloads Directory */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <label className="text-xs font-semibold text-neutral-300">Downloads Storage Directory</label>
              <button
                type="button"
                onClick={() => api.openPathInExplorer(settings.downloadDirectory)}
                className="text-[11px] text-accent hover:underline flex items-center gap-1 cursor-pointer"
                title="Open directory in Windows Explorer"
              >
                <FolderOpen className="w-3 h-3" />
                <span>Open Folder</span>
              </button>
            </div>
            <div className="flex items-center gap-2">
              <input
                type="text"
                value={settings.downloadDirectory}
                onChange={(e) => setSettings({ ...settings, downloadDirectory: e.target.value })}
                className="flex-1 h-9 bg-bg-main border border-border rounded px-3 text-xs text-white font-mono focus:outline-none focus:border-accent"
              />
              <button
                type="button"
                onClick={handleBrowseDownloadDir}
                className="h-9 px-3 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-200 font-medium flex items-center gap-1.5 transition-colors cursor-pointer flex-shrink-0"
              >
                <Folder className="w-3.5 h-3.5" />
                <span>Browse...</span>
              </button>
            </div>
            <div className="flex items-center justify-between pt-1">
              <span className="text-[11px] text-neutral-500">
                Downloaded `.zip`, `.rar`, and `.7z` packages.
              </span>
              <button
                type="button"
                onClick={handleCleanDownloadCache}
                disabled={cleaningDownloads}
                className="text-[11px] text-rose-400 hover:text-rose-300 flex items-center gap-1 cursor-pointer transition-colors"
                title="Deletes temporary archive files from downloads directory"
              >
                <Trash2 className="w-3 h-3" />
                <span>{cleaningDownloads ? 'Cleaning...' : 'Clean Cache'}</span>
              </button>
            </div>
          </div>

          {/* Backups Directory */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <label className="text-xs font-semibold text-neutral-300">Filesystem Backup Directory</label>
              <button
                type="button"
                onClick={() => api.openPathInExplorer(settings.backupDirectory)}
                className="text-[11px] text-accent hover:underline flex items-center gap-1 cursor-pointer"
                title="Open directory in Windows Explorer"
              >
                <FolderOpen className="w-3 h-3" />
                <span>Open Folder</span>
              </button>
            </div>
            <div className="flex items-center gap-2">
              <input
                type="text"
                value={settings.backupDirectory}
                onChange={(e) => setSettings({ ...settings, backupDirectory: e.target.value })}
                className="flex-1 h-9 bg-bg-main border border-border rounded px-3 text-xs text-white font-mono focus:outline-none focus:border-accent"
              />
              <button
                type="button"
                onClick={handleBrowseBackupDir}
                className="h-9 px-3 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-200 font-medium flex items-center gap-1.5 transition-colors cursor-pointer flex-shrink-0"
              >
                <Folder className="w-3.5 h-3.5" />
                <span>Browse...</span>
              </button>
            </div>
            <div className="flex items-center justify-between pt-1">
              <span className="text-[11px] text-neutral-500">
                Original vanilla files preserved for 1-click rollback.
              </span>
              <button
                type="button"
                onClick={handlePurgeStaleBackups}
                disabled={cleaningBackups}
                className="text-[11px] text-rose-400 hover:text-rose-300 flex items-center gap-1 cursor-pointer transition-colors"
                title="Purges unreferenced backup files"
              >
                <Trash2 className="w-3 h-3" />
                <span>{cleaningBackups ? 'Purging...' : 'Purge Stale Backups'}</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Card 4: Appearance & Visual Themes */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-4">
        <div className="flex items-center gap-2 pb-2 border-b border-border-subtle">
          <Palette className="w-4 h-4 text-accent" />
          <h2 className="text-xs font-bold uppercase tracking-wider text-neutral-200">
            Interface Theme & Visual Appearance
          </h2>
        </div>

        <p className="text-xs text-neutral-400">
          Select a high-density color scheme. Themes apply in real-time across all views.
        </p>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
          {THEMES.map((t) => {
            const isSelected = settings.theme === t.id;
            return (
              <button
                key={t.id}
                type="button"
                onClick={() => handleThemeChange(t.id)}
                className={`p-3 rounded-lg border text-left transition-all relative flex flex-col justify-between cursor-pointer ${
                  isSelected
                    ? 'border-accent bg-accent/10 ring-1 ring-accent'
                    : 'border-border bg-bg-main hover:border-neutral-500'
                }`}
              >
                <div className="flex items-center justify-between mb-2">
                  <span className="text-xs font-bold text-white flex items-center gap-1.5">
                    <span
                      className="w-3 h-3 rounded-full border border-black/30"
                      style={{ backgroundColor: t.previewAccent }}
                    />
                    {t.name}
                  </span>
                  {isSelected && (
                    <span className="text-[10px] font-mono font-bold text-accent px-1.5 py-0.2 rounded bg-accent/20">
                      Active
                    </span>
                  )}
                </div>
                <p className="text-[11px] text-neutral-400 leading-tight mb-3">{t.desc}</p>
                <div className="flex items-center gap-1.5 h-3">
                  <div className="flex-1 h-2 rounded-sm border" style={{ backgroundColor: t.previewBg, borderColor: t.previewBorder }} />
                  <div className="flex-1 h-2 rounded-sm" style={{ backgroundColor: t.previewCard }} />
                  <div className="w-4 h-2 rounded-sm" style={{ backgroundColor: t.previewAccent }} />
                </div>
              </button>
            );
          })}
        </div>
      </div>

      {/* Card 5: About & GitHub Repository Information */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-4">
        <div className="flex items-center justify-between pb-2 border-b border-border-subtle">
          <div className="flex items-center gap-2">
            <Layers className="w-4 h-4 text-accent" />
            <h2 className="text-xs font-bold uppercase tracking-wider text-neutral-200">
              About San Andreas Mod Manager (SAMM)
            </h2>
          </div>
          <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-neutral-800 text-neutral-300 border border-neutral-700">
            Version 0.1.0 (Production Release)
          </span>
        </div>

        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div className="space-y-1">
            <div className="text-xs text-white font-semibold">
              Engineered for High-Performance GTA: San Andreas Mod Management
            </div>
            <p className="text-[11px] text-neutral-400 max-w-xl">
              Built on Tauri v2, Rust native transactional file management, and React 18. Guarantees zero-data-loss rollback, automatic RenderWare aspect ratio enforcement, and single-click mod deployment.
            </p>
          </div>

          <button
            type="button"
            onClick={() => api.openUrl('https://github.com/athuls-engineer/samm')}
            className="h-9 px-4 rounded-md bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-white font-semibold flex items-center gap-1.5 transition-colors flex-shrink-0 cursor-pointer shadow-sm"
          >
            <ExternalLink className="w-3.5 h-3.5 text-accent" />
            <span>GitHub Repository</span>
          </button>
        </div>
      </div>
    </div>
  );
};
