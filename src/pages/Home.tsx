import React, { useState, useEffect } from 'react';
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
  Monitor,
  VolumeX,
  Shield,
  Terminal,
  Save,
  RotateCcw,
  SlidersHorizontal,
  ChevronDown,
  Sparkles,
  Loader2,
} from 'lucide-react';

interface HomeProps {
  activeGame: GameInstallation | null;
  activeProfile: Profile | null;
  profiles?: Profile[];
  onSelectProfile?: (profileId: string) => void;
  installedMods: InstalledMod[];
  conflictsCount: number;
  duplicatesCount?: number;
  backupsCount: number;
  isGameRunning?: boolean;
  onNavigate: (tab: any) => void;
  onLaunchGame: (customArgs: string[]) => Promise<any> | void;
  onKillGame?: () => void;
  onRestartGame?: (customArgs?: string[]) => void;
  onToast?: (text: string, type?: 'success' | 'error') => void;
}

export const Home: React.FC<HomeProps> = ({
  activeGame,
  activeProfile,
  profiles = [],
  onSelectProfile,
  installedMods,
  conflictsCount,
  duplicatesCount = 0,
  backupsCount,
  isGameRunning = false,
  onNavigate,
  onLaunchGame,
  onKillGame,
  onRestartGame,
  onToast,
}) => {
  // Launch options flags
  const [argNoIntro, setArgNoIntro] = useState(true);
  const [argWindowed, setArgWindowed] = useState(false);
  const [argNoSound, setArgNoSound] = useState(false);
  const [argSafeMode, setArgSafeMode] = useState(false);
  const [argDebug, setArgDebug] = useState(false);
  const [argResolution, setArgResolution] = useState<'native' | '1080p' | '1440p' | '4k' | 'custom'>('native');
  const [customWidth, setCustomWidth] = useState('1920');
  const [customHeight, setCustomHeight] = useState('1080');
  const [argScreen, setArgScreen] = useState('0');
  const [customArgsInput, setCustomArgsInput] = useState('');
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [launching, setLaunching] = useState(false);
  const [savingDefault, setSavingDefault] = useState(false);

  // Load defaults from application settings on mount
  useEffect(() => {
    api.getSettings().then((s) => {
      if (s?.defaultLaunchArgs) {
        const raw = s.defaultLaunchArgs.trim();
        setCustomArgsInput(raw);
        if (raw.includes('-novid')) setArgNoIntro(true);
        if (raw.includes('-window')) setArgWindowed(true);
        if (raw.includes('-nosound')) setArgNoSound(true);
        if (raw.includes('-safemode')) setArgSafeMode(true);
        if (raw.includes('-debug')) setArgDebug(true);
        if (raw.includes('-width 1920') && raw.includes('-height 1080')) {
          setArgResolution('1080p');
        } else if (raw.includes('-width 2560') && raw.includes('-height 1440')) {
          setArgResolution('1440p');
        } else if (raw.includes('-width 3840') && raw.includes('-height 2160')) {
          setArgResolution('4k');
        }
      }
    }).catch(() => {});
  }, []);

  const buildArgs = (overrideSafeMode = false): string[] => {
    const args: string[] = [];

    if (overrideSafeMode || argSafeMode) {
      args.push('-safemode');
    }
    if (argNoIntro) args.push('-novid');
    if (argWindowed) args.push('-window');
    if (argNoSound) args.push('-nosound');
    if (argDebug) args.push('-debug');

    if (argScreen && argScreen !== '0') {
      args.push('-screen', argScreen);
    }

    if (argResolution === '1080p') {
      args.push('-width', '1920', '-height', '1080');
    } else if (argResolution === '1440p') {
      args.push('-width', '2560', '-height', '1440');
    } else if (argResolution === '4k') {
      args.push('-width', '3840', '-height', '2160');
    } else if (argResolution === 'custom' && customWidth && customHeight) {
      args.push('-width', customWidth, '-height', customHeight);
    }

    // Add extra user-typed flags (tokenized cleanly)
    if (customArgsInput.trim()) {
      for (const token of customArgsInput.trim().split(/\s+/)) {
        if (token && !args.includes(token)) {
          args.push(token);
        }
      }
    }

    return args;
  };

  const handleLaunch = async (safeMode = false) => {
    if (launching || !activeGame) return;
    setLaunching(true);
    const args = buildArgs(safeMode);
    try {
      await onLaunchGame(args);
    } catch (e: any) {
      console.warn('Game launch caught in Home:', e);
    } finally {
      setTimeout(() => setLaunching(false), 2000);
    }
  };

  const handleSaveDefaults = async () => {
    setSavingDefault(true);
    try {
      const s = await api.getSettings();
      const currentArgs = buildArgs().join(' ');
      await api.saveSettings({ ...s, defaultLaunchArgs: currentArgs });
      onToast?.('Launch options saved as default startup configuration.', 'success');
    } catch (e: any) {
      onToast?.(`Failed to save launch defaults: ${e?.message || e}`, 'error');
    } finally {
      setSavingDefault(false);
    }
  };

  const toggleQuickFlag = (flag: string) => {
    const parts = customArgsInput.split(/\s+/).filter(Boolean);
    if (parts.includes(flag)) {
      setCustomArgsInput(parts.filter((p) => p !== flag).join(' '));
    } else {
      setCustomArgsInput([...parts, flag].join(' '));
    }
  };

  const handleOpenFolder = async () => {
    if (!activeGame) return;
    try {
      await api.openPathInExplorer(activeGame.rootPath);
    } catch (e: any) {
      onToast?.(`Failed to open folder: ${e?.message || e}`, 'error');
    }
  };

  const activeCommandString = `gta_sa.exe ${buildArgs().join(' ')}`.trim();

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

            <div className="text-xs text-neutral-400 font-mono flex items-center gap-2">
              <span className="truncate max-w-md">{activeGame.rootPath}</span>
              <button
                onClick={handleOpenFolder}
                className="text-neutral-400 hover:text-white transition-colors"
                title="Open Game Folder in Explorer"
              >
                <FolderOpen className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          <div className="flex items-center gap-2 flex-shrink-0">
            <button
              onClick={() => onNavigate('installations')}
              className="px-3.5 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs font-semibold text-neutral-200 border border-neutral-700 transition-colors"
            >
              Switch Game
            </button>
            <button
              onClick={() => onNavigate('settings')}
              className="px-3.5 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs font-semibold text-neutral-200 border border-neutral-700 transition-colors"
            >
              Configure
            </button>
          </div>
        </div>
      ) : (
        <div className="bg-bg-card border border-amber-500/30 rounded-lg p-6 text-center space-y-3">
          <AlertCircle className="w-8 h-8 text-amber-400 mx-auto" />
          <h2 className="text-sm font-bold text-white">No Active Game Installation Selected</h2>
          <p className="text-xs text-neutral-400 max-w-md mx-auto">
            Point SAMM to your GTA: San Andreas directory to begin managing mods, auto-resolving conflicts, and launching the game.
          </p>
          <button
            onClick={() => onNavigate('installations')}
            className="px-5 py-2 rounded bg-accent hover:bg-accent-hover text-bg-main font-bold text-xs shadow-md transition-colors inline-flex items-center gap-2"
          >
            <Gamepad2 className="w-4 h-4" />
            <span>Select or Scan Game Directory</span>
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
          <div className="text-lg font-bold text-white truncate">
            {activeProfile?.name || 'Default'}
          </div>
          <div className="text-[11px] text-neutral-400 truncate">
            {activeProfile?.description || 'Standard profile loadout'}
          </div>
        </div>

        <div
          onClick={() => onNavigate('conflicts')}
          className="bg-bg-card hover:bg-bg-hover border border-border p-4 rounded-lg cursor-pointer transition-colors space-y-1"
        >
          <div className="flex items-center justify-between text-neutral-400">
            <span className="text-xs font-medium">Conflicts & Duplicates</span>
            <AlertTriangle className={`w-4 h-4 ${conflictsCount + duplicatesCount > 0 ? 'text-amber-400' : 'text-neutral-500'}`} />
          </div>
          <div className={`text-2xl font-black font-mono ${conflictsCount + duplicatesCount > 0 ? 'text-amber-400' : 'text-white'}`}>
            {conflictsCount + duplicatesCount}
          </div>
          <div className="text-[11px] text-neutral-400 truncate">
            {conflictsCount + duplicatesCount === 0
              ? 'Zero file collisions detected'
              : `${duplicatesCount} duplicate${duplicatesCount === 1 ? '' : 's'}, ${conflictsCount} collision${conflictsCount === 1 ? '' : 's'}`}
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

      {/* Comprehensive Game Launch & Engine Configuration Panel */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-5 shadow-sm">
        {/* Header & Primary Launch Actions */}
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 border-b border-border-subtle pb-4">
          <div>
            <div className="flex items-center gap-2">
              <Play className="w-4 h-4 text-accent fill-current" />
              <h2 className="text-sm font-bold text-white tracking-wide uppercase">
                Launch Options & Engine Parameters
              </h2>
            </div>
            <p className="text-xs text-neutral-400 mt-0.5">
              Configure display geometry, RenderWare video flags, and mod execution environments.
            </p>
          </div>

          <div className="flex items-center gap-2 flex-wrap">
            {isGameRunning ? (
              <>
                {onRestartGame && (
                  <button
                    onClick={() => onRestartGame(buildArgs())}
                    className="h-9 px-4 rounded bg-amber-500/20 hover:bg-amber-500/30 border border-amber-500/50 text-xs font-bold text-amber-300 flex items-center gap-1.5 transition-colors cursor-pointer"
                    title="Restart GTA: San Andreas with active flags"
                  >
                    <RotateCcw className="w-3.5 h-3.5" />
                    <span>Restart Game</span>
                  </button>
                )}
                {onKillGame && (
                  <button
                    onClick={onKillGame}
                    className="h-9 px-4 rounded bg-rose-950/90 hover:bg-rose-900 border border-rose-800 text-xs font-bold text-rose-300 flex items-center gap-1.5 transition-colors cursor-pointer"
                    title="Force terminate running game process"
                  >
                    <Square className="w-3.5 h-3.5 fill-current" />
                    <span>Stop Process</span>
                  </button>
                )}
              </>
            ) : (
              <>
                <button
                  onClick={() => handleLaunch(true)}
                  disabled={!activeGame || launching}
                  className="h-9 px-3.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs font-semibold text-neutral-300 flex items-center gap-1.5 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                  title="Launch with -safemode flag to bypass high-res asset hooks if crashed"
                >
                  <Shield className="w-3.5 h-3.5 text-neutral-400" />
                  <span>Safe Mode</span>
                </button>

                <button
                  onClick={() => handleLaunch(false)}
                  disabled={!activeGame || launching}
                  className={`h-9 px-6 rounded font-bold text-xs flex items-center gap-2 transition-all shadow-md ${
                    activeGame && !launching
                      ? 'bg-accent hover:bg-accent-hover text-bg-main active:scale-[0.98] cursor-pointer'
                      : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
                  }`}
                >
                  {launching ? (
                    <>
                      <Loader2 className="w-4 h-4 animate-spin text-accent" />
                      <span>Starting Process...</span>
                    </>
                  ) : (
                    <>
                      <Play className="w-4 h-4 fill-current" />
                      <span>Launch GTA: San Andreas</span>
                    </>
                  )}
                </button>
              </>
            )}
          </div>
        </div>

        {/* Section 1: Core Engine Feature Flags (Checkboxes) */}
        <div className="space-y-2">
          <div className="text-[11px] font-bold uppercase tracking-wider text-neutral-400">
            Engine & Execution Flags
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-3 text-xs">
            {/* Skip Intro */}
            <label className="flex items-center gap-2.5 p-2.5 rounded bg-bg-main border border-border hover:border-neutral-600 transition-colors cursor-pointer select-none">
              <input
                type="checkbox"
                checked={argNoIntro}
                onChange={(e) => setArgNoIntro(e.target.checked)}
                className="accent-amber-500 rounded w-4 h-4"
              />
              <div className="min-w-0">
                <div className="font-semibold text-white">Skip Intro Videos</div>
                <div className="text-[10px] text-neutral-500 font-mono">-novid</div>
              </div>
            </label>

            {/* Windowed Mode */}
            <label className="flex items-center gap-2.5 p-2.5 rounded bg-bg-main border border-border hover:border-neutral-600 transition-colors cursor-pointer select-none">
              <input
                type="checkbox"
                checked={argWindowed}
                onChange={(e) => setArgWindowed(e.target.checked)}
                className="accent-amber-500 rounded w-4 h-4"
              />
              <div className="min-w-0">
                <div className="font-semibold text-white">Windowed Mode</div>
                <div className="text-[10px] text-neutral-500 font-mono">-window</div>
              </div>
            </label>

            {/* Disable Audio */}
            <label className="flex items-center gap-2.5 p-2.5 rounded bg-bg-main border border-border hover:border-neutral-600 transition-colors cursor-pointer select-none">
              <input
                type="checkbox"
                checked={argNoSound}
                onChange={(e) => setArgNoSound(e.target.checked)}
                className="accent-amber-500 rounded w-4 h-4"
              />
              <div className="min-w-0">
                <div className="font-semibold text-white">Mute Game Audio</div>
                <div className="text-[10px] text-neutral-500 font-mono">-nosound</div>
              </div>
            </label>

            {/* Safe Mode */}
            <label className="flex items-center gap-2.5 p-2.5 rounded bg-bg-main border border-border hover:border-neutral-600 transition-colors cursor-pointer select-none">
              <input
                type="checkbox"
                checked={argSafeMode}
                onChange={(e) => setArgSafeMode(e.target.checked)}
                className="accent-amber-500 rounded w-4 h-4"
              />
              <div className="min-w-0">
                <div className="font-semibold text-white">Safe Mode Fallback</div>
                <div className="text-[10px] text-neutral-500 font-mono">-safemode</div>
              </div>
            </label>

            {/* Developer Debug */}
            <label className="flex items-center gap-2.5 p-2.5 rounded bg-bg-main border border-border hover:border-neutral-600 transition-colors cursor-pointer select-none">
              <input
                type="checkbox"
                checked={argDebug}
                onChange={(e) => setArgDebug(e.target.checked)}
                className="accent-amber-500 rounded w-4 h-4"
              />
              <div className="min-w-0">
                <div className="font-semibold text-white">Developer Debug</div>
                <div className="text-[10px] text-neutral-500 font-mono">-debug</div>
              </div>
            </label>
          </div>
        </div>

        {/* Section 2: Display Geometry & Monitor Target */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-1">
          {/* Resolution Presets */}
          <div className="space-y-1.5">
            <label className="block text-xs font-semibold text-neutral-300 flex items-center gap-1.5">
              <Monitor className="w-3.5 h-3.5 text-accent" />
              <span>Display Resolution Preset</span>
            </label>
            <div className="grid grid-cols-2 sm:grid-cols-4 gap-2">
              {[
                { id: 'native', label: 'Default Geometry' },
                { id: '1080p', label: '1080p FHD' },
                { id: '1440p', label: '1440p 2K QHD' },
                { id: '4k', label: '4K UHD' },
              ].map((res) => (
                <button
                  key={res.id}
                  type="button"
                  onClick={() => setArgResolution(res.id as any)}
                  className={`h-8 px-2.5 rounded text-xs font-medium border transition-colors cursor-pointer ${
                    argResolution === res.id
                      ? 'bg-accent/15 border-accent text-accent font-bold'
                      : 'bg-bg-main border-border text-neutral-300 hover:border-neutral-600'
                  }`}
                >
                  {res.label}
                </button>
              ))}
            </div>

            {argResolution === 'custom' && (
              <div className="flex items-center gap-2 pt-2">
                <input
                  type="number"
                  value={customWidth}
                  onChange={(e) => setCustomWidth(e.target.value)}
                  placeholder="Width (e.g. 1920)"
                  className="w-28 h-8 bg-bg-main border border-border rounded px-2.5 text-xs text-white font-mono"
                />
                <span className="text-neutral-500 font-mono text-xs">x</span>
                <input
                  type="number"
                  value={customHeight}
                  onChange={(e) => setCustomHeight(e.target.value)}
                  placeholder="Height (e.g. 1080)"
                  className="w-28 h-8 bg-bg-main border border-border rounded px-2.5 text-xs text-white font-mono"
                />
              </div>
            )}
          </div>

          {/* Monitor & Active Profile Selector */}
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {/* Monitor Adapter */}
            <div className="space-y-1.5">
              <label className="block text-xs font-semibold text-neutral-300">
                Target Display Adapter
              </label>
              <select
                value={argScreen}
                onChange={(e) => setArgScreen(e.target.value)}
                className="w-full h-8 bg-bg-main border border-border rounded px-2.5 text-xs text-white focus:outline-none focus:border-accent cursor-pointer"
              >
                <option value="0">Primary Display (-screen 0)</option>
                <option value="1">Secondary Display (-screen 1)</option>
                <option value="2">Tertiary Display (-screen 2)</option>
              </select>
            </div>

            {/* Inline Profile Switcher */}
            <div className="space-y-1.5">
              <label className="block text-xs font-semibold text-neutral-300 flex items-center justify-between">
                <span>Active Profile</span>
                <span className="text-[10px] text-accent font-mono cursor-pointer hover:underline" onClick={() => onNavigate('profiles')}>
                  Manage
                </span>
              </label>
              {profiles && profiles.length > 0 ? (
                <select
                  value={activeProfile?.id || ''}
                  onChange={(e) => onSelectProfile && onSelectProfile(e.target.value)}
                  className="w-full h-8 bg-bg-main border border-border rounded px-2.5 text-xs text-white focus:outline-none focus:border-accent cursor-pointer"
                >
                  {profiles.map((p) => (
                    <option key={p.id} value={p.id}>
                      {p.name} ({p.installedModIds?.length || 0} mods)
                    </option>
                  ))}
                </select>
              ) : (
                <div className="h-8 px-2.5 flex items-center bg-bg-main border border-border rounded text-xs text-neutral-400">
                  {activeProfile?.name || 'Default Profile'}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Section 3: Custom Command Arguments Bar */}
        <div className="space-y-2 pt-2 border-t border-border-subtle">
          <div className="flex items-center justify-between text-xs">
            <span className="font-semibold text-neutral-300 flex items-center gap-1.5">
              <SlidersHorizontal className="w-3.5 h-3.5 text-accent" />
              <span>Custom Launch Arguments</span>
            </span>
            <div className="flex items-center gap-1 text-[10px] font-mono text-neutral-400">
              <span>Quick Flag Badges:</span>
              <button
                type="button"
                onClick={() => toggleQuickFlag('-novid')}
                className="px-1.5 py-0.5 rounded bg-bg-main hover:bg-neutral-800 border border-border text-neutral-300 cursor-pointer"
              >
                + -novid
              </button>
              <button
                type="button"
                onClick={() => toggleQuickFlag('-window')}
                className="px-1.5 py-0.5 rounded bg-bg-main hover:bg-neutral-800 border border-border text-neutral-300 cursor-pointer"
              >
                + -window
              </button>
              <button
                type="button"
                onClick={() => toggleQuickFlag('-safemode')}
                className="px-1.5 py-0.5 rounded bg-bg-main hover:bg-neutral-800 border border-border text-neutral-300 cursor-pointer"
              >
                + -safemode
              </button>
              <button
                type="button"
                onClick={() => toggleQuickFlag('-nosound')}
                className="px-1.5 py-0.5 rounded bg-bg-main hover:bg-neutral-800 border border-border text-neutral-300 cursor-pointer"
              >
                + -nosound
              </button>
              <button
                type="button"
                onClick={() => toggleQuickFlag('-debug')}
                className="px-1.5 py-0.5 rounded bg-bg-main hover:bg-neutral-800 border border-border text-neutral-300 cursor-pointer"
              >
                + -debug
              </button>
            </div>
          </div>

          <div className="flex gap-2">
            <input
              type="text"
              value={customArgsInput}
              onChange={(e) => setCustomArgsInput(e.target.value)}
              placeholder="e.g. -novid -window -dx9 -screen 0"
              className="flex-1 h-8 bg-bg-main border border-border rounded px-3 text-xs text-white font-mono focus:outline-none focus:border-accent"
            />
            <button
              type="button"
              onClick={handleSaveDefaults}
              disabled={savingDefault}
              className="h-8 px-3 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-200 font-semibold flex items-center gap-1.5 transition-colors cursor-pointer flex-shrink-0"
              title="Save current parameters to settings so they load automatically"
            >
              <Save className="w-3.5 h-3.5 text-accent" />
              <span>{savingDefault ? 'Saving...' : 'Save as Default'}</span>
            </button>
          </div>
        </div>

        {/* Section 4: Live Command Execution Preview */}
        <div className="bg-bg-main border border-border rounded p-2.5 flex items-center justify-between gap-3 text-xs">
          <div className="flex items-center gap-2 min-w-0 font-mono text-[11px]">
            <Terminal className="w-4 h-4 text-emerald-400 flex-shrink-0" />
            <span className="text-neutral-500 flex-shrink-0 select-none">Exec:</span>
            <span className="text-emerald-300 font-semibold truncate select-all">
              {activeCommandString}
            </span>
          </div>

          <div className="text-[10px] text-neutral-500 font-medium flex-shrink-0 select-none hidden sm:block">
            Tokenized via direct process spawn (No cmd.exe injection)
          </div>
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
