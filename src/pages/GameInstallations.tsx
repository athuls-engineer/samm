import React, { useState } from 'react';
import { GameInstallation } from '../types';
import { api } from '../api/client';
import {
  Gamepad2,
  Search,
  CheckCircle2,
  AlertCircle,
  HelpCircle,
  FolderOpen,
  Trash2,
  RefreshCw,
  PlusCircle,
  Check,
} from 'lucide-react';

interface GameInstallationsProps {
  installations: GameInstallation[];
  onSelectActive: (id: string) => void;
  onRefresh: () => void;
}

export const GameInstallations: React.FC<GameInstallationsProps> = ({
  installations,
  onSelectActive,
  onRefresh,
}) => {
  const [manualPath, setManualPath] = useState('');
  const [inspecting, setInspecting] = useState(false);
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleManualInspect = async () => {
    if (!manualPath.trim()) return;
    setInspecting(true);
    setError(null);
    try {
      const inspected = await api.inspectGamePath(manualPath.trim());
      await api.saveGameInstallation(inspected);
      setManualPath('');
      onRefresh();
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Inspection failed');
    } finally {
      setInspecting(false);
    }
  };

  const handleScanSystem = async () => {
    setScanning(true);
    setError(null);
    try {
      const discovered = await api.discoverInstallations();
      for (const d of discovered) {
        await api.saveGameInstallation(d);
      }
      onRefresh();
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Scan failed');
    } finally {
      setScanning(false);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await api.deleteGameInstallation(id);
      onRefresh();
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Failed to remove installation');
    }
  };

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h1 className="text-lg font-bold text-white">Game Detection & Version Verification</h1>
          <p className="text-xs text-neutral-400">
            Evidence-based classification of GTA: San Andreas installations against the verified Classic PC 1.0 baseline.
          </p>
        </div>

        <button
          onClick={handleScanSystem}
          disabled={scanning}
          className="px-4 py-2 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs font-semibold text-white flex items-center gap-2 transition-colors self-start md:self-auto"
        >
          <RefreshCw className={`w-3.5 h-3.5 ${scanning ? 'animate-spin' : ''}`} />
          <span>{scanning ? 'Scanning Steam & Program Files...' : 'Scan Common Paths'}</span>
        </button>
      </div>

      {/* Manual Path Entry Box */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-3">
        <h2 className="text-xs font-bold text-white flex items-center gap-2">
          <FolderOpen className="w-4 h-4 text-accent" />
          Manually Add Game Folder or Executable
        </h2>
        <div className="flex gap-2">
          <input
            type="text"
            value={manualPath}
            onChange={(e) => setManualPath(e.target.value)}
            placeholder="e.g. C:\Games\GTA San Andreas or C:\Program Files (x86)\Steam\steamapps\common\Grand Theft Auto San Andreas"
            className="flex-1 bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white placeholder-neutral-500 font-mono focus:outline-none focus:border-accent"
          />
          <button
            onClick={handleManualInspect}
            disabled={inspecting || !manualPath.trim()}
            className={`px-4 py-1.5 rounded text-xs font-bold flex items-center gap-1.5 ${
              manualPath.trim()
                ? 'bg-accent hover:bg-accent-hover text-bg-main shadow'
                : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
            }`}
          >
            <span>{inspecting ? 'Inspecting Evidence...' : 'Inspect & Save'}</span>
          </button>
        </div>

        {error && (
          <div className="p-3 bg-rose-950/40 border border-rose-800 rounded text-xs text-rose-300 flex items-center gap-2">
            <AlertCircle className="w-4 h-4 flex-shrink-0" />
            <span>{error}</span>
          </div>
        )}
      </div>

      {/* Installations List */}
      <div className="space-y-4">
        {installations.map((inst) => (
          <div
            key={inst.id}
            className={`bg-bg-card border rounded-lg p-5 space-y-4 transition-colors ${
              inst.isActive ? 'border-accent shadow-sm' : 'border-border'
            }`}
          >
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-3">
              <div>
                <div className="flex items-center gap-2.5">
                  <Gamepad2 className="w-5 h-5 text-accent" />
                  <h3 className="font-bold text-sm text-white">{inst.name}</h3>
                  {inst.isClassic10 && inst.versionConfidence === 'confirmed' ? (
                    <span className="text-[11px] font-semibold text-emerald-400 bg-emerald-950/70 border border-emerald-800 px-2 py-0.5 rounded flex items-center gap-1">
                      <CheckCircle2 className="w-3.5 h-3.5" /> Classic 1.0 (Confirmed)
                    </span>
                  ) : (
                    <span className="text-[11px] font-semibold text-amber-400 bg-amber-950/70 border border-amber-800 px-2 py-0.5 rounded flex items-center gap-1">
                      <AlertCircle className="w-3.5 h-3.5" /> {inst.detectedEdition}
                    </span>
                  )}
                </div>
                <div className="text-xs text-neutral-400 font-mono mt-1">
                  Root: <span className="text-neutral-200">{inst.rootPath}</span>
                </div>
                <div className="text-[11px] text-neutral-400 font-mono">
                  Exe SHA-256: {inst.executableSha256 || 'N/A'} ({(inst.executableSizeBytes / (1024 * 1024)).toFixed(2)} MB)
                </div>
              </div>

              {/* Activation action */}
              <div className="flex items-center gap-2">
                {inst.isActive ? (
                  <span className="flex items-center gap-1.5 px-3 py-1 rounded bg-emerald-950 text-emerald-400 border border-emerald-800 text-xs font-bold">
                    <Check className="w-3.5 h-3.5" />
                    <span>Active Target</span>
                  </span>
                ) : (
                  <button
                    onClick={() => onSelectActive(inst.id)}
                    className="px-4 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs font-semibold text-white border border-neutral-700 transition-colors"
                  >
                    Set as Active
                  </button>
                )}

                <button
                  onClick={() => handleDelete(inst.id)}
                  className="p-1.5 text-neutral-500 hover:text-rose-400 rounded transition-colors"
                  title="Remove from SAMM"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              </div>
            </div>

            {/* Evidence Checklist */}
            <div className="bg-bg-main/60 border border-border-subtle rounded p-3 space-y-2 text-xs">
              <div className="text-[11px] font-semibold text-neutral-400 uppercase tracking-wider">
                Inspection Evidence Report
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                {inst.evidenceList.map((ev, idx) => (
                  <div key={idx} className="flex items-start gap-2 text-neutral-300">
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 flex-shrink-0 mt-0.5" />
                    <span className="text-[11px] leading-tight">{ev}</span>
                  </div>
                ))}
              </div>

              {inst.uncertainties.length > 0 && (
                <div className="pt-2 border-t border-border-subtle space-y-1">
                  <div className="text-[11px] font-semibold text-amber-400">Uncertainties & Warnings:</div>
                  {inst.uncertainties.map((un, idx) => (
                    <div key={idx} className="flex items-start gap-2 text-amber-300 text-[11px]">
                      <AlertCircle className="w-3.5 h-3.5 text-amber-400 flex-shrink-0 mt-0.5" />
                      <span>{un}</span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
