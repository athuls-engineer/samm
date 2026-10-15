import React, { useState } from 'react';
import { InstalledMod } from '../types';
import {
  Package,
  Search,
  CheckCircle2,
  Trash2,
  List,
  ExternalLink,
  Power,
  AlertTriangle,
  History,
} from 'lucide-react';

interface InstalledModsProps {
  mods: InstalledMod[];
  onToggleState: (modId: string, enabled: boolean) => void;
  onUninstall: (modId: string) => void;
  onInspectFiles: (mod: InstalledMod) => void;
}

export const InstalledMods: React.FC<InstalledModsProps> = ({
  mods,
  onToggleState,
  onUninstall,
  onInspectFiles,
}) => {
  const [query, setQuery] = useState('');
  const [strategyFilter, setStrategyFilter] = useState<string>('all');
  const [confirmUninstallId, setConfirmUninstallId] = useState<string | null>(null);

  const filtered = mods.filter((m) => {
    const matchesQuery = m.name.toLowerCase().includes(query.toLowerCase()) || m.author.toLowerCase().includes(query.toLowerCase());
    const matchesStrategy = strategyFilter === 'all' || m.strategy.toLowerCase() === strategyFilter.toLowerCase();
    return matchesQuery && matchesStrategy;
  });

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h1 className="text-lg font-bold text-white">Installed GTA: San Andreas Mods</h1>
          <p className="text-xs text-neutral-400">
            Active mod packages installed in your selected game directory. Safe toggling and transparent uninstall with automatic backup restoration.
          </p>
        </div>

        {/* Filter bar */}
        <div className="flex items-center gap-3">
          <div className="relative">
            <Search className="w-4 h-4 text-neutral-400 absolute left-3 top-2" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search installed mods..."
              className="bg-bg-card border border-border rounded pl-9 pr-3 py-1.5 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-accent w-48"
            />
          </div>

          <select
            value={strategyFilter}
            onChange={(e) => setStrategyFilter(e.target.value)}
            className="bg-bg-card border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent cursor-pointer"
          >
            <option value="all">All Strategies</option>
            <option value="cleo">CLEO Scripts</option>
            <option value="asi">ASI Plugins</option>
            <option value="modloader">ModLoader Packages</option>
            <option value="assets">Assets / Models</option>
          </select>
        </div>
      </div>

      {/* Mods List */}
      {filtered.length === 0 ? (
        <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-2">
          <Package className="w-8 h-8 text-neutral-500 mx-auto" />
          <div className="text-sm font-semibold text-neutral-300">No Installed Mods Found</div>
          <div>Import an archive or discover verified community mods to begin.</div>
        </div>
      ) : (
        <div className="space-y-3">
          {filtered.map((mod) => (
            <div
              key={mod.id}
              className={`bg-bg-card border rounded-lg p-4 flex flex-col md:flex-row items-start md:items-center justify-between gap-4 transition-colors ${
                mod.enabled ? 'border-border' : 'border-neutral-800 opacity-70'
              }`}
            >
              <div className="flex items-center gap-3">
                <button
                  onClick={() => onToggleState(mod.id, !mod.enabled)}
                  title={mod.enabled ? 'Disable Mod' : 'Enable Mod'}
                  className={`w-9 h-9 rounded flex items-center justify-center transition-colors ${
                    mod.enabled
                      ? 'bg-emerald-950/80 text-emerald-400 border border-emerald-800 hover:bg-emerald-900'
                      : 'bg-neutral-800 text-neutral-500 hover:text-neutral-300 border border-neutral-700'
                  }`}
                >
                  <Power className="w-4 h-4" />
                </button>

                <div>
                  <div className="flex items-center gap-2">
                    <span className="font-bold text-xs text-white">{mod.name}</span>
                    <span className="text-[10px] font-mono px-1.5 py-0.2 rounded bg-neutral-800 text-neutral-400 uppercase font-semibold">
                      {mod.strategy}
                    </span>
                    {mod.hasConflicts && (
                      <span className="flex items-center gap-1 text-[10px] text-amber-400 font-semibold bg-amber-950/60 px-1.5 py-0.2 rounded border border-amber-800">
                        <AlertTriangle className="w-3 h-3" /> Collision
                      </span>
                    )}
                  </div>
                  <div className="text-[11px] text-neutral-400 flex items-center gap-3 mt-0.5">
                    <span>By {mod.author}</span>
                    <span>•</span>
                    <span>{mod.files.length} files tracked</span>
                    <span>•</span>
                    <span>Installed {new Date(mod.installedAt).toLocaleDateString()}</span>
                  </div>
                </div>
              </div>

              {/* Action Buttons */}
              <div className="flex items-center gap-2 self-end md:self-auto">
                <button
                  onClick={() => onInspectFiles(mod)}
                  className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1.5 transition-colors"
                >
                  <List className="w-3.5 h-3.5" />
                  <span>Manifest</span>
                </button>

                {confirmUninstallId === mod.id ? (
                  <div className="flex items-center gap-1.5 bg-rose-950/80 border border-rose-800 px-2 py-1 rounded">
                    <span className="text-[11px] text-rose-300 font-semibold">Restore backups & uninstall?</span>
                    <button
                      onClick={() => {
                        onUninstall(mod.id);
                        setConfirmUninstallId(null);
                      }}
                      className="px-2 py-0.5 bg-rose-600 hover:bg-rose-500 text-white rounded text-[10px] font-bold"
                    >
                      Yes, Uninstall
                    </button>
                    <button
                      onClick={() => setConfirmUninstallId(null)}
                      className="px-2 py-0.5 bg-neutral-700 hover:bg-neutral-600 text-neutral-200 rounded text-[10px]"
                    >
                      Cancel
                    </button>
                  </div>
                ) : (
                  <button
                    onClick={() => setConfirmUninstallId(mod.id)}
                    className="p-1.5 rounded text-neutral-400 hover:text-rose-400 hover:bg-neutral-800 transition-colors"
                    title="Uninstall Mod & Restore Original Files"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
