import React, { useState, useEffect } from 'react';
import { ModRecord, ModSource } from '../types';
import { api } from '../api/client';
import {
  Search,
  ExternalLink,
  Download,
  CheckCircle2,
  AlertCircle,
  HelpCircle,
  Layers,
  Filter,
  RefreshCw,
  Info,
} from 'lucide-react';

interface DiscoverModsProps {
  sources: ModSource[];
  onSelectMod: (mod: ModRecord) => void;
  onDownloadMod: (mod: ModRecord) => void;
}

export const DiscoverMods: React.FC<DiscoverModsProps> = ({
  sources,
  onSelectMod,
  onDownloadMod,
}) => {
  const [query, setQuery] = useState('');
  const [selectedSource, setSelectedSource] = useState<string>('all');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [mods, setMods] = useState<ModRecord[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchMods = async () => {
    setLoading(true);
    setError(null);
    try {
      const results = await api.searchMods(
        query,
        selectedCategory === 'All' ? undefined : selectedCategory,
        selectedSource === 'all' ? undefined : selectedSource
      );
      setMods(results);
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Failed to search mods');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    const timer = setTimeout(() => {
      fetchMods();
    }, 300);
    return () => clearTimeout(timer);
  }, [query, selectedSource, selectedCategory]);

  const categories = [
    'All',
    'Plugins / ASI',
    'Scripts / CLEO',
    'Tools / Frameworks',
    'Graphics / Plugins',
    'Vehicles / Models',
  ];

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      {/* Header and Search controls */}
      <div className="space-y-4">
        <div>
          <h1 className="text-lg font-bold text-white">Discover GTA: San Andreas Mods</h1>
          <p className="text-xs text-neutral-400">
            Search verified community repositories (GTAinside, LibertyCity, MixMods, GTAGarage) and curated PC baselines.
          </p>
        </div>

        {/* Filter bar */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-3 bg-bg-card border border-border p-3 rounded-lg">
          {/* Search box */}
          <div className="md:col-span-2 relative">
            <Search className="w-4 h-4 text-neutral-400 absolute left-3 top-2.5" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search by mod name, author, or keyword (e.g. SilentPatch, SkyGfx, Widescreen)..."
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

          {/* Category selector */}
          <div className="relative">
            <select
              value={selectedCategory}
              onChange={(e) => setSelectedCategory(e.target.value)}
              className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent appearance-none cursor-pointer"
            >
              {categories.map((c) => (
                <option key={c} value={c}>
                  {c}
                </option>
              ))}
            </select>
            <Layers className="w-3.5 h-3.5 text-neutral-400 absolute right-3 top-2.5 pointer-events-none" />
          </div>
        </div>
      </div>

      {/* Results status */}
      <div className="flex items-center justify-between text-xs text-neutral-400">
        <div>
          Showing <span className="text-white font-semibold">{mods.length}</span> verified listings
        </div>
        {loading && (
          <div className="flex items-center gap-1.5 text-accent text-xs">
            <RefreshCw className="w-3.5 h-3.5 animate-spin" />
            <span>Refreshing catalog...</span>
          </div>
        )}
      </div>

      {error && (
        <div className="p-3 bg-rose-950/40 border border-rose-800 rounded text-xs text-rose-300 flex items-center gap-2">
          <AlertCircle className="w-4 h-4 flex-shrink-0" />
          <span>{error}</span>
        </div>
      )}

      {/* Mod Cards List */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {mods.map((mod) => (
          <div
            key={mod.id}
            className="bg-bg-card border border-border hover:border-neutral-600 rounded-lg p-5 flex flex-col justify-between transition-colors space-y-4"
          >
            <div className="space-y-2">
              <div className="flex items-start justify-between gap-2">
                <div>
                  <h3 className="text-sm font-bold text-white hover:text-accent cursor-pointer transition-colors" onClick={() => onSelectMod(mod)}>
                    {mod.title}
                  </h3>
                  <div className="text-[11px] text-neutral-400">
                    By <span className="text-neutral-300 font-semibold">{mod.author}</span> | Source: <span className="text-accent">{mod.sourceId}</span>
                  </div>
                </div>

                <span className="text-[10px] font-mono uppercase px-2 py-0.5 rounded bg-neutral-800 text-neutral-300 font-bold border border-neutral-700">
                  {mod.installationStrategy}
                </span>
              </div>

              <p className="text-xs text-neutral-300 line-clamp-2 leading-relaxed">
                {mod.description}
              </p>

              {/* Tags & compatibility */}
              <div className="flex flex-wrap items-center gap-2 pt-1 text-[11px]">
                {mod.confidence === 'confirmed' ? (
                  <span className="flex items-center gap-1 text-emerald-400 bg-emerald-950/50 border border-emerald-800/80 px-2 py-0.5 rounded font-semibold text-[10px]">
                    <CheckCircle2 className="w-3 h-3" /> Confirmed 1.0 Baseline
                  </span>
                ) : (
                  <span className="flex items-center gap-1 text-blue-400 bg-blue-950/50 border border-blue-800/80 px-2 py-0.5 rounded font-semibold text-[10px]">
                    <HelpCircle className="w-3 h-3" /> Reported Compatible
                  </span>
                )}

                {mod.fileSizeFormatted && (
                  <span className="text-neutral-400 font-mono text-[10px]">
                    {mod.fileSizeFormatted}
                  </span>
                )}
              </div>
            </div>

            {/* Actions */}
            <div className="pt-3 border-t border-border-subtle flex items-center justify-between">
              <div className="flex items-center gap-2">
                <button
                  onClick={() => onSelectMod(mod)}
                  className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs text-neutral-200 font-medium flex items-center gap-1.5 transition-colors"
                >
                  <Info className="w-3.5 h-3.5" />
                  <span>Inspect Details</span>
                </button>

                <a
                  href={mod.pageUrl}
                  target="_blank"
                  rel="noreferrer"
                  className="p-1.5 rounded text-neutral-400 hover:text-white hover:bg-neutral-800 transition-colors"
                  title="Open original mod page in browser"
                >
                  <ExternalLink className="w-4 h-4" />
                </a>
              </div>

              {mod.downloadUrl ? (
                <button
                  onClick={() => onDownloadMod(mod)}
                  className="px-4 py-1.5 rounded bg-accent hover:bg-accent-hover text-bg-main text-xs font-bold flex items-center gap-1.5 transition-colors shadow"
                >
                  <Download className="w-3.5 h-3.5" />
                  <span>Download</span>
                </button>
              ) : (
                <a
                  href={mod.pageUrl}
                  target="_blank"
                  rel="noreferrer"
                  className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-neutral-300 text-xs font-semibold flex items-center gap-1.5 transition-colors"
                >
                  <ExternalLink className="w-3.5 h-3.5" />
                  <span>Download on Website</span>
                </a>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
