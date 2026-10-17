import React, { useState } from 'react';
import { GameInstallation } from '../types';
import { api } from '../api/client';
import {
  Gamepad2,
  CheckCircle2,
  AlertCircle,
  FolderOpen,
  Trash2,
  RefreshCw,
  Check,
  FolderSearch,
  ShieldCheck,
  ChevronRight,
} from 'lucide-react';

interface GameInstallationsProps {
  installations: GameInstallation[];
  onSelectActive: (id: string) => void;
  onRefresh: () => void;
  onNavigateToInstalled?: () => void;
}

export const GameInstallations: React.FC<GameInstallationsProps> = ({
  installations,
  onSelectActive,
  onRefresh,
  onNavigateToInstalled,
}) => {
  const [manualPath, setManualPath] = useState('');
  const [inspecting, setInspecting] = useState(false);
  const [scanning, setScanning] = useState(false);
  const [rescanningId, setRescanningId] = useState<string | null>(null);
  const [rescanResult, setRescanResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleManualInspect = async (pathToInspect?: string) => {
    const target = (pathToInspect || manualPath).trim();
    if (!target) return;
    setInspecting(true);
    setError(null);
    setRescanResult(null);
    try {
      const inspected = await api.inspectGamePath(target);
      await api.saveGameInstallation(inspected);
      setManualPath('');
      onRefresh();
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Failed to inspect game directory');
    } finally {
      setInspecting(false);
    }
  };

  const handleBrowseFolder = async () => {
    setError(null);
    setRescanResult(null);
    try {
      const selected = await api.pickFolder();
      if (selected) {
        setManualPath(selected);
        await handleManualInspect(selected);
      }
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Folder selection cancelled or failed');
    }
  };

  const handleRescanExistingMods = async (gameId: string) => {
    setRescanningId(gameId);
    setError(null);
    setRescanResult(null);
    try {
      const synced = await api.syncExistingMods(gameId);
      onRefresh();
      setRescanResult(`Scan complete: Found and registered ${synced.length} existing mods in this folder.`);
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Failed to scan existing mods');
    } finally {
      setRescanningId(null);
    }
  };

  const handleScanSystem = async () => {
    setScanning(true);
    setError(null);
    setRescanResult(null);
    try {
      const discovered = await api.discoverInstallations();
      for (const d of discovered) {
        await api.saveGameInstallation(d);
      }
      onRefresh();
    } catch (e: any) {
      setError(typeof e === 'string' ? e : e?.message || 'Auto-scan failed');
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
    <div className="p-6 md:p-8 space-y-6 max-w-6xl mx-auto">
      {/* Top Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border-subtle pb-5">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-white">Game Installations & Version Setup</h1>
          <p className="text-xs text-neutral-400 mt-1">
            Configure your GTA: San Andreas directory. SAMM verifies PC v1.0 compatibility and auto-detects any existing mods.
          </p>
        </div>

        <button
          onClick={handleScanSystem}
          disabled={scanning}
          className="px-4 py-2 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs font-semibold text-white flex items-center gap-2 transition-colors self-start md:self-auto"
        >
          <RefreshCw className={`w-3.5 h-3.5 ${scanning ? 'animate-spin' : ''}`} />
          <span>{scanning ? 'Scanning System...' : 'Auto-Scan Steam / Program Files'}</span>
        </button>
      </div>

      {/* Folder Selection Box */}
      <div className="bg-bg-card border border-border rounded-lg p-5 space-y-3 shadow-sm">
        <div className="flex items-center justify-between">
          <h2 className="text-xs font-bold text-white flex items-center gap-2">
            <FolderOpen className="w-4 h-4 text-accent" />
            <span>Select GTA: San Andreas Folder</span>
          </h2>
          <span className="text-[11px] text-neutral-400">Directly browse or paste your game path</span>
        </div>

        <div className="flex flex-col sm:flex-row gap-2">
          <input
            type="text"
            value={manualPath}
            onChange={(e) => setManualPath(e.target.value)}
            placeholder="e.g. C:\Games\GTA San Andreas"
            className="flex-1 bg-bg-main border border-border rounded px-3 py-2 text-xs text-white placeholder-neutral-500 font-mono focus:outline-none focus:border-accent"
          />

          <button
            onClick={handleBrowseFolder}
            disabled={inspecting}
            className="px-4 py-2 rounded text-xs font-semibold bg-neutral-800 hover:bg-neutral-700 text-neutral-200 border border-neutral-700 flex items-center justify-center gap-2 transition-colors"
          >
            <FolderSearch className="w-3.5 h-3.5 text-accent" />
            <span>Browse Folder...</span>
          </button>

          <button
            onClick={() => handleManualInspect()}
            disabled={inspecting || !manualPath.trim()}
            className={`px-5 py-2 rounded text-xs font-bold flex items-center justify-center gap-1.5 transition-all ${
              manualPath.trim() && !inspecting
                ? 'bg-accent hover:bg-accent-hover text-bg-main shadow'
                : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
            }`}
          >
            <span>{inspecting ? 'Checking Directory...' : 'Add Folder'}</span>
          </button>
        </div>

        {error && (
          <div className="p-3 bg-rose-950/40 border border-rose-800 rounded text-xs text-rose-300 flex items-center gap-2">
            <AlertCircle className="w-4 h-4 flex-shrink-0" />
            <span>{error}</span>
          </div>
        )}

        {rescanResult && (
          <div className="p-3 bg-emerald-950/40 border border-emerald-800 rounded text-xs text-emerald-300 flex items-center justify-between gap-2">
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400 flex-shrink-0" />
              <span>{rescanResult}</span>
            </div>
            {onNavigateToInstalled && (
              <button
                onClick={onNavigateToInstalled}
                className="px-2.5 py-1 rounded bg-emerald-900/80 hover:bg-emerald-800 text-white font-semibold text-[11px] transition-colors flex items-center gap-1"
              >
                <span>View Installed Mods</span>
                <ChevronRight className="w-3 h-3" />
              </button>
            )}
          </div>
        )}
      </div>

      {/* Installations List */}
      <div className="space-y-4">
        {installations.map((inst) => (
          <div
            key={inst.id}
            className={`bg-bg-card border rounded-lg p-5 space-y-4 transition-all shadow-sm ${
              inst.isActive ? 'border-accent ring-1 ring-accent/30' : 'border-border'
            }`}
          >
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-3">
              <div>
                <div className="flex items-center gap-2.5">
                  <Gamepad2 className="w-5 h-5 text-accent" />
                  <h3 className="font-bold text-sm text-white">{inst.name}</h3>
                  {inst.isClassic10 && inst.versionConfidence === 'confirmed' ? (
                    <span className="text-[11px] font-semibold text-emerald-400 bg-emerald-950/70 border border-emerald-800 px-2 py-0.5 rounded flex items-center gap-1">
                      <CheckCircle2 className="w-3.5 h-3.5" /> Classic 1.0 (Verified)
                    </span>
                  ) : (
                    <span className="text-[11px] font-semibold text-amber-400 bg-amber-950/70 border border-amber-800 px-2 py-0.5 rounded flex items-center gap-1">
                      <AlertCircle className="w-3.5 h-3.5" /> {inst.detectedEdition}
                    </span>
                  )}
                </div>
                <div className="text-xs text-neutral-400 font-mono mt-1">
                  Root Directory: <span className="text-neutral-200">{inst.rootPath}</span>
                </div>
                <div className="text-[11px] text-neutral-400 font-mono mt-0.5">
                  Executable: {inst.executablePath.split(/[\\/]/).pop()} | SHA-256: {inst.executableSha256 ? `${inst.executableSha256.slice(0, 16)}...` : 'N/A'} ({(inst.executableSizeBytes / (1024 * 1024)).toFixed(2)} MB)
                </div>
              </div>

              {/* Action buttons */}
              <div className="flex items-center gap-2 flex-wrap">
                <button
                  onClick={() => api.openPathInExplorer(inst.rootPath)}
                  className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs font-semibold text-neutral-200 border border-neutral-700 flex items-center gap-1.5 transition-colors"
                  title="Open this game directory in Windows Explorer"
                >
                  <FolderOpen className="w-3.5 h-3.5 text-accent" />
                  <span>Folder</span>
                </button>

                <button
                  onClick={() => handleRescanExistingMods(inst.id)}
                  disabled={rescanningId === inst.id}
                  className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs font-semibold text-amber-300 border border-neutral-700 flex items-center gap-1.5 transition-colors"
                  title="Scan this game folder for pre-installed ModLoader folders, ASI plugins, and CLEO scripts"
                >
                  <RefreshCw className={`w-3.5 h-3.5 ${rescanningId === inst.id ? 'animate-spin' : ''}`} />
                  <span>{rescanningId === inst.id ? 'Scanning...' : 'Rescan Mods'}</span>
                </button>

                {inst.isActive ? (
                  <span className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-emerald-950 text-emerald-400 border border-emerald-800 text-xs font-bold">
                    <Check className="w-3.5 h-3.5" />
                    <span>Active Target</span>
                  </span>
                ) : (
                  <button
                    onClick={() => onSelectActive(inst.id)}
                    className="px-3.5 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 text-xs font-semibold text-white border border-neutral-700 transition-colors"
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

            {/* Invariants & File Verification */}
            <div className="bg-bg-main/60 border border-border-subtle rounded p-3 space-y-2 text-xs">
              <div className="text-[11px] font-semibold text-neutral-400 uppercase tracking-wider flex items-center gap-1.5">
                <ShieldCheck className="w-3 h-3 text-accent" />
                <span>Verified Game Invariants & Structure</span>
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
                  <div className="text-[11px] font-semibold text-amber-400">Notes & Warnings:</div>
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
