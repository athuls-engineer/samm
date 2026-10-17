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
  Star,
  Flame,
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
  const [selectedCategory, setSelectedCategory] = useState<string>('Essential & Popular');
  const [filterDirectOnly, setFilterDirectOnly] = useState(false);
  const [mods, setMods] = useState<ModRecord[]>(() => {
    return modResultsCache.get('Essential & Popular_all_') || modResultsCache.get('All_all_') || [];
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
    { id: 'Essential & Popular', label: 'Essential & Popular' },
    { id: 'All', label: 'All Mods' },
    { id: 'Plugins / ASI', label: 'ASI & Engine Fixes' },
    { id: 'Total Conversions', label: 'Total Conversions & Maps' },
    { id: 'Graphics / Plugins', label: 'Graphics, Shaders & Lighting' },
    { id: 'Graphics / Textures', label: 'HD Textures & World' },
    { id: 'Skins / Models', label: 'Characters & Pedestrians' },
    { id: 'Weapons / Assets', label: 'Weapons & Arsenal' },
    { id: 'Scripts / CLEO', label: 'CLEO Scripts' },
    { id: 'Tools / Frameworks', label: 'Loaders & Frameworks' },
    { id: 'Gameplay / Scripts', label: 'Gameplay & Physics' },
    { id: 'Vehicles / Assets', label: 'Vehicles & Handling' },
    { id: 'Audio / Sounds', label: 'Audio & Sounds' },
    { id: 'Tools', label: 'Modding Tools' },
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

  const directInstallCount = mods.filter((m) => Boolean(m.downloadUrl)).length;
  const displayedMods = filterDirectOnly
    ? mods.filter((m) => Boolean(m.downloadUrl))
    : mods;

  return (
    <div className="p-6 md:p-8 space-y-5 max-w-7xl mx-auto">
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
            Browse verified GTA: San Andreas mods with transactional installation, automatic backup, and conflict prevention.
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
        {/* Category Pills & Quick 1-Click Toggle */}
        <div className="flex items-center justify-between gap-3 flex-nowrap">
          <div className="flex items-center gap-1.5 overflow-x-auto no-scrollbar flex-1 py-0.5">
            {quickFilterTabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setSelectedCategory(tab.id)}
                className={`h-9 px-3.5 rounded-md font-semibold whitespace-nowrap transition-colors text-xs flex items-center justify-center cursor-pointer flex-shrink-0 ${
                  selectedCategory === tab.id
                    ? 'bg-accent text-bg-main font-bold shadow-sm'
                    : 'bg-bg-card hover:bg-neutral-800 text-neutral-300 border border-border'
                }`}
              >
                {tab.label}
              </button>
            ))}
          </div>

          <div className="h-5 w-px bg-border flex-shrink-0" />

          <button
            onClick={() => setFilterDirectOnly(!filterDirectOnly)}
            className={`h-9 px-3.5 rounded-md font-bold whitespace-nowrap transition-all text-xs flex items-center justify-center gap-1.5 border flex-shrink-0 cursor-pointer ${
              filterDirectOnly
                ? 'bg-accent text-neutral-950 border-amber-400 shadow-md ring-1 ring-amber-400'
                : 'bg-bg-card hover:bg-neutral-800 text-accent border-border hover:border-accent/60'
            }`}
            title="Show only mods with direct 1-click automated installation"
          >
            <Zap className="w-3.5 h-3.5 fill-current" />
            <span>1-Click Ready Only ({directInstallCount})</span>
          </button>
        </div>

        {/* Inputs row */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-3 bg-bg-card border border-border p-2 rounded-lg">
          {/* Search box */}
          <div className="md:col-span-3 relative flex items-center">
            <Search className="w-4 h-4 text-neutral-400 absolute left-3 pointer-events-none" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search by mod name, author, or feature (e.g. SilentPatch, SkyGfx, Widescreen, Limit Adjuster)..."
              className="w-full h-9 bg-bg-main border border-border rounded pl-9 pr-3 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-accent"
            />
          </div>

          {/* Source selector */}
          <div className="relative flex items-center">
            <select
              value={selectedSource}
              onChange={(e) => setSelectedSource(e.target.value)}
              className="w-full h-9 bg-bg-main border border-border rounded px-3 pr-8 text-xs text-white focus:outline-none focus:border-accent appearance-none cursor-pointer"
            >
              <option value="all">All Mod Repositories</option>
              {sources.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.name}
                </option>
              ))}
            </select>
            <Filter className="w-3.5 h-3.5 text-neutral-400 absolute right-3 pointer-events-none" />
          </div>
        </div>
      </div>

      {/* Catalog Status */}
      <div className="flex items-center justify-between text-xs text-neutral-400">
        <div>
          Showing <span className="text-white font-semibold">{displayedMods.length}</span> verified mods
          {selectedCategory !== 'All' && <span> in <span className="text-accent">{selectedCategory}</span></span>}
          {filterDirectOnly && <span className="text-amber-400 font-semibold"> (1-Click Only)</span>}
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
        {displayedMods.map((mod) => {
          const autoStatus = autoInstallStates[mod.id];
          const isInstalled = isModInstalled(mod.id);
          const isBusy = autoStatus && (autoStatus.step === 'downloading' || autoStatus.step === 'planning' || autoStatus.step === 'installing');

          return (
            <div
              key={mod.id}
              className={`bg-bg-card border rounded-lg p-4 flex flex-col justify-between transition-all duration-150 relative overflow-hidden min-h-[350px] ${
                isInstalled
                  ? 'border-emerald-800/80 bg-bg-card/90'
                  : 'border-border hover:border-neutral-500'
              }`}
            >
              {/* Card Header & Content */}
              <div className="space-y-2.5 flex-1 flex flex-col">
                {/* Badges strip */}
                <div className="h-6 flex items-center justify-between gap-2">
                  <div className="flex items-center gap-1.5 flex-wrap">
                    <span className={`text-[10px] font-mono font-bold uppercase px-2 py-0.5 rounded border leading-none flex items-center ${getSourceBadgeColor(mod.sourceId)}`}>
                      {mod.sourceId}
                    </span>
                    {mod.downloadUrl ? (
                      <span className="text-[10px] font-mono font-bold px-1.5 py-0.5 rounded bg-amber-950/80 text-amber-300 border border-amber-800/80 flex items-center gap-1 leading-none">
                        <Zap className="w-2.5 h-2.5 fill-current" /> 1-Click
                      </span>
                    ) : (
                      <span className="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded bg-neutral-900 text-neutral-400 border border-neutral-800 flex items-center gap-1 leading-none">
                        <ExternalLink className="w-2.5 h-2.5" /> Web Page
                      </span>
                    )}
                  </div>

                  <span className="text-[10px] font-mono font-semibold px-2 py-0.5 rounded bg-neutral-800 text-neutral-300 border border-neutral-700 leading-none flex items-center">
                    {mod.installationStrategy.toUpperCase()}
                  </span>
                </div>

                {/* Title and Author */}
                <div className="space-y-0.5">
                  <h3
                    className="text-sm font-bold text-white hover:text-accent cursor-pointer transition-colors leading-snug line-clamp-2 h-10 flex items-start"
                    onClick={() => onSelectMod(mod)}
                    title={mod.title}
                  >
                    {mod.title}
                  </h3>
                  <div className="text-[11px] text-neutral-400 truncate h-4">
                    By <span className="text-neutral-300 font-medium">{mod.author}</span>
                  </div>
                </div>

                {/* Description */}
                <p className="text-xs text-neutral-300 line-clamp-2 leading-relaxed h-9 overflow-hidden">
                  {mod.description}
                </p>

                {/* Tags & Baseline Info */}
                <div className="h-7 flex items-center gap-1.5 overflow-hidden text-[10px] mt-auto">
                  {mod.tags.some((t) => t.toLowerCase() === 'essential') && (
                    <span className="flex items-center gap-1 text-amber-300 bg-amber-950/80 border border-amber-700/80 px-1.5 py-0.5 rounded font-bold shadow-sm whitespace-nowrap">
                      <Star className="w-3 h-3 fill-current text-amber-400" /> Essential
                    </span>
                  )}
                  {mod.tags.some((t) => t.toLowerCase() === 'popular') && (
                    <span className="flex items-center gap-1 text-rose-300 bg-rose-950/80 border border-rose-800/80 px-1.5 py-0.5 rounded font-bold shadow-sm whitespace-nowrap">
                      <Flame className="w-3 h-3 fill-current text-rose-400" /> Popular
                    </span>
                  )}

                  {mod.confidence === 'confirmed' ? (
                    <span className="flex items-center gap-1 text-emerald-400 bg-emerald-950/60 border border-emerald-800/80 px-1.5 py-0.5 rounded font-semibold whitespace-nowrap">
                      <CheckCircle2 className="w-3 h-3" /> 1.0 US
                    </span>
                  ) : (
                    <span className="flex items-center gap-1 text-blue-400 bg-blue-950/60 border border-blue-800/80 px-1.5 py-0.5 rounded font-semibold whitespace-nowrap">
                      <HelpCircle className="w-3 h-3" /> Compatible
                    </span>
                  )}

                  {mod.fileSizeFormatted && (
                    <span className="text-neutral-400 font-mono bg-neutral-900 border border-neutral-800 px-1.5 py-0.5 rounded whitespace-nowrap">
                      {mod.fileSizeFormatted}
                    </span>
                  )}

                  {isInstalled && (
                    <span className="text-emerald-300 font-semibold bg-emerald-900/60 border border-emerald-700 px-1.5 py-0.5 rounded flex items-center gap-1 whitespace-nowrap">
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
                <div className="h-8 flex items-center justify-between gap-2">
                  <div className="flex items-center gap-1.5">
                    <button
                      onClick={() => onSelectMod(mod)}
                      className="h-8 px-2.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1 transition-colors cursor-pointer"
                      title="Inspect Mod details, requirements & files"
                    >
                      <Info className="w-3.5 h-3.5" />
                      <span>Details</span>
                    </button>

                    <a
                      href={mod.pageUrl}
                      target="_blank"
                      rel="noreferrer"
                      onClick={(e) => {
                        e.preventDefault();
                        api.openUrl(mod.pageUrl);
                      }}
                      className="h-8 w-8 flex items-center justify-center rounded text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors cursor-pointer"
                      title="Open source website in browser"
                    >
                      <ExternalLink className="w-3.5 h-3.5" />
                    </a>
                  </div>

                  {/* Primary Trigger Button */}
                  {mod.downloadUrl ? (
                    <button
                      onClick={() => onOneClickInstall(mod)}
                      disabled={isBusy}
                      className={`h-8 px-3.5 rounded text-xs font-bold flex items-center gap-1.5 transition-all shadow cursor-pointer ${
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
                      onClick={(e) => {
                        e.preventDefault();
                        api.openUrl(mod.pageUrl);
                      }}
                      className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-neutral-300 text-xs font-semibold flex items-center gap-1.5 transition-colors cursor-pointer"
                      title="Open mod page in browser"
                    >
                      <ExternalLink className="w-3.5 h-3.5 text-neutral-400" />
                      <span>Open Mod Page</span>
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
