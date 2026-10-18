import React, { useState } from 'react';
import { FileConflict, GameInstallation, DuplicateModGroup } from '../types';
import { api } from '../api/client';
import {
  AlertTriangle,
  CheckCircle2,
  ShieldAlert,
  Layers,
  Check,
  Zap,
  RefreshCw,
  ShieldCheck,
  Info,
  Trash2,
  Copy,
  FolderOpen,
  AlertCircle,
  FileCode,
} from 'lucide-react';

interface ConflictsProps {
  conflicts: FileConflict[];
  duplicates: DuplicateModGroup[];
  activeGame: GameInstallation | null;
  onRefresh: () => void;
  onToast: (text: string, type?: 'success' | 'error') => void;
}

export const Conflicts: React.FC<ConflictsProps> = ({
  conflicts,
  duplicates,
  activeGame,
  onRefresh,
  onToast,
}) => {
  const [activeSubTab, setActiveSubTab] = useState<'duplicates' | 'collisions'>(
    duplicates.length > 0 ? 'duplicates' : 'collisions'
  );
  const [resolvingId, setResolvingId] = useState<string | null>(null);
  const [autoResolving, setAutoResolving] = useState(false);
  const [scanningConflicts, setScanningConflicts] = useState(false);

  const [removingDupId, setRemovingDupId] = useState<string | null>(null);
  const [removingAllDups, setRemovingAllDups] = useState(false);
  const [scanningDuplicates, setScanningDuplicates] = useState(false);
  const [selectedKeepMap, setSelectedKeepMap] = useState<Record<string, string>>({});

  // Duplicates Handlers
  const handleScanDuplicates = async () => {
    if (!activeGame) {
      onToast('No active game selected.', 'error');
      return;
    }

    setScanningDuplicates(true);
    try {
      const dups = await api.getDuplicateMods(activeGame.id);
      onRefresh();
      if (dups.length === 0) {
        onToast('Duplicate check complete: Zero duplicate mods detected.');
      } else {
        const totalRedundant = dups.reduce((acc, g) => acc + g.duplicatesToRemove.length, 0);
        onToast(`Duplicate check complete: Found ${dups.length} duplicate mod groups (${totalRedundant} redundant items).`);
      }
    } catch (e: any) {
      onToast(typeof e === 'string' ? e : e?.message || 'Failed to check for duplicate mods', 'error');
    } finally {
      setScanningDuplicates(false);
    }
  };

  const handleRemoveSingleDuplicate = async (targetId: string, name: string) => {
    if (!activeGame) {
      onToast('No active game selected.', 'error');
      return;
    }

    setRemovingDupId(targetId);
    try {
      const res = await api.removeDuplicateMod(activeGame.id, targetId);
      onToast(`Uninstalled duplicate: ${res}`);
      onRefresh();
    } catch (e: any) {
      onToast(typeof e === 'string' ? e : e?.message || `Failed to remove duplicate '${name}'`, 'error');
    } finally {
      setRemovingDupId(null);
    }
  };

  const handleRemoveAllDuplicates = async () => {
    if (!activeGame) {
      onToast('No active game selected.', 'error');
      return;
    }

    setRemovingAllDups(true);
    try {
      const res = await api.removeAllDuplicates(activeGame.id);
      onToast(`Successfully removed ${res.removedCount} duplicate installations and freed up disk space.`);
      onRefresh();
    } catch (e: any) {
      onToast(typeof e === 'string' ? e : e?.message || 'Failed to remove duplicates', 'error');
    } finally {
      setRemovingAllDups(false);
    }
  };

  // Conflicts Handlers
  const handleScanConflicts = async () => {
    if (!activeGame) {
      onToast('No active game selected.', 'error');
      return;
    }

    setScanningConflicts(true);
    try {
      const detected = await api.checkForConflicts(activeGame.id);
      onRefresh();
      if (detected.length === 0) {
        onToast('Conflict check complete: Zero file conflicts detected.');
      } else {
        onToast(`Conflict check complete: Detected ${detected.length} file collision${detected.length === 1 ? '' : 's'}.`);
      }
    } catch (e: any) {
      onToast(typeof e === 'string' ? e : e?.message || 'Failed to check for conflicts', 'error');
    } finally {
      setScanningConflicts(false);
    }
  };

  const handleResolveSingle = async (
    conflict: FileConflict,
    keepModId: string,
    yieldModId: string,
    chosenName: string
  ) => {
    if (!activeGame) {
      onToast('No active game selected.', 'error');
      return;
    }

    setResolvingId(conflict.id);
    try {
      await api.resolveConflict(
        activeGame.id,
        conflict.destinationRelPath,
        keepModId,
        yieldModId
      );
      onToast(`Resolved '${conflict.destinationRelPath}': Using version from ${chosenName}`);
      onRefresh();
    } catch (e: any) {
      onToast(typeof e === 'string' ? e : e?.message || 'Failed to resolve conflict', 'error');
    } finally {
      setResolvingId(null);
    }
  };

  const handleAutoResolveAll = async () => {
    if (!activeGame) {
      onToast('No active game selected.', 'error');
      return;
    }

    setAutoResolving(true);
    try {
      const res = await api.autoResolveConflicts(activeGame.id);
      const count = typeof res === 'number' ? res : res.resolvedCount;
      const actions = typeof res === 'object' && res.actionsTaken && res.actionsTaken.length > 0
        ? ` - ${res.actionsTaken[0]}`
        : '';
      onToast(`Successfully resolved ${count} conflict${count === 1 ? '' : 's'}! Compatibility guaranteed.${actions}`);
      onRefresh();
    } catch (e: any) {
      onToast(typeof e === 'string' ? e : e?.message || 'Failed to auto-resolve conflicts', 'error');
    } finally {
      setAutoResolving(false);
    }
  };

  const isSharedDependency = (path: string) => {
    const lower = path.toLowerCase();
    return (
      lower.includes('vorbisfile') ||
      lower.includes('vorbishooked') ||
      lower.includes('limitadjuster') ||
      lower.includes('cleo.asi') ||
      lower.includes('d3d9.dll')
    );
  };

  const totalRedundantCount = duplicates.reduce((acc, g) => acc + g.duplicatesToRemove.length, 0);

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      {/* Top Header & Sub-Tabs */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border-subtle pb-5">
        <div>
          <div className="flex items-center gap-3">
            <h1 className="text-xl font-bold tracking-tight text-white">Conflicts & Duplicates</h1>
            {duplicates.length > 0 || conflicts.length > 0 ? (
              <span className="text-[11px] font-mono px-2 py-0.5 rounded-full bg-rose-950/80 text-rose-300 border border-rose-800 font-bold">
                {duplicates.length + conflicts.length} Issues Found
              </span>
            ) : (
              <span className="text-[11px] font-mono px-2 py-0.5 rounded-full bg-emerald-950/80 text-emerald-400 border border-emerald-800 font-bold flex items-center gap-1">
                <CheckCircle2 className="w-3.5 h-3.5" /> Clean Installation
              </span>
            )}
          </div>
          <p className="text-xs text-neutral-400 mt-1">
            Detect and remove duplicate mod installations, and resolve file collisions between installed mods.
          </p>
        </div>

        {/* Sub-Tab Switcher */}
        <div className="flex items-center gap-1.5 bg-neutral-900 border border-neutral-800 p-1 rounded-lg self-start md:self-auto">
          <button
            onClick={() => setActiveSubTab('duplicates')}
            className={`flex items-center gap-2 px-3.5 py-1.5 rounded-md text-xs font-semibold transition-all ${
              activeSubTab === 'duplicates'
                ? 'bg-neutral-800 text-white shadow-sm border border-neutral-700'
                : 'text-neutral-400 hover:text-neutral-200'
            }`}
          >
            <Copy className="w-3.5 h-3.5 text-accent" />
            <span>Duplicate Mods</span>
            {duplicates.length > 0 ? (
              <span className="text-[10px] font-mono font-bold px-1.5 py-0.2 rounded-full bg-rose-950 text-rose-300 border border-rose-800">
                {duplicates.length}
              </span>
            ) : (
              <span className="text-[10px] font-mono font-bold px-1.5 py-0.2 rounded-full bg-emerald-950 text-emerald-400 border border-emerald-800">
                0
              </span>
            )}
          </button>

          <button
            onClick={() => setActiveSubTab('collisions')}
            className={`flex items-center gap-2 px-3.5 py-1.5 rounded-md text-xs font-semibold transition-all ${
              activeSubTab === 'collisions'
                ? 'bg-neutral-800 text-white shadow-sm border border-neutral-700'
                : 'text-neutral-400 hover:text-neutral-200'
            }`}
          >
            <ShieldAlert className="w-3.5 h-3.5 text-amber-400" />
            <span>File Collisions</span>
            {conflicts.length > 0 ? (
              <span className="text-[10px] font-mono font-bold px-1.5 py-0.2 rounded-full bg-amber-950 text-amber-300 border border-amber-800">
                {conflicts.length}
              </span>
            ) : (
              <span className="text-[10px] font-mono font-bold px-1.5 py-0.2 rounded-full bg-emerald-950 text-emerald-400 border border-emerald-800">
                0
              </span>
            )}
          </button>
        </div>
      </div>

      {/* SUB-TAB 1: DUPLICATE MODS */}
      {activeSubTab === 'duplicates' && (
        <div className="space-y-6">
          {/* Action Bar */}
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 bg-neutral-900/60 border border-neutral-800 p-3.5 rounded-lg">
            <div className="text-xs text-neutral-300 flex items-center gap-2">
              <Info className="w-4 h-4 text-neutral-400 flex-shrink-0" />
              <span>
                Duplicate mods across ModLoader and root directory cause texture failures, menu invisibility, and crashes.
              </span>
            </div>

            <div className="flex items-center gap-2.5 flex-wrap self-end sm:self-auto">
              <button
                onClick={handleScanDuplicates}
                disabled={scanningDuplicates || removingAllDups}
                className="flex items-center gap-2 px-3.5 py-1.5 rounded-md bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 hover:border-neutral-500 font-semibold text-xs transition-all shadow-sm disabled:opacity-50"
                title="Scan GTA San Andreas directory for duplicate mod files and folders"
              >
                <RefreshCw className={`w-3.5 h-3.5 ${scanningDuplicates ? 'animate-spin text-accent' : 'text-neutral-400'}`} />
                <span>{scanningDuplicates ? 'Scanning Duplicates...' : 'Check for Duplicates'}</span>
              </button>

              {totalRedundantCount > 0 && (
                <button
                  onClick={handleRemoveAllDuplicates}
                  disabled={removingAllDups || scanningDuplicates}
                  className="flex items-center gap-2 px-4 py-1.5 rounded-md bg-rose-600 hover:bg-rose-500 text-white font-bold text-xs transition-all shadow-md disabled:opacity-50"
                  title="Uninstall all redundant duplicate copies and keep recommended ModLoader versions"
                >
                  {removingAllDups ? (
                    <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                  ) : (
                    <Trash2 className="w-3.5 h-3.5" />
                  )}
                  <span>Remove All Duplicates ({totalRedundantCount})</span>
                </button>
              )}
            </div>
          </div>

          {/* Duplicates List or Zero State */}
          {duplicates.length === 0 ? (
            <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-4 shadow-sm">
              <div className="w-12 h-12 rounded-full bg-emerald-950/60 border border-emerald-800 flex items-center justify-center mx-auto text-emerald-400">
                <ShieldCheck className="w-6 h-6" />
              </div>
              <div className="text-sm font-bold text-white">Zero Duplicate Mods Detected</div>
              <p className="max-w-md mx-auto text-neutral-400 text-xs leading-relaxed">
                All installed mods across your ModLoader directory, scripts folder, and root directory are unique. No overlapping redundant copies detected.
              </p>
              <div className="pt-2">
                <button
                  onClick={handleScanDuplicates}
                  disabled={scanningDuplicates}
                  className="inline-flex items-center gap-2 px-4 py-2 rounded-md bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 hover:border-neutral-500 font-semibold text-xs transition-all shadow-sm disabled:opacity-50"
                >
                  <RefreshCw className={`w-3.5 h-3.5 ${scanningDuplicates ? 'animate-spin text-accent' : 'text-neutral-400'}`} />
                  <span>{scanningDuplicates ? 'Scanning Directory...' : 'Scan for Duplicates Now'}</span>
                </button>
              </div>
            </div>
          ) : (
            <div className="space-y-4">
              {duplicates.map((group) => {
                const activeKeepId = selectedKeepMap[group.groupId] || group.recommendedKeepId || group.items[0]?.id;
                const recommendedItem = group.items.find((i) => i.id === activeKeepId) || group.items[0];
                const redundantItems = group.items.filter((i) => i.id !== activeKeepId);

                return (
                  <div
                    key={group.groupId}
                    className="bg-bg-card border border-border hover:border-neutral-700 rounded-lg p-5 space-y-4 shadow-sm transition-all"
                  >
                    {/* Group Header */}
                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-border-subtle pb-3">
                      <div className="flex items-center gap-2.5 flex-wrap">
                        <AlertTriangle className="w-4 h-4 text-amber-400 flex-shrink-0" />
                        <h2 className="text-sm font-bold text-white tracking-wide">{group.modTitle}</h2>
                        <span className="text-[10px] font-mono uppercase bg-amber-950/80 text-amber-300 border border-amber-800 px-2 py-0.5 rounded font-bold">
                          {group.items.length} Copies Found
                        </span>
                      </div>

                      <div className="flex items-center gap-3">
                        <span className="text-[11px] font-mono text-neutral-400">
                          Family: <code className="text-neutral-300">{group.groupId}</code>
                        </span>

                        {redundantItems.length > 0 && (
                          <button
                            onClick={async () => {
                              for (const item of redundantItems) {
                                await handleRemoveSingleDuplicate(item.id, item.name);
                              }
                            }}
                            disabled={removingAllDups || Boolean(removingDupId)}
                            className="flex items-center gap-1.5 px-2.5 py-1 rounded bg-rose-950/80 hover:bg-rose-900 border border-rose-800 text-rose-200 text-xs font-semibold transition-colors disabled:opacity-50"
                            title="Uninstall all redundant duplicates in this group"
                          >
                            <Trash2 className="w-3 h-3 text-rose-400" />
                            <span>Remove {redundantItems.length} Duplicate{redundantItems.length === 1 ? '' : 's'}</span>
                          </button>
                        )}
                      </div>
                    </div>

                    {/* Conflict Explanation */}
                    <div className="text-xs bg-amber-950/30 border border-amber-900/50 p-3 rounded-md text-amber-200/90 leading-relaxed flex items-start gap-2.5">
                      <AlertCircle className="w-4 h-4 text-amber-400 flex-shrink-0 mt-0.5" />
                      <span>{group.reason}</span>
                    </div>

                    {/* Versions Comparison Grid */}
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                      {/* Version 1: Recommended / Selected Keep */}
                      {recommendedItem && (
                        <div className="bg-emerald-950/20 border border-emerald-800/80 rounded-md p-4 space-y-3 flex flex-col justify-between">
                          <div className="space-y-3">
                            <div className="flex items-center justify-between gap-2">
                              <span className="text-[10px] uppercase font-bold text-emerald-400 bg-emerald-950 px-2 py-0.5 rounded border border-emerald-800 font-mono flex items-center gap-1">
                                <CheckCircle2 className="w-3 h-3" /> Recommended Keep
                              </span>
                              <span className="text-[10px] font-mono text-emerald-300 font-semibold uppercase">
                                {recommendedItem.locationType === 'modloader' ? 'ModLoader Isolated' : recommendedItem.locationType}
                              </span>
                            </div>

                            <div className="space-y-1">
                              <div className="text-xs font-bold text-white truncate" title={recommendedItem.name}>
                                {recommendedItem.name}
                              </div>
                              <div className="text-[11px] font-mono text-neutral-400 truncate" title={recommendedItem.primaryPath}>
                                Path: {recommendedItem.primaryPath}
                              </div>
                            </div>

                            <div className="flex items-center justify-between pt-1 border-t border-emerald-900/40 text-[11px] text-neutral-400 font-mono">
                              <span>{recommendedItem.fileCount} file{recommendedItem.fileCount === 1 ? '' : 's'}</span>
                              <span className="text-emerald-400 font-semibold">{recommendedItem.sizeFormatted}</span>
                            </div>
                          </div>

                          <div className="pt-2 flex items-center justify-between text-[11px] border-t border-emerald-900/30">
                            <span className="text-emerald-400/90 font-medium text-[10px] flex items-center gap-1">
                              <CheckCircle2 className="w-3 h-3 text-emerald-400" /> Preserved Version
                            </span>
                            <button
                              onClick={() => handleRemoveSingleDuplicate(recommendedItem.id, recommendedItem.name)}
                              disabled={removingDupId === recommendedItem.id || removingAllDups}
                              className="text-neutral-500 hover:text-rose-400 text-[10px] underline transition-colors disabled:opacity-50"
                              title="Uninstall this copy as well"
                            >
                              Uninstall
                            </button>
                          </div>
                        </div>
                      )}

                      {/* Version 2+: Redundant Duplicates to Remove */}
                      {redundantItems.map((item) => {
                        const isRemoving = removingDupId === item.id || removingAllDups;

                        return (
                          <div
                            key={item.id}
                            className="bg-rose-950/20 border border-rose-900/80 rounded-md p-4 space-y-3 flex flex-col justify-between"
                          >
                            <div className="space-y-3">
                              <div className="flex items-center justify-between gap-2">
                                <span className="text-[10px] uppercase font-bold text-rose-400 bg-rose-950 px-2 py-0.5 rounded border border-rose-800 font-mono flex items-center gap-1">
                                  <AlertTriangle className="w-3 h-3" /> Redundant Duplicate
                                </span>
                                <span className="text-[10px] font-mono text-rose-300 font-semibold uppercase">
                                  {item.locationType === 'root_asi' ? 'Root ASI Plugin' : item.locationType === 'loose_folder' ? 'Loose Mod Folder' : item.locationType}
                                </span>
                              </div>

                              <div className="space-y-1">
                                <div className="text-xs font-bold text-white truncate" title={item.name}>
                                  {item.name}
                                </div>
                                <div className="text-[11px] font-mono text-neutral-400 truncate" title={item.primaryPath}>
                                  Path: {item.primaryPath}
                                </div>
                              </div>

                              <div className="flex items-center justify-between pt-1 border-t border-rose-900/40 text-[11px] text-neutral-400 font-mono">
                                <span>{item.fileCount} file{item.fileCount === 1 ? '' : 's'}</span>
                                <span className="text-rose-400 font-semibold">{item.sizeFormatted}</span>
                              </div>
                            </div>

                            {/* Actions: Keep This Copy Instead OR Uninstall Duplicate */}
                            <div className="pt-2 flex items-center gap-2">
                              <button
                                onClick={() => setSelectedKeepMap((prev) => ({ ...prev, [group.groupId]: item.id }))}
                                className="flex-1 flex items-center justify-center gap-1 px-2 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white text-[11px] font-semibold border border-neutral-700 transition-colors shadow-sm"
                                title="Keep this version and mark other copies as redundant"
                              >
                                <CheckCircle2 className="w-3 h-3 text-emerald-400" />
                                <span>Keep Instead</span>
                              </button>

                              <button
                                onClick={() => handleRemoveSingleDuplicate(item.id, item.name)}
                                disabled={isRemoving}
                                className="flex-1 flex items-center justify-center gap-1 px-2 py-1.5 rounded bg-rose-600 hover:bg-rose-500 text-white text-[11px] font-bold transition-colors shadow-sm disabled:opacity-50"
                              >
                                {isRemoving ? (
                                  <RefreshCw className="w-3 h-3 animate-spin" />
                                ) : (
                                  <Trash2 className="w-3 h-3" />
                                )}
                                <span>{isRemoving ? 'Removing...' : 'Uninstall'}</span>
                              </button>
                            </div>
                          </div>
                        );
                      })}
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      )}

      {/* SUB-TAB 2: FILE COLLISIONS */}
      {activeSubTab === 'collisions' && (
        <div className="space-y-6">
          {/* Header Action Bar */}
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 bg-neutral-900/60 border border-neutral-800 p-3.5 rounded-lg">
            <div className="text-xs text-neutral-300 flex items-center gap-2">
              <Info className="w-4 h-4 text-neutral-400 flex-shrink-0" />
              <span>
                Resolve overlapping target files between installed mods. Choose which mod takes precedence.
              </span>
            </div>

            <div className="flex items-center gap-2.5 flex-wrap self-end sm:self-auto">
              <button
                onClick={handleScanConflicts}
                disabled={scanningConflicts || autoResolving}
                className="flex items-center gap-2 px-3.5 py-1.5 rounded-md bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 hover:border-neutral-500 font-semibold text-xs transition-all shadow-sm disabled:opacity-50"
                title="Scan game directory, ModLoader, and mod manifests for live file collisions"
              >
                <RefreshCw className={`w-3.5 h-3.5 ${scanningConflicts ? 'animate-spin text-accent' : 'text-neutral-400'}`} />
                <span>{scanningConflicts ? 'Scanning Files...' : 'Check for Conflicts'}</span>
              </button>

              {conflicts.length > 0 && (
                <button
                  onClick={handleAutoResolveAll}
                  disabled={autoResolving || scanningConflicts}
                  className="flex items-center gap-2 px-4 py-1.5 rounded-md bg-amber-500 hover:bg-amber-400 text-neutral-950 font-bold text-xs transition-all shadow-md disabled:opacity-50"
                >
                  {autoResolving ? (
                    <RefreshCw className="w-3.5 h-3.5 animate-spin text-neutral-950" />
                  ) : (
                    <Zap className="w-3.5 h-3.5 fill-current text-neutral-950" />
                  )}
                  <span>Auto-Resolve All ({conflicts.length})</span>
                </button>
              )}
            </div>
          </div>

          {conflicts.length === 0 ? (
            <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-4 shadow-sm">
              <div className="w-12 h-12 rounded-full bg-emerald-950/60 border border-emerald-800 flex items-center justify-center mx-auto text-emerald-400">
                <ShieldCheck className="w-6 h-6" />
              </div>
              <div className="text-sm font-bold text-white">Zero File Conflicts Detected</div>
              <p className="max-w-md mx-auto text-neutral-400 text-xs leading-relaxed">
                All installed mods have unique target paths or are cleanly isolated inside ModLoader folders. Your game directory is in a consistent state.
              </p>
              <div className="pt-2">
                <button
                  onClick={handleScanConflicts}
                  disabled={scanningConflicts}
                  className="inline-flex items-center gap-2 px-4 py-2 rounded-md bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 hover:border-neutral-500 font-semibold text-xs transition-all shadow-sm disabled:opacity-50"
                >
                  <RefreshCw className={`w-3.5 h-3.5 ${scanningConflicts ? 'animate-spin text-accent' : 'text-neutral-400'}`} />
                  <span>{scanningConflicts ? 'Running Collision Scan...' : 'Check for Conflicts Now'}</span>
                </button>
              </div>
            </div>
          ) : (
            <div className="space-y-4">
              {conflicts.map((conflict) => {
                const isBusy = resolvingId === conflict.id || autoResolving;
                const shared = isSharedDependency(conflict.destinationRelPath);

                return (
                  <div
                    key={conflict.id}
                    className="bg-bg-card border border-border hover:border-neutral-600 rounded-lg p-5 space-y-4 transition-all shadow-sm"
                  >
                    {/* Top status & filename */}
                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-border-subtle pb-3">
                      <div className="flex items-center gap-2.5 flex-wrap">
                        <ShieldAlert className="w-4 h-4 text-amber-400 flex-shrink-0" />
                        <span className="font-mono text-xs font-bold text-white bg-neutral-900 border border-neutral-700 px-2.5 py-1 rounded">
                          {conflict.destinationRelPath}
                        </span>
                        {shared && (
                          <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-sky-950/80 text-sky-300 border border-sky-800/80 font-semibold flex items-center gap-1">
                            <Layers className="w-3 h-3 text-sky-400" /> Shared Prerequisite
                          </span>
                        )}
                      </div>

                      <span className="text-[10px] uppercase font-bold text-amber-400 bg-amber-950/80 px-2 py-0.5 rounded border border-amber-800/80 self-start sm:self-auto font-mono">
                        Collision Detected
                      </span>
                    </div>

                    {/* Conflict Description / Explanation */}
                    <div className="text-xs text-neutral-300 space-y-2">
                      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 bg-neutral-900/70 border border-neutral-800 p-3 rounded-md">
                        <div className="space-y-1">
                          <span className="text-[10px] font-mono uppercase text-neutral-500 font-semibold">Active Mod (First Installed)</span>
                          <div className="font-semibold text-white text-xs">{conflict.activeModName}</div>
                        </div>
                        <div className="space-y-1">
                          <span className="text-[10px] font-mono uppercase text-neutral-500 font-semibold">Conflicting Mod (Duplicate Claim)</span>
                          <div className="font-semibold text-amber-300 text-xs">{conflict.conflictingModName}</div>
                        </div>
                      </div>

                      {shared ? (
                        <p className="text-[11px] text-neutral-400 leading-relaxed">
                          <Info className="w-3.5 h-3.5 text-neutral-400 inline-block -mt-0.5 mr-1" />
                          <strong>Notice:</strong> This is a standard shared library bundled across multiple mods. Selecting either version maintains full mod compatibility without duplicating files.
                        </p>
                      ) : (
                        <p className="text-[11px] text-neutral-400 leading-relaxed">
                          Both mods want to place files at the exact same location. Choose which mod's version should be used in GTA: San Andreas.
                        </p>
                      )}
                    </div>

                    {/* Resolution Action Buttons */}
                    <div className="pt-2 flex flex-wrap items-center gap-2.5">
                      <span className="text-[11px] font-medium text-neutral-400 mr-1">Choose Resolution:</span>

                      <button
                        onClick={() =>
                          handleResolveSingle(
                            conflict,
                            conflict.activeModId,
                            conflict.conflictingModId,
                            conflict.activeModName
                          )
                        }
                        disabled={isBusy}
                        className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 hover:border-neutral-500 text-xs font-semibold transition-colors disabled:opacity-50"
                      >
                        <Check className="w-3.5 h-3.5 text-emerald-400" />
                        <span>Keep "{conflict.activeModName}"</span>
                      </button>

                      <button
                        onClick={() =>
                          handleResolveSingle(
                            conflict,
                            conflict.conflictingModId,
                            conflict.activeModId,
                            conflict.conflictingModName
                          )
                        }
                        disabled={isBusy}
                        className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 hover:border-neutral-500 text-xs font-semibold transition-colors disabled:opacity-50"
                      >
                        <Check className="w-3.5 h-3.5 text-amber-400" />
                        <span>Use "{conflict.conflictingModName}"</span>
                      </button>

                      {shared && (
                        <button
                          onClick={() =>
                            handleResolveSingle(
                              conflict,
                              conflict.activeModId,
                              conflict.conflictingModId,
                              conflict.activeModName
                            )
                          }
                          disabled={isBusy}
                          className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-sky-950/60 hover:bg-sky-900/60 text-sky-200 border border-sky-800 text-xs font-semibold transition-colors disabled:opacity-50"
                        >
                          <Zap className="w-3.5 h-3.5 text-sky-400" />
                          <span>Accept Shared Dependency</span>
                        </button>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      )}
    </div>
  );
};
