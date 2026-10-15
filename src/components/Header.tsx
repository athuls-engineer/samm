import React from 'react';
import { GameInstallation, Profile } from '../types';
import { Play, PlusCircle, CheckCircle2, AlertCircle, HelpCircle, XCircle } from 'lucide-react';

interface HeaderProps {
  activeGame: GameInstallation | null;
  activeProfile: Profile | null;
  onLaunchGame: () => void;
  onImportArchive: () => void;
  onSelectGameClick: () => void;
}

export const Header: React.FC<HeaderProps> = ({
  activeGame,
  activeProfile,
  onLaunchGame,
  onImportArchive,
  onSelectGameClick,
}) => {
  const renderConfidenceBadge = (confidence?: string, is10?: boolean) => {
    if (!confidence) return null;

    if (confidence === 'confirmed' && is10) {
      return (
        <span className="flex items-center gap-1 text-[11px] font-medium text-emerald-400 bg-emerald-950/60 border border-emerald-800/80 px-2 py-0.5 rounded">
          <CheckCircle2 className="w-3.5 h-3.5" />
          Classic 1.0 (Verified)
        </span>
      );
    }
    if (confidence === 'likely') {
      return (
        <span className="flex items-center gap-1 text-[11px] font-medium text-amber-400 bg-amber-950/60 border border-amber-800/80 px-2 py-0.5 rounded">
          <AlertCircle className="w-3.5 h-3.5" />
          Likely 1.0 (Patched)
        </span>
      );
    }
    if (confidence === 'ambiguous') {
      return (
        <span className="flex items-center gap-1 text-[11px] font-medium text-blue-400 bg-blue-950/60 border border-blue-800/80 px-2 py-0.5 rounded">
          <HelpCircle className="w-3.5 h-3.5" />
          Steam / Modified
        </span>
      );
    }
    return (
      <span className="flex items-center gap-1 text-[11px] font-medium text-rose-400 bg-rose-950/60 border border-rose-800/80 px-2 py-0.5 rounded">
        <XCircle className="w-3.5 h-3.5" />
        Incompatible
      </span>
    );
  };

  return (
    <header className="h-16 border-b border-border-subtle bg-bg-subtle/80 px-6 flex items-center justify-between">
      {/* Game info pill */}
      <div className="flex items-center gap-3">
        {activeGame ? (
          <div
            onClick={onSelectGameClick}
            className="flex items-center gap-3 bg-bg-card hover:bg-bg-hover border border-border px-3 py-1.5 rounded cursor-pointer transition-colors"
          >
            <div>
              <div className="text-xs font-semibold text-white flex items-center gap-2">
                <span>{activeGame.name}</span>
                {renderConfidenceBadge(activeGame.versionConfidence, activeGame.isClassic10)}
              </div>
              <div className="text-[11px] text-neutral-400 font-mono truncate max-w-sm">
                {activeGame.rootPath}
              </div>
            </div>
          </div>
        ) : (
          <button
            onClick={onSelectGameClick}
            className="flex items-center gap-2 bg-amber-900/30 border border-amber-700/60 text-amber-300 px-3 py-1.5 rounded text-xs hover:bg-amber-900/50 transition-colors"
          >
            <AlertCircle className="w-4 h-4" />
            <span>No Game Installation Configured - Click to Locate</span>
          </button>
        )}

        {/* Profile Pill */}
        {activeProfile && (
          <div className="text-xs px-2.5 py-1 rounded bg-neutral-800/80 border border-neutral-700 text-neutral-300 font-medium flex items-center gap-1.5">
            <span className="w-1.5 h-1.5 rounded-full bg-accent"></span>
            <span className="text-neutral-400 text-[10px] uppercase tracking-wider">Profile:</span>
            <span>{activeProfile.name}</span>
          </div>
        )}
      </div>

      {/* Primary Actions */}
      <div className="flex items-center gap-3">
        <button
          onClick={onImportArchive}
          className="flex items-center gap-2 px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-200 font-medium transition-colors"
        >
          <PlusCircle className="w-4 h-4 text-accent" />
          <span>Import Mod Archive</span>
        </button>

        <button
          onClick={onLaunchGame}
          disabled={!activeGame}
          className={`flex items-center gap-2 px-4 py-1.5 rounded text-xs font-semibold shadow transition-colors ${
            activeGame
              ? 'bg-accent hover:bg-accent-hover text-bg-main'
              : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
          }`}
        >
          <Play className="w-4 h-4 fill-current" />
          <span>Launch Game</span>
        </button>
      </div>
    </header>
  );
};
