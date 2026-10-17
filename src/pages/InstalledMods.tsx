import React, { useState, useMemo, useRef, useEffect } from 'react';
import { InstalledMod, GameInstallation } from '../types';
import { api } from '../api/client';
import {
  Package,
  Search,
  CheckCircle2,
  Trash2,
  List,
  Power,
  AlertTriangle,
  RefreshCw,
  FolderOpen,
  CheckSquare,
  Square,
  ArrowUpDown,
  Copy,
  Check,
  HardDrive,
  Filter,
  X,
} from 'lucide-react';

interface InstalledModsProps {
  mods: InstalledMod[];
  activeGame: GameInstallation | null;
  onToggleState: (modId: string, enabled: boolean) => void;
  onUninstall: (modId: string) => void;
  onInspectFiles: (mod: InstalledMod) => void;
  onRescanFolder?: () => void;
  onToast?: (text: string, type?: 'success' | 'error') => void;
}

type SortOption = 'name_asc' | 'name_desc' | 'files_desc' | 'type' | 'enabled_first' | 'disabled_first';

export const InstalledMods: React.FC<InstalledModsProps> = ({
  mods,
  activeGame,
  onToggleState,
  onUninstall,
  onInspectFiles,
  onRescanFolder,
  onToast,
}) => {
  const [query, setQuery] = useState('');
  const [sourceFilter, setSourceFilter] = useState<'all' | 'preinstalled' | 'managed'>('all');
  const [strategyFilter, setStrategyFilter] = useState<string>('all');
  const [sortBy, setSortBy] = useState<SortOption>('name_asc');
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [confirmUninstallId, setConfirmUninstallId] = useState<string | null>(null);
  const [confirmBatchUninstall, setConfirmBatchUninstall] = useState(false);
  const [rescanning, setRescanning] = useState(false);
  const [copiedModList, setCopiedModList] = useState(false);

  const searchInputRef = useRef<HTMLInputElement>(null);

  // Global Ctrl+F listener
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f') {
        e.preventDefault();
        searchInputRef.current?.focus();
      } else if (e.key === 'Escape') {
        if (query) setQuery('');
        if (selectedIds.size > 0) setSelectedIds(new Set());
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [query, selectedIds]);

  // Counts and Metrics
  const preinstalledCount = mods.filter(
    (m) => m.sourceName.toLowerCase().includes('pre-installed') || m.sourceName.toLowerCase().includes('detected')
  ).length;
  const managedCount = mods.length - preinstalledCount;
  const enabledCount = mods.filter((m) => m.enabled).length;

  const totalSizeBytes = useMemo(() => {
    let sum = 0;
    for (let i = 0; i < mods.length; i++) {
      const files = mods[i].files;
      if (!files) continue;
      for (let j = 0; j < files.length; j++) {
        sum += files[j].fileSizeBytes || 0;
      }
    }
    return sum;
  }, [mods]);

  const formattedTotalSize = useMemo(() => {
    if (totalSizeBytes > 1024 * 1024 * 1024) {
      return `${(totalSizeBytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
    }
    return `${(totalSizeBytes / (1024 * 1024)).toFixed(1)} MB`;
  }, [totalSizeBytes]);

  // Filtered and Sorted Mods
  const filteredAndSorted = useMemo(() => {
    const q = query.trim().toLowerCase();
    const strat = strategyFilter.toLowerCase();

    const list = mods.filter((m) => {
      const isPreinstalled =
        m.sourceName.toLowerCase().includes('pre-installed') || m.sourceName.toLowerCase().includes('detected');
      if (sourceFilter === 'preinstalled' && !isPreinstalled) return false;
      if (sourceFilter === 'managed' && isPreinstalled) return false;

      const matchesStrategy = strategyFilter === 'all' || m.strategy.toLowerCase() === strat;
      if (!matchesStrategy) return false;

      if (!q) return true;

      // Quick metadata check
      if (
        m.name.toLowerCase().includes(q) ||
        m.author.toLowerCase().includes(q) ||
        m.id.toLowerCase().includes(q)
      ) {
        return true;
      }

      // Check files (bounded to first 100 to prevent freezing when a mod has thousands of files)
      if (m.files && m.files.length > 0) {
        const bound = Math.min(m.files.length, 100);
        for (let i = 0; i < bound; i++) {
          if (m.files[i].destinationRelPath.toLowerCase().includes(q)) {
            return true;
          }
        }
      }

      return false;
    });

    list.sort((a, b) => {
      switch (sortBy) {
        case 'name_asc':
          return a.name.localeCompare(b.name);
        case 'name_desc':
          return b.name.localeCompare(a.name);
        case 'files_desc':
          return (b.files?.length || 0) - (a.files?.length || 0);
        case 'type':
          return a.strategy.localeCompare(b.strategy);
        case 'enabled_first':
          return Number(b.enabled) - Number(a.enabled);
        case 'disabled_first':
          return Number(a.enabled) - Number(b.enabled);
        default:
          return 0;
      }
    });

    return list;
  }, [mods, query, sourceFilter, strategyFilter, sortBy]);

  // Batch actions
  const handleToggleSelectAll = () => {
    if (selectedIds.size === filteredAndSorted.length && filteredAndSorted.length > 0) {
      setSelectedIds(new Set());
    } else {
      setSelectedIds(new Set(filteredAndSorted.map((m) => m.id)));
    }
  };

  const handleToggleSelectOne = (id: string) => {
    const next = new Set(selectedIds);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    setSelectedIds(next);
  };

  const handleBatchToggle = async (enable: boolean) => {
    if (!activeGame || selectedIds.size === 0) return;
    try {
      const idsArray = Array.from(selectedIds);
      await api.batchToggleMods(activeGame.id, idsArray, enable);
      idsArray.forEach((id) => onToggleState(id, enable));
      onToast?.(
        `${enable ? 'Enabled' : 'Disabled'} ${idsArray.length} mod${idsArray.length === 1 ? '' : 's'}.`
      );
    } catch (e: any) {
      onToast?.(`Batch toggle failed: ${e?.message || e}`, 'error');
    }
  };

  const handleBatchUninstall = async () => {
    if (!activeGame || selectedIds.size === 0) return;
    try {
      const idsArray = Array.from(selectedIds);
      await api.batchUninstallMods(activeGame.id, idsArray);
      idsArray.forEach((id) => onUninstall(id));
      setSelectedIds(new Set());
      setConfirmBatchUninstall(false);
      onToast?.(`Uninstalled ${idsArray.length} mod${idsArray.length === 1 ? '' : 's'} successfully.`);
    } catch (e: any) {
      onToast?.(`Batch uninstall failed: ${e?.message || e}`, 'error');
    }
  };

  const handleOpenModFolder = async (mod: InstalledMod) => {
    if (!activeGame) return;
    const root = activeGame.rootPath.replace(/\\/g, '/');

    let targetPath = root;
    if (mod.id === 'github_rainbomizer') {
      targetPath = `${root}/rainbomizer`;
    } else if (mod.files && mod.files.length > 0) {
      const firstRel = mod.files[0].destinationRelPath.replace(/\\/g, '/');
      const parts = firstRel.split('/');
      if (parts.length > 1) {
        if (parts[0].toLowerCase() === 'modloader' && parts.length >= 2) {
          targetPath = `${root}/${parts[0]}/${parts[1]}`;
        } else {
          targetPath = `${root}/${parts[0]}`;
        }
      }
    }

    try {
      await api.openPathInExplorer(targetPath);
    } catch (e: any) {
      onToast?.(`Failed to open folder: ${e?.message || e}`, 'error');
    }
  };

  const handleExportModList = () => {
    const lines = [
      `# GTA: San Andreas Mod List (${activeGame ? activeGame.name : 'SAMM'})`,
      `Generated on ${new Date().toLocaleDateString()} - Total: ${mods.length} mods (${enabledCount} enabled)`,
      '',
    ];

    mods.forEach((m, idx) => {
      const status = m.enabled ? '[ENABLED]' : '[DISABLED]';
      lines.push(`${idx + 1}. ${m.name} - by ${m.author} (${m.strategy.toUpperCase()}) ${status}`);
    });

    const text = lines.join('\n');
    navigator.clipboard.writeText(text);
    setCopiedModList(true);
    onToast?.('Mod list copied to clipboard!');
    setTimeout(() => setCopiedModList(false), 2500);
  };

  const handleRescan = async () => {
    if (!onRescanFolder) return;
    setRescanning(true);
    try {
      await onRescanFolder();
    } finally {
      setRescanning(false);
    }
  };

  return (
    <div className="p-6 md:p-8 space-y-5 max-w-6xl mx-auto">
      {/* Top Header & Metrics Ribbon */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border-subtle pb-5">
        <div>
          <div className="flex items-center gap-2.5">
            <h1 className="text-xl font-bold tracking-tight text-white">Installed Mods</h1>
            <span className="text-xs font-mono font-bold px-2 py-0.5 rounded-full bg-neutral-800 text-neutral-300 border border-neutral-700">
              {mods.length} Total
            </span>
          </div>
          <p className="text-xs text-neutral-400 mt-1">
            Active mod loadout for {activeGame ? <strong className="text-white">{activeGame.name}</strong> : 'GTA San Andreas'}.
          </p>
        </div>

        <div className="flex items-center gap-2 self-start md:self-auto flex-wrap">
          <button
            onClick={handleExportModList}
            className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs font-semibold text-neutral-200 flex items-center gap-1.5 transition-colors"
            title="Copy formatted mod list to clipboard"
          >
            {copiedModList ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5 text-neutral-400" />}
            <span>{copiedModList ? 'Copied List' : 'Export Mod List'}</span>
          </button>

          {onRescanFolder && activeGame && (
            <button
              onClick={handleRescan}
              disabled={rescanning}
              className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs font-semibold text-white flex items-center gap-1.5 transition-colors"
              title="Rescan game directory for newly dropped ASI plugins, CLEO scripts, and ModLoader folders"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${rescanning ? 'animate-spin text-accent' : ''}`} />
              <span>{rescanning ? 'Scanning...' : 'Rescan Folder'}</span>
            </button>
          )}
        </div>
      </div>

      {/* Metrics Ribbon */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-2.5">
        <div className="bg-bg-card/70 border border-border/80 px-3 py-2 rounded-md">
          <div className="text-[10px] uppercase font-mono font-semibold tracking-wider text-neutral-400">Total Mods</div>
          <div className="text-base font-bold text-white font-mono mt-0.5">{mods.length}</div>
        </div>
        <div className="bg-bg-card/70 border border-border/80 px-3 py-2 rounded-md">
          <div className="text-[10px] uppercase font-mono font-semibold tracking-wider text-neutral-400">Active / Enabled</div>
          <div className="text-base font-bold text-emerald-400 font-mono mt-0.5">
            {enabledCount} <span className="text-[11px] text-neutral-500 font-sans font-normal">({mods.length - enabledCount} off)</span>
          </div>
        </div>
        <div className="bg-bg-card/70 border border-border/80 px-3 py-2 rounded-md">
          <div className="text-[10px] uppercase font-mono font-semibold tracking-wider text-neutral-400">Tracked Files</div>
          <div className="text-base font-bold text-neutral-200 font-mono mt-0.5">
            {mods.reduce((acc, m) => acc + m.files.length, 0)}
          </div>
        </div>
        <div className="bg-bg-card/70 border border-border/80 px-3 py-2 rounded-md">
          <div className="text-[10px] uppercase font-mono font-semibold tracking-wider text-neutral-400">Disk Footprint</div>
          <div className="text-base font-bold text-accent font-mono mt-0.5">{formattedTotalSize}</div>
        </div>
      </div>

      {/* Filter, Sort, and Search Bar */}
      <div className="space-y-3">
        <div className="flex flex-wrap items-center justify-between gap-3">
          {/* Source Filter Pills */}
          <div className="flex items-center gap-1.5 text-xs">
            <button
              onClick={() => setSourceFilter('all')}
              className={`px-3 py-1 rounded font-semibold transition-colors ${
                sourceFilter === 'all'
                  ? 'bg-accent text-bg-main shadow-sm'
                  : 'bg-bg-card hover:bg-neutral-800 text-neutral-300 border border-border'
              }`}
            >
              All ({mods.length})
            </button>
            <button
              onClick={() => setSourceFilter('preinstalled')}
              className={`px-3 py-1 rounded font-semibold transition-colors ${
                sourceFilter === 'preinstalled'
                  ? 'bg-accent text-bg-main shadow-sm'
                  : 'bg-bg-card hover:bg-neutral-800 text-neutral-300 border border-border'
              }`}
            >
              Pre-installed ({preinstalledCount})
            </button>
            <button
              onClick={() => setSourceFilter('managed')}
              className={`px-3 py-1 rounded font-semibold transition-colors ${
                sourceFilter === 'managed'
                  ? 'bg-accent text-bg-main shadow-sm'
                  : 'bg-bg-card hover:bg-neutral-800 text-neutral-300 border border-border'
              }`}
            >
              Managed ({managedCount})
            </button>
          </div>

          {/* Sort By Dropdown */}
          <div className="flex items-center gap-2 text-xs">
            <ArrowUpDown className="w-3.5 h-3.5 text-neutral-400" />
            <span className="text-neutral-400 text-[11px]">Sort:</span>
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortOption)}
              className="bg-bg-card border border-border rounded px-2.5 py-1 text-xs text-neutral-200 focus:outline-none focus:border-accent cursor-pointer"
            >
              <option value="name_asc">Name (A to Z)</option>
              <option value="name_desc">Name (Z to A)</option>
              <option value="files_desc">Tracked Files (High to Low)</option>
              <option value="type">Mod Type / Strategy</option>
              <option value="enabled_first">Enabled First</option>
              <option value="disabled_first">Disabled First</option>
            </select>
          </div>
        </div>

        {/* Search Input & Mod Type Filter */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-2.5 bg-bg-card border border-border p-2 rounded-lg">
          <div className="md:col-span-3 relative">
            <Search className="w-4 h-4 text-neutral-400 absolute left-3 top-2.5" />
            <input
              ref={searchInputRef}
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search installed mods by name, author, or file name (Ctrl+F)..."
              className="w-full bg-bg-main border border-border rounded pl-9 pr-8 py-1.5 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-accent font-sans"
            />
            {query && (
              <button
                onClick={() => setQuery('')}
                className="absolute right-2.5 top-2.5 text-neutral-400 hover:text-white"
                title="Clear search"
              >
                <X className="w-3.5 h-3.5" />
              </button>
            )}
          </div>

          <select
            value={strategyFilter}
            onChange={(e) => setStrategyFilter(e.target.value)}
            className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent cursor-pointer"
          >
            <option value="all">All Categories</option>
            <option value="modloader">ModLoader Folders</option>
            <option value="asi">ASI Plugins</option>
            <option value="cleo">CLEO Scripts</option>
            <option value="assets">Model & Texture Assets</option>
          </select>
        </div>
      </div>

      {/* Batch Action Bar (Appears when 1 or more mods are selected) */}
      {selectedIds.size > 0 && (
        <div className="bg-neutral-800/90 border border-neutral-700 rounded-lg p-3 flex flex-wrap items-center justify-between gap-3 shadow-md animate-in fade-in">
          <div className="flex items-center gap-3">
            <span className="text-xs font-bold text-accent font-mono">
              {selectedIds.size} {selectedIds.size === 1 ? 'mod' : 'mods'} selected
            </span>
            <button
              onClick={() => setSelectedIds(new Set())}
              className="text-xs text-neutral-400 hover:text-white hover:underline"
            >
              Clear selection
            </button>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => handleBatchToggle(true)}
              className="px-3 py-1 rounded bg-emerald-950/90 hover:bg-emerald-900 border border-emerald-800 text-xs font-semibold text-emerald-300 flex items-center gap-1.5 transition-colors"
            >
              <Power className="w-3.5 h-3.5" />
              <span>Enable All</span>
            </button>

            <button
              onClick={() => handleBatchToggle(false)}
              className="px-3 py-1 rounded bg-neutral-700 hover:bg-neutral-600 border border-neutral-600 text-xs font-semibold text-neutral-200 flex items-center gap-1.5 transition-colors"
            >
              <Power className="w-3.5 h-3.5" />
              <span>Disable All</span>
            </button>

            {confirmBatchUninstall ? (
              <div className="flex items-center gap-1.5 bg-rose-950 border border-rose-800 px-2 py-0.5 rounded">
                <span className="text-[11px] text-rose-300 font-bold">Remove {selectedIds.size} mods?</span>
                <button
                  onClick={handleBatchUninstall}
                  className="px-2 py-0.5 bg-rose-600 hover:bg-rose-500 text-white rounded text-[10px] font-bold"
                >
                  Yes, Remove
                </button>
                <button
                  onClick={() => setConfirmBatchUninstall(false)}
                  className="px-2 py-0.5 bg-neutral-700 hover:bg-neutral-600 text-neutral-200 rounded text-[10px]"
                >
                  Cancel
                </button>
              </div>
            ) : (
              <button
                onClick={() => setConfirmBatchUninstall(true)}
                className="px-3 py-1 rounded bg-rose-950/80 hover:bg-rose-900 border border-rose-800 text-xs font-semibold text-rose-300 flex items-center gap-1.5 transition-colors"
              >
                <Trash2 className="w-3.5 h-3.5" />
                <span>Uninstall Selected</span>
              </button>
            )}
          </div>
        </div>
      )}

      {/* Master Select All Checkbox */}
      {filteredAndSorted.length > 0 && (
        <div className="flex items-center justify-between text-xs text-neutral-400 px-1">
          <label className="flex items-center gap-2 cursor-pointer select-none">
            <input
              type="checkbox"
              checked={selectedIds.size === filteredAndSorted.length && filteredAndSorted.length > 0}
              onChange={handleToggleSelectAll}
              className="accent-amber-500 rounded"
            />
            <span className="font-semibold text-neutral-300">
              Select All Shown ({filteredAndSorted.length})
            </span>
          </label>
        </div>
      )}

      {/* Mods List */}
      {filteredAndSorted.length === 0 ? (
        <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-2">
          <Package className="w-8 h-8 text-neutral-500 mx-auto" />
          <div className="text-sm font-semibold text-neutral-300">No Installed Mods Found</div>
          <div>
            {query.trim() || sourceFilter !== 'all' || strategyFilter !== 'all'
              ? 'No mods match the selected filter criteria.'
              : 'Add mods via 1-Click Install, import an archive, or click "Rescan Folder" to detect existing mods.'}
          </div>
        </div>
      ) : (
        <div className="space-y-2">
          {filteredAndSorted.map((mod) => {
            const isPreinstalled =
              mod.sourceName.toLowerCase().includes('pre-installed') ||
              mod.sourceName.toLowerCase().includes('detected');
            const isSelected = selectedIds.has(mod.id);

            return (
              <div
                key={mod.id}
                className={`group bg-bg-card border rounded-lg px-3.5 py-2.5 flex items-center justify-between gap-3 transition-all ${
                  isSelected
                    ? 'border-accent bg-neutral-800/40 shadow-sm'
                    : mod.enabled
                    ? 'border-border hover:border-neutral-700 hover:bg-neutral-800/20'
                    : 'border-neutral-800/80 bg-neutral-900/40 opacity-75 hover:opacity-90'
                }`}
              >
                {/* Left: Checkbox + Mod Info */}
                <div className="flex items-center gap-3 min-w-0 flex-1">
                  {/* Select Checkbox */}
                  <input
                    type="checkbox"
                    checked={isSelected}
                    onChange={() => handleToggleSelectOne(mod.id)}
                    className="accent-amber-500 rounded w-4 h-4 cursor-pointer flex-shrink-0"
                    title={isSelected ? 'Deselect mod' : 'Select mod for batch actions'}
                  />

                  <div className="min-w-0 flex-1">
                    <div className="flex items-center gap-2 flex-wrap">
                      <span className={`font-semibold text-sm truncate ${mod.enabled ? 'text-white' : 'text-neutral-400'}`}>
                        {mod.name}
                      </span>

                      {/* Mod Strategy Badge */}
                      <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-neutral-800 text-neutral-400 border border-neutral-700/80 font-semibold uppercase">
                        {mod.strategy}
                      </span>

                      {/* Origin Badge */}
                      {isPreinstalled && (
                        <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-sky-950/60 text-sky-300 border border-sky-800/60 font-medium">
                          Pre-installed
                        </span>
                      )}

                      {/* Conflict Indicator */}
                      {mod.hasConflicts && (
                        <span className="flex items-center gap-1 text-[10px] text-amber-400 font-semibold bg-amber-950/60 px-1.5 py-0.5 rounded border border-amber-800/80">
                          <AlertTriangle className="w-3 h-3" /> Collision
                        </span>
                      )}
                    </div>

                    <div className="text-xs text-neutral-400 flex items-center gap-2 mt-0.5">
                      <span>by <strong className="text-neutral-300 font-normal">{mod.author}</strong></span>
                      <span className="text-neutral-600">&bull;</span>
                      <span>{mod.files.length} {mod.files.length === 1 ? 'file' : 'files'}</span>
                    </div>
                  </div>
                </div>

                {/* Right: Actions + Modern Toggle Switch */}
                <div className="flex items-center gap-1.5 flex-shrink-0">
                  {/* Files Modal Trigger */}
                  <button
                    onClick={() => onInspectFiles(mod)}
                    className="px-2 py-1 rounded bg-neutral-800/80 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1 transition-colors"
                    title="View files belonging to this mod"
                  >
                    <List className="w-3.5 h-3.5 text-neutral-400" />
                    <span className="font-mono text-[11px]">{mod.files.length}</span>
                  </button>

                  {/* Open Mod Directory */}
                  {activeGame && (
                    <button
                      onClick={() => handleOpenModFolder(mod)}
                      className="p-1.5 rounded bg-neutral-800/80 hover:bg-neutral-700 border border-neutral-700 text-neutral-300 hover:text-white transition-colors"
                      title="Reveal mod in Windows Explorer"
                    >
                      <FolderOpen className="w-3.5 h-3.5 text-accent" />
                    </button>
                  )}

                  {/* Single Delete / Uninstall */}
                  {confirmUninstallId === mod.id ? (
                    <div className="flex items-center gap-1 bg-rose-950/90 border border-rose-800 px-2 py-0.5 rounded shadow">
                      <span className="text-[11px] text-rose-300 font-semibold">Delete?</span>
                      <button
                        onClick={() => {
                          onUninstall(mod.id);
                          setConfirmUninstallId(null);
                        }}
                        className="px-2 py-0.5 bg-rose-600 hover:bg-rose-500 text-white rounded text-[10px] font-bold cursor-pointer"
                      >
                        Yes
                      </button>
                      <button
                        onClick={() => setConfirmUninstallId(null)}
                        className="px-1.5 py-0.5 bg-neutral-700 hover:bg-neutral-600 text-neutral-200 rounded text-[10px] cursor-pointer"
                      >
                        Cancel
                      </button>
                    </div>
                  ) : (
                    <button
                      onClick={() => setConfirmUninstallId(mod.id)}
                      className="p-1.5 rounded text-neutral-400 hover:text-rose-400 hover:bg-neutral-800 transition-colors"
                      title="Uninstall mod"
                    >
                      <Trash2 className="w-3.5 h-3.5" />
                    </button>
                  )}

                  <div className="h-4 w-px bg-neutral-700/60 mx-1" />

                  {/* Sleek Modern Toggle Switch */}
                  <button
                    type="button"
                    role="switch"
                    aria-checked={mod.enabled}
                    onClick={() => onToggleState(mod.id, !mod.enabled)}
                    title={mod.enabled ? 'Mod is ON (Click to turn OFF)' : 'Mod is OFF (Click to turn ON)'}
                    className={`relative inline-flex h-5 w-9 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none ${
                      mod.enabled ? 'bg-emerald-500 hover:bg-emerald-400' : 'bg-neutral-700 hover:bg-neutral-600'
                    }`}
                  >
                    <span
                      className={`pointer-events-none inline-block h-4 w-4 transform rounded-full bg-white shadow-md ring-0 transition duration-200 ease-in-out ${
                        mod.enabled ? 'translate-x-4' : 'translate-x-0'
                      }`}
                    />
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
};
