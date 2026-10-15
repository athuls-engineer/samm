import React from 'react';
import { FileConflict } from '../types';
import { AlertTriangle, CheckCircle2, ShieldAlert, Layers } from 'lucide-react';

interface ConflictsProps {
  conflicts: FileConflict[];
  onRefresh: () => void;
}

export const Conflicts: React.FC<ConflictsProps> = ({ conflicts, onRefresh }) => {
  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      <div>
        <h1 className="text-lg font-bold text-white">File Collision & Conflict Inspector</h1>
        <p className="text-xs text-neutral-400">
          Identifies destination path collisions between multiple installed mods targeting the same game file.
        </p>
      </div>

      {conflicts.length === 0 ? (
        <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-2">
          <CheckCircle2 className="w-8 h-8 text-emerald-400 mx-auto" />
          <div className="text-sm font-semibold text-neutral-200">Zero File Collisions Detected</div>
          <div>All installed mods have unique destination files or are cleanly isolated inside ModLoader folders.</div>
        </div>
      ) : (
        <div className="space-y-3">
          {conflicts.map((conflict) => (
            <div
              key={conflict.id}
              className="bg-bg-card border border-amber-800/80 rounded-lg p-5 space-y-3"
            >
              <div className="flex items-start justify-between">
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <ShieldAlert className="w-4 h-4 text-amber-400" />
                    <span className="font-bold text-xs text-white">Destination Collision</span>
                    <span className="font-mono text-xs font-semibold text-amber-300 bg-amber-950 px-2 py-0.5 rounded border border-amber-900">
                      {conflict.destinationRelPath}
                    </span>
                  </div>
                  <div className="text-xs text-neutral-300">
                    Active Supplier: <strong className="text-white">{conflict.activeModName}</strong> | Colliding Mod: <strong className="text-amber-200">{conflict.conflictingModName}</strong>
                  </div>
                </div>

                <span className="text-[10px] uppercase font-bold text-amber-400 bg-amber-950/80 px-2 py-0.5 rounded border border-amber-800">
                  {conflict.status}
                </span>
              </div>

              {conflict.resolutionNotes && (
                <div className="p-3 bg-neutral-900 rounded border border-neutral-800 text-[11px] text-neutral-300">
                  {conflict.resolutionNotes}
                </div>
              )}

              <div className="p-3 bg-neutral-800/60 rounded text-[11px] text-neutral-400 flex items-center gap-2">
                <Layers className="w-4 h-4 text-accent flex-shrink-0" />
                <span>
                  <strong>Safe Resolution Recommendation:</strong> For texture and model replacements, install both mods into separate ModLoader folders. ModLoader resolves asset priority in memory without corrupting game files.
                </span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
