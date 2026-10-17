import React, { useState } from 'react';
import { GameInstallation, InstalledMod, Profile } from '../types';
import { api } from '../api/client';
import {
  Gamepad2,
  Package,
  Layers,
  AlertTriangle,
  History,
  Play,
  CheckCircle2,
  AlertCircle,
  ExternalLink,
  FolderOpen,
  Square,
  ShieldCheck,
  Zap,
  ChevronRight,
} from 'lucide-react';

interface HomeProps {
  activeGame: GameInstallation | null;
  activeProfile: Profile | null;
  installedMods: InstalledMod[];
  conflictsCount: number;
  backupsCount: number;
  isGameRunning?: boolean;
  onNavigate: (tab: any) => void;
  onLaunchGame: (customArgs: string[]) => void;
  onKillGame?: () => void;
  onToast?: (text: string, type?: 'success' | 'error') => void;
}

export const Home: React.FC<HomeProps> = ({
  activeGame,
  activeProfile,
  installedMods,
  conflictsCount,
  backupsCount,
  isGameRunning = false,
  onNavigate,
  onLaunchGame,
  onKillGame,
  onToast,
}) => {
  const [argWindowed, setArgWindowed] = useState(false);
  const [argNoIntro, setArgNoIntro] = useState(true);
  const [arg1080p, setArg1080p] = useState(false);
  const [launching, setLaunching] = useState(false);

  const handleLaunch = () => {
    setLaunching(true);
    const args: string[] = [];
    if (argWindowed) args.push('-window');
    if (argNoIntro) args.push('-novid');
    if (arg1080p) {
      args.push('-width');
      args.push('1920');
      args.push('-height');
      args.push('1080');
    }

    onLaunchGame(args);
    setTimeout(() => setLaunching(false), 2000);
  };

  const handleOpenFolder = async () => {
    if (!activeGame) return;
    try {
      await api.openPathInExplorer(activeGame.rootPath);
    } catch (e: any) {
      onToast?.(`Failed to open folder: ${e?.message || e}`, 'error');
    }
  };

  return (
    <div className="p-6 md:p-8 space-y-6 max-w-6xl mx-auto">
      {/* Active Game Target Banner */}
      {activeGame ? (
        <div className="bg-bg-card border border-border rounded-lg p-5 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 shadow-sm">
          <div className="space-y-1.5 min-w-0">
            <div className="flex items-center gap-2.5 flex-wrap">
              <Gamepad2 className="w-5 h-5 text-accent flex-shrink-0" />
              <h1 className="text-base md:text-lg font-bold text-white truncate">{activeGame.name}</h1>
              {activeGame.isClassic10 && activeGame.versionConfidence === 'confirmed' ? (
                <span className="text-[11px] font-semibold text-emerald-400 bg-emerald-950/70 border border-emerald-800 px-2 py-0.5 rounded flex items-center gap-1">
                  <CheckCircle2 className="w-3.5 h-3.5" /> Classic 1.0 Baseline (Verified)
                </span>
              ) : (
                <span className="text-[11px] font-semibold text-amber-400 bg-amber-950/70 border border-amber-800 px-2 py-0.5 rounded flex items-center gap-1">
                  <AlertCircle className="w-3.5 h-3.5" /> {activeGame.detectedEdition}
                </span>
              )}

              {isGameRunning && (
                <span className="text-[11px] font-semibold text-emerald-400 bg-emerald-950/80 border border-emerald-700 px-2 py-0.5 rounded flex items-center gap-1 animate-pulse">
                  <span className="w-2 h-2 rounded-full bg-emerald-400"></span>
                  GTA:SA Running
                </span>
              )}
            </div>

            <div className="text-xs text-neutral-400 font-mono flex items-center gap-2 truncate">
              <span className="text-neutral-500">Directory:</span>
              <span className="text-neutral-300 truncate">{activeGame.rootPath}</span>
            </div>

            <div className="text-[11px] text-neutral-400 font-mono">
              Executable: {activeGame.executablePath.split(/[\\/]/).pop()} ({(activeGame.executableSizeBytes / (1024 * 1024)).toFixed(2)} MB)
            </div>
          </div>

          <div className="flex items-center gap-2 self-start md:self-auto flex-shrink-0">
            <button
              onClick={handleOpenFolder}
              className="text-xs font-semibold px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 flex items-center gap-1.5 transition-colors"
              title="Open game directory in Windows Explorer"
            >
              <FolderOpen className="w-3.5 h-3.5 text-accent" />
              <span>Open Folder</span>
            </button>

            <button
              onClick={() => onNavigate('installations')}
              className="text-xs font-semibold px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-300 border border-neutral-700 flex items-center gap-1.5 transition-colors"
            >
              <span>Switch Game</span>
              <ExternalLink className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
      ) : (
        <div className="bg-amber-950/30 border border-amber-800/80 rounded-lg p-6 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
          <div className="space-y-1">
            <h2 className="text-sm font-bold text-amber-200 flex items-center gap-2">
              <AlertCircle className="w-5 h-5 text-amber-400" />
              No GTA: San Andreas Game Target Configured
            </h2>
            <p className="text-xs text-amber-300/80">
              Select or browse for your GTA San Andreas installation folder to begin managing mods and plugins.
            </p>
          </div>
          <button
            onClick={() => onNavigate('installations')}
            className="px-4 py-2 bg-accent hover:bg-accent-hover text-bg-main font-bold text-xs rounded transition-colors shadow"
          >
            Locate Game Folder
          </button>
        </div>
      )}

      {/* Metrics Overview Cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div
          onClick={() => onNavigate('installed')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">Installed Mods</span>
            <Package className="w-4 h-4 text-accent" />
          </div>
          <div className="text-2xl font-black text-white font-mono">{installedMods.length}</div>
          <div className="text-[11px] text-neutral-400">
            {installedMods.filter((m) => m.enabled).length} active, {installedMods.filter((m) => !m.enabled).length} disabled
          </div>
        </div>

        <div
          onClick={() => onNavigate('profiles')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">Active Profile</span>
            <Layers className="w-4 h-4 text-accent" />
          </div>
          <div className="text-sm font-bold text-white truncate">
            {activeProfile?.name || 'Default Profile'}
          </div>
          <div className="text-[11px] text-neutral-400 truncate">
            {activeProfile?.description || 'Standard profile'}
          </div>
        </div>

        <div
          onClick={() => onNavigate('conflicts')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">File Conflicts</span>
            <AlertTriangle className={`w-4 h-4 ${conflictsCount > 0 ? 'text-amber-400' : 'text-neutral-500'}`} />
          </div>
          <div className={`text-2xl font-black font-mono ${conflictsCount > 0 ? 'text-amber-400' : 'text-white'}`}>
            {conflictsCount}
          </div>
          <div className="text-[11px] text-neutral-400">
            {conflictsCount === 0 ? 'Zero conflicts' : `${conflictsCount} conflicts detected`}
          </div>
        </div>

        <div
          onClick={() => onNavigate('backups')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">Backups</span>
            <History className="w-4 h-4 text-accent" />
          </div>
          <div className="text-2xl font-black text-white font-mono">{backupsCount}</div>
          <div className="text-[11px] text-neutral-400">Clean original vanilla files</div>
        </div>
      </div>

      {/* Game Launch Configuration */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-4">
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
          <div>
            <h2 className="text-sm font-bold text-white flex items-center gap-2">
              <Play className="w-4 h-4 text-accent fill-current" />
              Launch Options
            </h2>
            <p className="text-xs text-neutral-400">
              Launch GTA: San Andreas with profile loadouts and custom engine parameters.
            </p>
          </div>

          <div className="flex items-center gap-2">
            {isGameRunning && onKillGame && (
              <button
                onClick={onKillGame}
                className="px-4 py-2 rounded bg-rose-950/90 hover:bg-rose-900 border border-rose-800 text-xs font-semibold text-rose-300 flex items-center gap-1.5 transition-colors shadow"
                title="Force kill hanging gta_sa.exe process"
              >
                <Square className="w-3.5 h-3.5 fill-current" />
                <span>Kill Game</span>
              </button>
            )}

            <button
              onClick={handleLaunch}
              disabled={!activeGame || launching}
              className={`px-6 py-2 rounded font-bold text-xs flex items-center gap-2 transition-colors shadow ${
                activeGame && !launching
                  ? 'bg-accent hover:bg-accent-hover text-bg-main'
                  : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
              }`}
            >
              <Play className="w-3.5 h-3.5 fill-current" />
              <span>{launching ? 'Starting Process...' : 'Launch GTA: San Andreas'}</span>
            </button>
          </div>
        </div>

        {/* Startup Flags */}
        <div className="flex flex-wrap gap-4 pt-3 border-t border-border-subtle text-xs text-neutral-300">
          <label className="flex items-center gap-2 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={argNoIntro}
              onChange={(e) => setArgNoIntro(e.target.checked)}
              className="accent-amber-500 rounded"
            />
            <span>Skip Intro Videos (<code className="text-neutral-400 font-mono">-novid</code>)</span>
          </label>

          <label className="flex items-center gap-2 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={argWindowed}
              onChange={(e) => setArgWindowed(e.target.checked)}
              className="accent-amber-500 rounded"
            />
            <span>Windowed Mode (<code className="text-neutral-400 font-mono">-window</code>)</span>
          </label>

          <label className="flex items-center gap-2 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={arg1080p}
              onChange={(e) => setArg1080p(e.target.checked)}
              className="accent-amber-500 rounded"
            />
            <span>Force 1080p (<code className="text-neutral-400 font-mono">-width 1920 -height 1080</code>)</span>
          </label>
        </div>
      </div>

      {/* Active Mod Manifest Preview */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-3">
        <div className="flex items-center justify-between">
          <h2 className="text-xs font-bold text-white uppercase tracking-wider">
            Active Mods ({installedMods.length})
          </h2>
          <button
            onClick={() => onNavigate('installed')}
            className="text-xs text-accent hover:text-amber-300 font-semibold flex items-center gap-1 transition-colors"
          >
            <span>Manage All Mods</span>
            <ChevronRight className="w-3.5 h-3.5" />
          </button>
        </div>

        {installedMods.length === 0 ? (
          <div className="py-8 text-center text-xs text-neutral-400 space-y-2">
            <div>No mods currently registered in this game directory.</div>
            <button
              onClick={() => onNavigate('discover')}
              className="px-3 py-1.5 bg-neutral-800 hover:bg-neutral-700 text-neutral-200 rounded font-semibold text-xs transition-colors"
            >
              Browse Discovery Catalog
            </button>
          </div>
        ) : (
          <div className="divide-y divide-border-subtle font-mono text-xs">
            {installedMods.slice(0, 6).map((mod) => (
              <div key={mod.id} className="py-2.5 flex items-center justify-between">
                <div>
                  <div className="font-semibold text-neutral-200">{mod.name}</div>
                  <div className="text-[11px] text-neutral-400 font-sans">
                    Strategy: {mod.strategy} | {mod.files.length} tracked {mod.files.length === 1 ? 'file' : 'files'}
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <span
                    className={`text-[10px] font-bold px-2 py-0.5 rounded ${
                      mod.enabled
                        ? 'bg-emerald-950 text-emerald-400 border border-emerald-800'
                        : 'bg-neutral-800 text-neutral-400'
                    }`}
                  >
                    {mod.enabled ? 'ENABLED' : 'DISABLED'}
                  </span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
