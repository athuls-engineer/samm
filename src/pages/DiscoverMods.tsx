import React, { useState, useEffect } from 'react';
import { ModRecord, ModSource, InstalledMod, AutoInstallStatus } from '../types';
import { api } from '../api/client';
import {
  Search,
  ExternalLink,
  CheckCircle2,
  AlertCircle,
  HelpCircle,
  Layers,
  Filter,
  RefreshCw,
  Info,
  Zap,
  FolderDown,
  RotateCcw,
  Loader2,
  ShieldCheck,
  Sparkles,
} from 'lucide-react';

interface DiscoverModsProps {
  sources: ModSource[];
  installedMods: InstalledMod[];
  autoInstallStates: Record<string, AutoInstallStatus>;
  onSelectMod: (mod: ModRecord) => void;
  onOneClickInstall: (mod: ModRecord) => void;
  onImportArchive: () => void;
}

// Global module-level cache for instant tab transitions
const modResultsCache = new Map<string, ModRecord[]>();

export const DiscoverMods: React.FC<DiscoverModsProps> = ({
  sources,
  installedMods,
  autoInstallStates,
  onSelectMod,
  onOneClickInstall,
  onImportArchive,
}) => {
  const [query, setQuery] = useState('');
  const [selectedSource, setSelectedSource] = useState<string>('all');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [mods, setMods] = useState<ModRecord[]>(() => {
    return modResultsCache.get('All_all_') || [];
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchMods = async () => {
    const trimmedQuery = query.trim();
    const cacheKey = `${selectedCategory}_${selectedSource}_${trimmedQuery}`;

    if (modResultsCache.has(cacheKey)) {
      setMods(modResultsCache.get(cacheKey)!);
      return;
    }

    setLoading(true);
    setError(null);
    try {
      const results = await api.searchMods(
        query,
        selectedCategory === 'All' ? undefined : selectedCategory,
        selectedSource === 'all' ? undefined : selectedSource
      );
      modResultsCache.set(cacheKey, results);
      setMods(results);
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Failed to search mods');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    const isActivelyTyping = query.trim().length > 0;
    // Instant (0ms) when switching categories/sources or initial mount; small 120ms debounce when typing
    const delay = isActivelyTyping ? 120 : 0;
    const timer = setTimeout(() => {
      fetchMods();
    }, delay);
    return () => clearTimeout(timer);
  }, [query, selectedSource, selectedCategory]);

  const quickFilterTabs = [
    { id: 'All', label: 'All Mods (220+ Classics)' },
    { id: 'Plugins / ASI', label: '⚡ ASI & Engine Fixes' },
    { id: 'Total Conversions', label: '🌍 Total Conversions & Maps' },
    { id: 'Graphics / Plugins', label: '🎨 PS2 Graphics & Shaders' },
    { id: 'Graphics / Textures', label: '🖼️ HD World & Textures' },
    { id: 'Skins / Models', label: '👤 HD Skins & Characters' },
    { id: 'Weapons / Assets', label: '🔫 Weapons & Arsenal' },
    { id: 'Scripts / CLEO', label: '📜 CLEO Scripts' },
    { id: 'Tools / Frameworks', label: '🛠️ Loaders & Frameworks' },
    { id: 'Gameplay / Scripts', label: '🎮 Gameplay & Overhauls' },
    { id: 'Vehicles / Assets', label: '🚗 Vehicles & Supercars' },
    { id: 'Audio / Sounds', label: '🎵 Radio & Soundpacks' },
    { id: 'Tools', label: '🔧 Standalone Editors' },
  ];

  const getSourceBadgeColor = (sourceId: string) => {
    switch (sourceId.toLowerCase()) {
      case 'gtainside':
        return 'bg-amber-950/70 text-amber-300 border-amber-800/60';
      case 'mixmods':
        return 'bg-emerald-950/70 text-emerald-300 border-emerald-800/60';
      case 'libertycity':
        return 'bg-sky-950/70 text-sky-300 border-sky-800/60';
      case 'gtagarage':
        return 'bg-purple-950/70 text-purple-300 border-purple-800/60';
      default:
        return 'bg-neutral-800 text-neutral-300 border-neutral-700';
    }
  };

  const isModInstalled = (modId: string) => {
    return installedMods.some((m) => m.id === modId);
  };

  return (
    <div className="p-6 md:p-8 space-y-6 max-w-7xl mx-auto">
      {/* Header and Quick Stats */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border-subtle pb-5">
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-xl font-bold tracking-tight text-white">Discover GTA: San Andreas Mods</h1>
            <span className="text-[10px] font-mono px-2 py-0.5 rounded-full bg-emerald-950/80 text-emerald-400 border border-emerald-800 font-semibold flex items-center gap-1">
              <ShieldCheck className="w-3 h-3" /> 1.0 Baseline Ready
            </span>
          </div>
          <p className="text-xs text-neutral-400 mt-1">
            Browse the canonical GTA SA modding library, or import from the <strong>100,000+ community mods</strong> on GTAinside and LibertyCity. Click <strong className="text-amber-400">⚡ 1-Click Install</strong> to auto-deploy into your active game directory.
          </p>
        </div>

        <button
          onClick={onImportArchive}
          className="flex items-center gap-2 px-4 py-2 rounded bg-amber-500 hover:bg-amber-400 text-neutral-950 font-bold text-xs transition-colors self-start md:self-auto shadow-md"
        >
          <FolderDown className="w-4 h-4 text-neutral-950" />
          <span>Import Any Mod / URL</span>
        </button>
      </div>

      {/* Filter and Search Bar */}
      <div className="space-y-3">
        {/* Category Pills */}
        <div className="flex items-center gap-1.5 overflow-x-auto pb-1 text-xs no-scrollbar">
          {quickFilterTabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setSelectedCategory(tab.id)}
              className={`px-3 py-1.5 rounded-md font-medium whitespace-nowrap transition-colors text-xs ${
                selectedCategory === tab.id
                  ? 'bg-accent text-bg-main font-bold shadow-sm'
                  : 'bg-bg-card hover:bg-neutral-800 text-neutral-300 border border-border'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>

        {/* Inputs row */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-3 bg-bg-card border border-border p-2.5 rounded-lg">
          {/* Search box */}
          <div className="md:col-span-3 relative">
            <Search className="w-4 h-4 text-neutral-400 absolute left-3 top-2.5" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search by mod name, author, or feature (e.g. SilentPatch, SkyGfx, Widescreen, Limit Adjuster)..."
              className="w-full bg-bg-main border border-border rounded pl-9 pr-3 py-1.5 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-accent"
            />
          </div>

          {/* Source selector */}
          <div className="relative">
            <select
              value={selectedSource}
              onChange={(e) => setSelectedSource(e.target.value)}
              className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent appearance-none cursor-pointer"
            >
              <option value="all">All Mod Repositories</option>
              {sources.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.name}
                </option>
              ))}
            </select>
            <Filter className="w-3.5 h-3.5 text-neutral-400 absolute right-3 top-2.5 pointer-events-none" />
          </div>
        </div>
      </div>

      {/* Catalog Status */}
      <div className="flex items-center justify-between text-xs text-neutral-400">
        <div>
          Showing <span className="text-white font-semibold">{mods.length}</span> verified mods
          {selectedCategory !== 'All' && <span> in <span className="text-accent">{selectedCategory}</span></span>}
        </div>
        {loading && (
          <div className="flex items-center gap-1.5 text-accent text-xs">
            <RefreshCw className="w-3.5 h-3.5 animate-spin" />
            <span>Scanning repositories...</span>
          </div>
        )}
      </div>

      {error && (
        <div className="p-3 bg-rose-950/40 border border-rose-800 rounded text-xs text-rose-300 flex items-center gap-2">
          <AlertCircle className="w-4 h-4 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Mod Cards Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {mods.map((mod) => {
          const autoStatus = autoInstallStates[mod.id];
          const isInstalled = isModInstalled(mod.id);
          const isBusy = autoStatus && (autoStatus.step === 'downloading' || autoStatus.step === 'planning' || autoStatus.step === 'installing');

          return (
            <div
              key={mod.id}
              className={`bg-bg-card border rounded-lg p-4 flex flex-col justify-between transition-all duration-150 relative overflow-hidden ${
                isInstalled
                  ? 'border-emerald-800/80 bg-bg-card/90'
                  : 'border-border hover:border-neutral-500'
              }`}
            >
              {/* Card Header & Content */}
              <div className="space-y-2.5">
                {/* Badges strip */}
                <div className="flex items-center justify-between gap-2">
                  <span className={`text-[10px] font-mono font-bold uppercase px-2 py-0.5 rounded border ${getSourceBadgeColor(mod.sourceId)}`}>
                    {mod.sourceId}
                  </span>

                  <span className="text-[10px] font-mono font-semibold px-2 py-0.5 rounded bg-neutral-800 text-neutral-300 border border-neutral-700">
                    {mod.installationStrategy.toUpperCase()}
                  </span>
                </div>

                {/* Title and Author */}
                <div>
                  <h3
                    className="text-sm font-bold text-white hover:text-accent cursor-pointer transition-colors leading-snug"
                    onClick={() => onSelectMod(mod)}
                  >
                    {mod.title}
                  </h3>
                  <div className="text-[11px] text-neutral-400 mt-0.5">
                    By <span className="text-neutral-300 font-medium">{mod.author}</span>
                  </div>
                </div>

                {/* Description */}
                <p className="text-xs text-neutral-300 line-clamp-2 leading-relaxed">
                  {mod.description}
                </p>

                {/* Tags & Baseline Info */}
                <div className="flex flex-wrap items-center gap-1.5 pt-1 text-[10px]">
                  {mod.confidence === 'confirmed' ? (
                    <span className="flex items-center gap-1 text-emerald-400 bg-emerald-950/60 border border-emerald-800/80 px-1.5 py-0.5 rounded font-semibold">
                      <CheckCircle2 className="w-3 h-3" /> 1.0 US Verified
                    </span>
                  ) : (
                    <span className="flex items-center gap-1 text-blue-400 bg-blue-950/60 border border-blue-800/80 px-1.5 py-0.5 rounded font-semibold">
                      <HelpCircle className="w-3 h-3" /> Compatible
                    </span>
                  )}

                  {mod.fileSizeFormatted && (
                    <span className="text-neutral-400 font-mono bg-neutral-900 border border-neutral-800 px-1.5 py-0.5 rounded">
                      {mod.fileSizeFormatted}
                    </span>
                  )}

                  {isInstalled && (
                    <span className="text-emerald-300 font-semibold bg-emerald-900/60 border border-emerald-700 px-1.5 py-0.5 rounded flex items-center gap-1">
                      <CheckCircle2 className="w-3 h-3" /> Installed
                    </span>
                  )}
                </div>
              </div>

              {/* Action Area */}
              <div className="pt-3 mt-3 border-t border-border-subtle flex flex-col gap-2">
                {/* Active Automated Progress Strip */}
                {isBusy && (
                  <div className="bg-bg-main border border-accent/40 rounded p-2 space-y-1.5">
                    <div className="flex items-center justify-between text-[11px] text-accent font-semibold">
                      <span className="flex items-center gap-1.5">
                        <Loader2 className="w-3 h-3 animate-spin" />
                        {autoStatus.step === 'downloading' && (autoStatus.message || 'Downloading...')}
                        {autoStatus.step === 'planning' && 'Inspecting & Backing Up...'}
                        {autoStatus.step === 'installing' && 'Deploying into GTA SA...'}
                      </span>
                      {autoStatus.progress !== undefined && (
                        <span className="font-mono">{autoStatus.progress}%</span>
                      )}
                    </div>
                    {autoStatus.progress !== undefined && (
                      <div className="w-full bg-neutral-800 h-1.5 rounded-full overflow-hidden">
                        <div
                          className="bg-accent h-full transition-all duration-300 rounded-full"
                          style={{ width: `${autoStatus.progress}%` }}
                        />
                      </div>
                    )}
                  </div>
                )}

                {/* Action Buttons Row */}
                <div className="flex items-center justify-between gap-2">
                  <div className="flex items-center gap-1.5">
                    <button
                      onClick={() => onSelectMod(mod)}
                      className="px-2.5 py-1 rounded bg-neutral-800 hover:bg-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1 transition-colors"
                      title="Inspect Mod details, requirements & files"
                    >
                      <Info className="w-3.5 h-3.5" />
                      <span>Details</span>
                    </button>

                    <a
                      href={mod.pageUrl}
                      target="_blank"
                      rel="noreferrer"
                      className="p-1.5 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors"
                      title="Open source website"
                    >
                      <ExternalLink className="w-3.5 h-3.5" />
                    </a>
                  </div>

                  {/* Primary Trigger Button */}
                  {mod.downloadUrl ? (
                    <button
                      onClick={() => onOneClickInstall(mod)}
                      disabled={isBusy}
                      className={`px-3.5 py-1.5 rounded text-xs font-bold flex items-center gap-1.5 transition-all shadow ${
                        isBusy
                          ? 'bg-neutral-800 text-neutral-500 cursor-not-allowed'
                          : isInstalled
                          ? 'bg-neutral-800 hover:bg-neutral-700 text-amber-300 border border-neutral-700'
                          : 'bg-accent hover:bg-accent-hover text-bg-main active:scale-[0.98]'
                      }`}
                    >
                      {isBusy ? (
                        <>
                          <Loader2 className="w-3.5 h-3.5 animate-spin" />
                          <span>Installing...</span>
                        </>
                      ) : isInstalled ? (
                        <>
                          <RotateCcw className="w-3.5 h-3.5" />
                          <span>Reinstall</span>
                        </>
                      ) : (
                        <>
                          <Zap className="w-3.5 h-3.5 fill-current" />
                          <span>1-Click Install</span>
                        </>
                      )}
                    </button>
                  ) : (
                    <a
                      href={mod.pageUrl}
                      target="_blank"
                      rel="noreferrer"
                      className="px-2.5 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-neutral-300 text-xs font-semibold flex items-center gap-1 transition-colors"
                    >
                      <ExternalLink className="w-3.5 h-3.5" />
                      <span>On Website</span>
                    </a>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
