import React, { useState } from 'react';
import { FileConflict, GameInstallation } from '../types';
import { api } from '../api/client';
import {
  AlertTriangle,
  CheckCircle2,
  ShieldAlert,
  Layers,
  Check,
  Zap,
  ArrowRight,
  RefreshCw,
  FileCode,
  ShieldCheck,
  Info,
} from 'lucide-react';

interface ConflictsProps {
  conflicts: FileConflict[];
  activeGame: GameInstallation | null;
  onRefresh: () => void;
  onToast: (text: string, type?: 'success' | 'error') => void;
}

export const Conflicts: React.FC<ConflictsProps> = ({
  conflicts,
  activeGame,
  onRefresh,
  onToast,
}) => {
  const [resolvingId, setResolvingId] = useState<string | null>(null);
  const [autoResolving, setAutoResolving] = useState(false);

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

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      {/* Header and Auto-Resolve action */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border-subtle pb-5">
        <div>
          <div className="flex items-center gap-2.5">
            <h1 className="text-xl font-bold tracking-tight text-white">File Conflicts</h1>
            {conflicts.length > 0 ? (
              <span className="text-[11px] font-mono px-2 py-0.5 rounded-full bg-amber-950/80 text-amber-300 border border-amber-800 font-bold">
                {conflicts.length} Unresolved
              </span>
            ) : (
              <span className="text-[11px] font-mono px-2 py-0.5 rounded-full bg-emerald-950/80 text-emerald-400 border border-emerald-800 font-bold flex items-center gap-1">
                <CheckCircle2 className="w-3.5 h-3.5" /> Conflict Free
              </span>
            )}
          </div>
          <p className="text-xs text-neutral-400 mt-1">
            Resolve overlapping files between multiple installed mods. Choose which mod wins or use 1-click auto-resolution.
          </p>
        </div>

        {conflicts.length > 0 && (
          <button
            onClick={handleAutoResolveAll}
            disabled={autoResolving}
            className="flex items-center gap-2 px-4 py-2 rounded-md bg-amber-500 hover:bg-amber-400 text-neutral-950 font-bold text-xs transition-all shadow-md self-start md:self-auto disabled:opacity-50"
          >
            {autoResolving ? (
              <RefreshCw className="w-4 h-4 animate-spin text-neutral-950" />
            ) : (
              <Zap className="w-4 h-4 fill-current text-neutral-950" />
            )}
            <span>Auto-Resolve All ({conflicts.length})</span>
          </button>
        )}
      </div>

      {conflicts.length === 0 ? (
        <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-3 shadow-sm">
          <div className="w-12 h-12 rounded-full bg-emerald-950/60 border border-emerald-800 flex items-center justify-center mx-auto text-emerald-400">
            <ShieldCheck className="w-6 h-6" />
          </div>
          <div className="text-sm font-bold text-white">Zero File Conflicts Detected</div>
          <p className="max-w-md mx-auto text-neutral-400 text-xs leading-relaxed">
            All installed mods have unique target paths or are cleanly isolated inside ModLoader folders. Your game directory is in a consistent state.
          </p>
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
                    Conflict Detected
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

                  {/* Option 1: Keep Active Mod */}
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

                  {/* Option 2: Use Conflicting Mod */}
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

                  {/* Option 3: Quick Shared Component Fix */}
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
  );
};
