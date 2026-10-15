import React, { useState } from 'react';
import { GameInstallation, InstalledMod, Profile } from '../types';
import {
  Gamepad2,
  Package,
  Layers,
  AlertTriangle,
  History,
  Play,
  CheckCircle2,
  AlertCircle,
  HelpCircle,
  ExternalLink,
  ShieldCheck,
} from 'lucide-react';

interface HomeProps {
  activeGame: GameInstallation | null;
  activeProfile: Profile | null;
  installedMods: InstalledMod[];
  conflictsCount: number;
  backupsCount: number;
  onNavigate: (tab: any) => void;
  onLaunchGame: (customArgs: string[]) => void;
}

export const Home: React.FC<HomeProps> = ({
  activeGame,
  activeProfile,
  installedMods,
  conflictsCount,
  backupsCount,
  onNavigate,
  onLaunchGame,
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

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      {/* Welcome / Active Game Banner */}
      {activeGame ? (
        <div className="bg-bg-card border border-border rounded-lg p-5 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
          <div className="space-y-1.5">
            <div className="flex items-center gap-2.5">
              <Gamepad2 className="w-6 h-6 text-accent" />
              <h1 className="text-lg font-bold text-white">{activeGame.name}</h1>
              {activeGame.isClassic10 && activeGame.versionConfidence === 'confirmed' ? (
                <span className="text-[11px] font-semibold text-emerald-400 bg-emerald-950/70 border border-emerald-800 px-2 py-0.5 rounded flex items-center gap-1">
                  <CheckCircle2 className="w-3.5 h-3.5" /> Classic 1.0 Baseline (Verified)
                </span>
              ) : (
                <span className="text-[11px] font-semibold text-amber-400 bg-amber-950/70 border border-amber-800 px-2 py-0.5 rounded flex items-center gap-1">
                  <AlertCircle className="w-3.5 h-3.5" /> {activeGame.detectedEdition}
                </span>
              )}
            </div>
            <div className="text-xs text-neutral-400 font-mono flex items-center gap-2">
              <span className="text-neutral-500">Target:</span>
              <span className="text-neutral-300">{activeGame.executablePath}</span>
            </div>
            <div className="text-[11px] text-neutral-400 font-mono">
              SHA-256: {activeGame.executableSha256 || 'N/A'} ({(activeGame.executableSizeBytes / (1024 * 1024)).toFixed(2)} MB)
            </div>
          </div>

          <button
            onClick={() => onNavigate('installations')}
            className="text-xs font-semibold px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-300 border border-neutral-700 flex items-center gap-1.5 transition-colors self-end md:self-auto"
          >
            <span>Inspect Evidence</span>
            <ExternalLink className="w-3.5 h-3.5" />
          </button>
        </div>
      ) : (
        <div className="bg-amber-950/30 border border-amber-800/80 rounded-lg p-6 flex items-center justify-between">
          <div className="space-y-1">
            <h2 className="text-sm font-bold text-amber-200 flex items-center gap-2">
              <AlertCircle className="w-5 h-5 text-amber-400" />
              No GTA: San Andreas Game Installation Selected
            </h2>
            <p className="text-xs text-amber-300/80">
              Locate or detect your game directory to verify compatibility with the Classic PC 1.0 baseline.
            </p>
          </div>
          <button
            onClick={() => onNavigate('installations')}
            className="px-4 py-2 bg-accent hover:bg-accent-hover text-bg-main font-bold text-xs rounded transition-colors shadow"
          >
            Locate Game Directory
          </button>
        </div>
      )}

      {/* Metric Cards Grid */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div
          onClick={() => onNavigate('installed')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">Installed Mods</span>
            <Package className="w-4 h-4 text-accent" />
          </div>
          <div className="text-2xl font-black text-white">{installedMods.length}</div>
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
            {activeProfile?.description || 'Isolated mod sandbox'}
          </div>
        </div>

        <div
          onClick={() => onNavigate('conflicts')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">File Collisions</span>
            <AlertTriangle className={`w-4 h-4 ${conflictsCount > 0 ? 'text-amber-400' : 'text-neutral-500'}`} />
          </div>
          <div className={`text-2xl font-black ${conflictsCount > 0 ? 'text-amber-400' : 'text-white'}`}>
            {conflictsCount}
          </div>
          <div className="text-[11px] text-neutral-400">
            {conflictsCount === 0 ? 'Zero collisions detected' : 'Requires resolution review'}
          </div>
        </div>

        <div
          onClick={() => onNavigate('backups')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">Recovery Points</span>
            <History className="w-4 h-4 text-accent" />
          </div>
          <div className="text-2xl font-black text-white">{backupsCount}</div>
          <div className="text-[11px] text-neutral-400">Original files preserved safely</div>
        </div>
      </div>

      {/* Launch Configuration Strip */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-sm font-bold text-white flex items-center gap-2">
              <Play className="w-4 h-4 text-accent fill-current" />
              Game Launch Configuration
            </h2>
            <p className="text-xs text-neutral-400">
              Launch GTA: San Andreas with selected mod profile and custom startup flags.
            </p>
          </div>

          <button
            onClick={handleLaunch}
            disabled={!activeGame || launching}
            className={`px-6 py-2.5 rounded font-bold text-xs flex items-center gap-2 transition-colors shadow ${
              activeGame && !launching
                ? 'bg-accent hover:bg-accent-hover text-bg-main'
                : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
            }`}
          >
            <Play className="w-4 h-4 fill-current" />
            <span>{launching ? 'Launching...' : 'Launch Grand Theft Auto: San Andreas'}</span>
          </button>
        </div>

        {/* Options */}
        <div className="flex flex-wrap gap-4 pt-2 border-t border-border-subtle text-xs text-neutral-300">
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={argNoIntro}
              onChange={(e) => setArgNoIntro(e.target.checked)}
              className="accent-amber-500 rounded"
            />
            <span>Skip Intro Videos (<code className="text-neutral-400">-novid</code>)</span>
          </label>

          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={argWindowed}
              onChange={(e) => setArgWindowed(e.target.checked)}
              className="accent-amber-500 rounded"
            />
            <span>Windowed Mode (<code className="text-neutral-400">-window</code>)</span>
          </label>

          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={arg1080p}
              onChange={(e) => setArg1080p(e.target.checked)}
              className="accent-amber-500 rounded"
            />
            <span>Force 1080p Startup (<code className="text-neutral-400">-width 1920 -height 1080</code>)</span>
          </label>
        </div>
      </div>

      {/* Recent Installed Mods */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-3">
        <div className="flex items-center justify-between">
          <h2 className="text-xs font-bold text-white uppercase tracking-wider">
            Active Installed Mods ({installedMods.length})
          </h2>
          <button
            onClick={() => onNavigate('installed')}
            className="text-xs text-accent hover:underline font-semibold"
          >
            Manage All
          </button>
        </div>

        {installedMods.length === 0 ? (
          <div className="py-8 text-center text-xs text-neutral-400 space-y-2">
            <div>No mods currently installed in this game directory.</div>
            <button
              onClick={() => onNavigate('discover')}
              className="px-3 py-1.5 bg-neutral-800 hover:bg-neutral-700 text-neutral-200 rounded font-semibold text-xs transition-colors"
            >
              Browse Discovery Catalog
            </button>
          </div>
        ) : (
          <div className="divide-y divide-border-subtle font-mono text-xs">
            {installedMods.slice(0, 5).map((mod) => (
              <div key={mod.id} className="py-2.5 flex items-center justify-between">
                <div>
                  <div className="font-semibold text-neutral-200">{mod.name}</div>
                  <div className="text-[11px] text-neutral-400 font-sans">
                    Strategy: {mod.strategy} | {mod.files.length} files installed
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
