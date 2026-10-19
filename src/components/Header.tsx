import React from 'react';
import { GameInstallation, Profile } from '../types';
import {
  Play,
  PlusCircle,
  CheckCircle2,
  AlertCircle,
  HelpCircle,
  XCircle,
  FolderOpen,
  Square,
  RotateCcw,
  Loader2,
} from 'lucide-react';

interface HeaderProps {
  activeGame: GameInstallation | null;
  activeProfile: Profile | null;
  isGameRunning?: boolean;
  isLaunching?: boolean;
  onLaunchGame: () => void;
  onKillGame?: () => void;
  onRestartGame?: () => void;
  onImportArchive: () => void;
  onSelectGameClick: () => void;
  onOpenGameFolder?: () => void;
}

export const Header: React.FC<HeaderProps> = ({
  activeGame,
  activeProfile,
  isGameRunning = false,
  isLaunching = false,
  onLaunchGame,
  onKillGame,
  onRestartGame,
  onImportArchive,
  onSelectGameClick,
  onOpenGameFolder,
}) => {
  const renderConfidenceBadge = (confidence?: string, is10?: boolean) => {
    if (!confidence) return null;

    if (confidence === 'confirmed' && is10) {
      return (
        <span className="flex items-center gap-1 text-[10px] font-medium text-emerald-400 bg-emerald-950/60 border border-emerald-800/80 px-1.5 py-0.5 rounded flex-shrink-0">
          <CheckCircle2 className="w-3 h-3" />
          <span className="hidden sm:inline">Classic 1.0 (Verified)</span>
          <span className="sm:hidden">1.0</span>
        </span>
      );
    }
    if (confidence === 'likely') {
      return (
        <span className="flex items-center gap-1 text-[10px] font-medium text-amber-400 bg-amber-950/60 border border-amber-800/80 px-1.5 py-0.5 rounded flex-shrink-0">
          <AlertCircle className="w-3 h-3" />
          <span className="hidden sm:inline">Likely 1.0</span>
          <span className="sm:hidden">1.0</span>
        </span>
      );
    }
    if (confidence === 'ambiguous') {
      return (
        <span className="flex items-center gap-1 text-[10px] font-medium text-blue-400 bg-blue-950/60 border border-blue-800/80 px-1.5 py-0.5 rounded flex-shrink-0">
          <HelpCircle className="w-3 h-3" />
          <span>Steam / Mod</span>
        </span>
      );
    }
    return (
      <span className="flex items-center gap-1 text-[10px] font-medium text-rose-400 bg-rose-950/60 border border-rose-800/80 px-1.5 py-0.5 rounded flex-shrink-0">
        <XCircle className="w-3 h-3" />
        <span>Incompatible</span>
      </span>
    );
  };

  return (
    <header className="h-16 border-b border-border-subtle bg-bg-subtle/80 px-4 md:px-6 flex items-center justify-between gap-3 overflow-hidden select-none">
      {/* Game info & profile pill */}
      <div className="flex items-center gap-2 min-w-0 flex-1 overflow-hidden">
        {activeGame ? (
          <div className="flex items-center gap-1.5 bg-bg-card border border-border rounded-md px-2.5 py-1 shadow-sm min-w-0">
            <button
              onClick={onSelectGameClick}
              className="text-left py-0.5 rounded cursor-pointer hover:opacity-80 transition-opacity min-w-0"
              title="Click to switch game installation"
            >
              <div className="text-xs font-semibold text-white flex items-center gap-1.5 min-w-0">
                <span className="truncate max-w-[120px] sm:max-w-[160px] md:max-w-[200px]">{activeGame.name}</span>
                {renderConfidenceBadge(activeGame.versionConfidence, activeGame.isClassic10)}
              </div>
              <div className="text-[10px] text-neutral-400 font-mono truncate max-w-[140px] sm:max-w-[200px] md:max-w-[280px]">
                {activeGame.rootPath}
              </div>
            </button>

            {onOpenGameFolder && (
              <button
                onClick={onOpenGameFolder}
                className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded transition-colors ml-1 flex-shrink-0"
                title="Open Game Directory in File Explorer"
              >
                <FolderOpen className="w-3.5 h-3.5 text-accent" />
              </button>
            )}
          </div>
        ) : (
          <button
            onClick={onSelectGameClick}
            className="flex items-center gap-2 bg-amber-900/30 border border-amber-700/60 text-amber-300 px-3 py-1.5 rounded-md text-xs hover:bg-amber-900/50 transition-colors shadow-sm flex-shrink-0"
          >
            <AlertCircle className="w-4 h-4 text-amber-400" />
            <span>Select Game Installation</span>
          </button>
        )}

        {/* Profile Pill (collapses on smaller screens to prevent crowding) */}
        {activeProfile && (
          <div className="hidden xl:flex text-xs px-2.5 py-1.5 rounded-md bg-neutral-800/80 border border-neutral-700 text-neutral-300 font-medium items-center gap-1.5 shadow-sm flex-shrink-0">
            <span className="w-1.5 h-1.5 rounded-full bg-accent"></span>
            <span className="text-neutral-400 text-[10px] uppercase tracking-wider font-mono">Profile:</span>
            <span className="truncate max-w-[120px] font-semibold text-white">{activeProfile.name}</span>
          </div>
        )}
      </div>

      {/* Primary Actions & Game Session Widget */}
      <div className="flex items-center gap-2 flex-shrink-0">
        {/* Import Mod Button */}
        <button
          onClick={onImportArchive}
          className="h-9 px-3.5 rounded-md bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-200 font-semibold transition-colors shadow-sm flex items-center gap-1.5 cursor-pointer"
          title="Install a mod from a .zip, .rar, .7z, or standalone .asi/.cs file"
        >
          <PlusCircle className="w-3.5 h-3.5 text-accent" />
          <span className="hidden sm:inline">Import Mod</span>
        </button>

        {/* Unified Game Control / Status */}
        {isGameRunning ? (
          <div className="h-9 flex items-center gap-1 bg-neutral-900 border border-neutral-700 rounded-md px-1 shadow-sm">
            {/* Running Status Indicator */}
            <div className="flex items-center gap-1.5 px-2 py-1 text-emerald-400 text-xs font-semibold select-none">
              <span className="relative flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
              </span>
              <span>Running</span>
            </div>

            {/* 1-Click Restart Button */}
            {onRestartGame && (
              <button
                onClick={onRestartGame}
                className="h-7 px-2.5 rounded bg-amber-500/15 hover:bg-amber-500/25 border border-amber-500/40 text-xs text-amber-300 font-semibold transition-colors flex items-center gap-1 cursor-pointer"
                title="Restart GTA San Andreas to apply mod changes"
              >
                <RotateCcw className="w-3.5 h-3.5" />
                <span>Restart</span>
              </button>
            )}

            {/* Kill Game Button */}
            {onKillGame && (
              <button
                onClick={onKillGame}
                className="h-7 px-2 rounded bg-rose-500/15 hover:bg-rose-500/25 border border-rose-500/40 text-xs text-rose-300 font-semibold transition-colors flex items-center gap-1 cursor-pointer"
                title="Force close GTA San Andreas"
              >
                <Square className="w-3 h-3 fill-current" />
                <span>Stop</span>
              </button>
            )}
          </div>
        ) : (
          <button
            onClick={onLaunchGame}
            disabled={!activeGame || isLaunching}
            className={`h-9 px-4 rounded-md text-xs font-bold shadow-sm transition-colors flex items-center gap-1.5 ${
              activeGame && !isLaunching
                ? 'bg-accent hover:bg-accent-hover text-bg-main active:scale-[0.98] cursor-pointer'
                : 'bg-neutral-800 text-neutral-500 cursor-not-allowed border border-neutral-700'
            }`}
          >
            {isLaunching ? (
              <>
                <Loader2 className="w-3.5 h-3.5 animate-spin text-accent" />
                <span>Launching...</span>
              </>
            ) : (
              <>
                <Play className="w-3.5 h-3.5 fill-current" />
                <span>Launch Game</span>
              </>
            )}
          </button>
        )}
      </div>
    </header>
  );
};
