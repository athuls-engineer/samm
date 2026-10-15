import React, { useState } from 'react';
import { ArchiveInspection } from '../types';
import { api } from '../api/client';
import { FileArchive, CheckCircle2, AlertTriangle, X, Search } from 'lucide-react';

interface ArchiveImportModalProps {
  onClose: () => void;
  onPlanReady: (archivePath: string, modTitle: string) => void;
}

export const ArchiveImportModal: React.FC<ArchiveImportModalProps> = ({
  onClose,
  onPlanReady,
}) => {
  const [archivePath, setArchivePath] = useState('');
  const [modTitle, setModTitle] = useState('');
  const [inspecting, setInspecting] = useState(false);
  const [inspection, setInspection] = useState<ArchiveInspection | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleInspect = async () => {
    if (!archivePath.trim()) return;
    setInspecting(true);
    setError(null);
    setInspection(null);

    try {
      const res = await api.inspectArchive(archivePath.trim());
      setInspection(res);
      if (!modTitle.trim()) {
        const guessedTitle = archivePath.split(/[\\/]/).pop()?.replace(/\.(zip|rar|7z)$/i, '').replace(/_/g, ' ') || 'Imported Mod';
        setModTitle(guessedTitle);
      }
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Failed to inspect archive');
    } finally {
      setInspecting(false);
    }
  };

  const handleProceed = () => {
    if (!archivePath.trim() || !modTitle.trim() || !inspection?.isSafe) return;
    onPlanReady(archivePath.trim(), modTitle.trim());
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="bg-bg-subtle border border-border rounded-lg shadow-2xl max-w-xl w-full flex flex-col overflow-hidden animate-in fade-in duration-150">
        <div className="p-4 border-b border-border flex items-center justify-between bg-bg-card">
          <div className="flex items-center gap-2.5">
            <FileArchive className="w-5 h-5 text-accent" />
            <div>
              <h2 className="text-xs font-bold text-white">Import Local Mod Archive</h2>
              <p className="text-[11px] text-neutral-400">
                Safe inspection and installation of manually downloaded ZIP packages.
              </p>
            </div>
          </div>
          <button onClick={onClose} className="text-neutral-400 hover:text-white p-1 rounded">
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-5 space-y-4">
          <div>
            <label className="block text-xs font-semibold text-neutral-300 mb-1">
              Local Archive File Path (ZIP)
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={archivePath}
                onChange={(e) => setArchivePath(e.target.value)}
                placeholder="C:\Downloads\silentpatch_sa.zip"
                className="flex-1 bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white placeholder-neutral-500 font-mono focus:outline-none focus:border-accent"
              />
              <button
                onClick={handleInspect}
                disabled={inspecting || !archivePath.trim()}
                className={`px-3 py-1.5 rounded text-xs font-semibold flex items-center gap-1.5 ${
                  archivePath.trim()
                    ? 'bg-neutral-700 hover:bg-neutral-600 text-white'
                    : 'bg-neutral-800 text-neutral-500 cursor-not-allowed'
                }`}
              >
                <Search className="w-3.5 h-3.5" />
                <span>{inspecting ? 'Inspecting...' : 'Inspect'}</span>
              </button>
            </div>
          </div>

          {error && (
            <div className="p-3 rounded bg-rose-950/40 border border-rose-800/80 text-rose-300 text-xs flex items-center gap-2">
              <AlertTriangle className="w-4 h-4 flex-shrink-0" />
              <span>{error}</span>
            </div>
          )}

          {inspection && (
            <div className="space-y-3">
              <div className="bg-bg-card border border-border p-3.5 rounded text-xs space-y-2">
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-white">Archive Safety Verification</span>
                  {inspection.isSafe ? (
                    <span className="flex items-center gap-1 text-[11px] text-emerald-400 font-bold">
                      <CheckCircle2 className="w-3.5 h-3.5" /> Safe to Extract
                    </span>
                  ) : (
                    <span className="flex items-center gap-1 text-[11px] text-rose-400 font-bold">
                      <AlertTriangle className="w-3.5 h-3.5" /> Traversal Detected
                    </span>
                  )}
                </div>

                <div className="grid grid-cols-3 gap-2 text-[11px] text-neutral-300 font-mono">
                  <div>Files: {inspection.totalEntries}</div>
                  <div>Size: {(inspection.totalUncompressedSize / (1024 * 1024)).toFixed(2)} MB</div>
                  <div>Ratio: {inspection.compressionRatio}:1</div>
                </div>

                {inspection.safetyViolations.length > 0 && (
                  <div className="p-2 rounded bg-rose-950/60 border border-rose-800 text-rose-300 text-[11px]">
                    <strong>Violations:</strong> {inspection.safetyViolations.join(', ')}
                  </div>
                )}
              </div>

              <div>
                <label className="block text-xs font-semibold text-neutral-300 mb-1">
                  Mod Display Name
                </label>
                <input
                  type="text"
                  value={modTitle}
                  onChange={(e) => setModTitle(e.target.value)}
                  placeholder="e.g. SilentPatch"
                  className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent"
                />
              </div>
            </div>
          )}
        </div>

        <div className="p-4 border-t border-border bg-bg-card flex items-center justify-between">
          <button
            onClick={onClose}
            className="px-4 py-1.5 rounded text-xs font-medium text-neutral-400 hover:text-white"
          >
            Cancel
          </button>
          <button
            onClick={handleProceed}
            disabled={!inspection || !inspection.isSafe || !modTitle.trim()}
            className={`px-5 py-2 rounded text-xs font-bold transition-colors shadow ${
              inspection && inspection.isSafe && modTitle.trim()
                ? 'bg-accent hover:bg-accent-hover text-bg-main'
                : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
            }`}
          >
            Proceed to Installation Plan
          </button>
        </div>
      </div>
    </div>
  );
};
