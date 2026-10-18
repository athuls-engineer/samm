import React, { useState, useEffect, useRef, useCallback } from 'react';
import {
  GameInstallation,
  InstalledMod,
  Profile,
  ModSource,
  FileConflict,
  BackupRecord,
  DownloadJob,
  ModRecord,
  InstallationPlan,
  AutoInstallStatus,
} from './types';
import { api } from './api/client';
import { RotateCcw } from 'lucide-react';
import { Sidebar, NavTab } from './components/Sidebar';
import { Header } from './components/Header';
import { PlanPreviewModal } from './components/PlanPreviewModal';
import { FileManifestModal } from './components/FileManifestModal';
import { ArchiveImportModal } from './components/ArchiveImportModal';
import { Home } from './pages/Home';
import { DiscoverMods } from './pages/DiscoverMods';
import { ModDetailsModal } from './pages/ModDetailsModal';
import { InstalledMods } from './pages/InstalledMods';
import { Downloads } from './pages/Downloads';
import { Profiles } from './pages/Profiles';
import { GameInstallations } from './pages/GameInstallations';
import { Conflicts } from './pages/Conflicts';
import { Backups } from './pages/Backups';
import { Diagnostics } from './pages/Diagnostics';
import { Settings } from './pages/Settings';

export const App: React.FC = () => {
  const [currentTab, setCurrentTab] = useState<NavTab>('home');
  const [installations, setInstallations] = useState<GameInstallation[]>([]);
  const [activeGame, setActiveGame] = useState<GameInstallation | null>(null);
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [activeProfile, setActiveProfile] = useState<Profile | null>(null);
  const [installedMods, setInstalledMods] = useState<InstalledMod[]>([]);
  const [sources, setSources] = useState<ModSource[]>([]);
  const [conflicts, setConflicts] = useState<FileConflict[]>([]);
  const [backups, setBackups] = useState<BackupRecord[]>([]);
  const [downloadJobs, setDownloadJobs] = useState<DownloadJob[]>([]);
  const [autoInstallStates, setAutoInstallStates] = useState<Record<string, AutoInstallStatus>>({});

  // Modals state
  const [selectedModForDetails, setSelectedModForDetails] = useState<ModRecord | null>(null);
  const [inspectingManifestMod, setInspectingManifestMod] = useState<InstalledMod | null>(null);
  const [showArchiveImportModal, setShowArchiveImportModal] = useState(false);
  const [activePlan, setActivePlan] = useState<InstallationPlan | null>(null);
  const [activePlanArchivePath, setActivePlanArchivePath] = useState<string>('');
  const [isInstalling, setIsInstalling] = useState(false);
  const [globalMessage, setGlobalMessage] = useState<{ text: string; type: 'success' | 'error' } | null>(null);

  const cancelledModIdsRef = useRef<Set<string>>(new Set());
  const activeOneClickJobIdsRef = useRef<Map<string, string>>(new Map());

  const showToast = (text: string, type: 'success' | 'error' = 'success') => {
    setGlobalMessage({ text, type });
    setTimeout(() => setGlobalMessage(null), 4000);
  };

  const loadData = async () => {
    try {
      // 1. Run root queries concurrently in parallel
      const [insts, srcs, dls, appSettings] = await Promise.all([
        api.getInstallations(),
        api.listSources(),
        api.getDownloadJobs(),
        api.getSettings().catch(() => null),
      ]);

      const activeTheme = appSettings?.theme || 'cyberpunk';
      document.documentElement.className = 'theme-' + activeTheme;

      setInstallations(insts);
      setSources(srcs);
      setDownloadJobs(dls);

      const active = insts.find((i) => i.isActive) || insts[0] || null;
      setActiveGame(active);

      if (active) {
        // 2. Run active game queries concurrently in parallel
        const [profs, mods, confs, bks] = await Promise.all([
          api.getProfiles(active.id),
          api.getInstalledMods(active.id),
          api.getConflicts(active.id),
          api.getBackups(active.id),
        ]);

        setProfiles(profs);
        const activeProf = profs.find((p) => p.isActive) || profs[0] || null;
        setActiveProfile(activeProf);
        setInstalledMods(mods);
        setConflicts(confs);
        setBackups(bks);
      } else {
        setProfiles([]);
        setActiveProfile(null);
        setInstalledMods([]);
        setConflicts([]);
        setBackups([]);
      }
    } catch (e) {
      console.error('Failed to load application data:', e);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  // Poll active downloads every 2.5 seconds if any are downloading
  useEffect(() => {
    const hasActiveDownloads = downloadJobs.some((j) => j.status === 'downloading');
    if (!hasActiveDownloads) return;

    const interval = setInterval(async () => {
      try {
        const dls = await api.getDownloadJobs();
        setDownloadJobs((prev) => {
          const prevStr = JSON.stringify(prev);
          const newStr = JSON.stringify(dls);
          return prevStr === newStr ? prev : dls;
        });
      } catch (e) {
        console.error(e);
      }
    }, 2500);

    return () => clearInterval(interval);
  }, [downloadJobs]);

  const [isGameRunning, setIsGameRunning] = useState(false);

  // Poll GTA San Andreas process status every 4s (only when window is visible)
  useEffect(() => {
    if (!activeGame?.id) {
      setIsGameRunning(false);
      return;
    }

    const gameId = activeGame.id;
    let active = true;

    const checkProcess = async () => {
      if (typeof document !== 'undefined' && document.hidden) return;
      try {
        const running = await api.isGameProcessRunning(gameId);
        if (active) {
          setIsGameRunning((prev) => (prev !== running ? running : prev));
        }
      } catch {
        // ignore
      }
    };

    checkProcess();
    const interval = setInterval(checkProcess, 4000);
    return () => {
      active = false;
      clearInterval(interval);
    };
  }, [activeGame?.id]);

  const handleKillGame = async () => {
    if (!activeGame) return;
    try {
      await api.killGameProcess(activeGame.id);
      setIsGameRunning(false);
      showToast('GTA: San Andreas process terminated.');
    } catch (e: any) {
      showToast(`Failed to kill process: ${e?.message || e}`, 'error');
    }
  };

  const handleOpenGameFolder = async () => {
    if (!activeGame) return;
    try {
      await api.openPathInExplorer(activeGame.rootPath);
    } catch (e: any) {
      showToast(`Failed to open folder: ${e?.message || e}`, 'error');
    }
  };

  const activeGameRef = useRef<GameInstallation | null>(null);
  activeGameRef.current = activeGame;

  const handleInspectArchiveDirectly = useCallback(async (archivePath: string) => {
    const currentActiveGame = activeGameRef.current;
    if (!currentActiveGame) {
      showToast('Please select or locate your GTA: San Andreas installation folder first!', 'error');
      setCurrentTab('installations');
      return;
    }

    const modTitle = archivePath.split(/[\\/]/).pop()?.replace(/\.[^.]+$/, '') || 'Mod Archive';
    const modSlug = modTitle.toLowerCase().replace(/[^a-z0-9_-]/g, '_');

    try {
      showToast(`Inspecting archive: ${archivePath.split(/[\\/]/).pop()}...`);
      const plan = await api.generateInstallationPlan(
        archivePath,
        currentActiveGame.id,
        `local_archive_${modSlug}_${Date.now()}`,
        modTitle
      );
      setActivePlan(plan);
      setActivePlanArchivePath(archivePath);
    } catch (e: any) {
      showToast(`Inspection failed: ${e?.message || e}`, 'error');
    }
  }, []);

  // Tauri 2 Window Drag and Drop Archive listener (registered once on mount)
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let isSubscribed = true;

    async function setupDragDrop() {
      try {
        const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow');
        const fn = await getCurrentWebviewWindow().onDragDropEvent((event) => {
          if (event.payload.type === 'drop') {
            const paths = event.payload.paths;
            if (paths && paths.length > 0) {
              const droppedPath = paths[0];
              const lower = droppedPath.toLowerCase();
              if (lower.endsWith('.zip') || lower.endsWith('.rar') || lower.endsWith('.7z') || lower.endsWith('.asi') || lower.endsWith('.cs') || lower.endsWith('.cleo')) {
                handleInspectArchiveDirectly(droppedPath);
              }
            }
          }
        });
        if (isSubscribed) {
          unlisten = fn;
        } else {
          fn();
        }
      } catch (e) {
        console.warn('Drag-and-drop listener setup:', e);
      }
    }

    setupDragDrop();
    return () => {
      isSubscribed = false;
      if (unlisten) unlisten();
    };
  }, [handleInspectArchiveDirectly]);

  // Global F5 and Escape listeners
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'F5') {
        e.preventDefault();
        loadData();
        showToast('Application data refreshed.');
      } else if (e.key === 'Escape') {
        setSelectedModForDetails(null);
        setInspectingManifestMod(null);
        setShowArchiveImportModal(false);
        setActivePlan(null);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  const handleSelectActiveGame = async (id: string) => {
    try {
      await api.setActiveInstallation(id);
      await loadData();
      showToast('Game installation activated.');
    } catch (e: any) {
      showToast(e?.toString() || 'Failed to activate installation', 'error');
    }
  };

  const handleLaunchGame = async (customArgs: string[] = []) => {
    if (!activeGame) return;
    try {
      const pid = await api.launchGame(activeGame.id, customArgs);
      setIsGameRunning(true);
      showToast(`GTA: San Andreas launched successfully (PID: ${pid})`);
    } catch (e: any) {
      showToast(`Launch failed: ${e?.toString()}`, 'error');
    }
  };

  const handleRestartGame = async (customArgs: string[] = ['-novid']) => {
    if (!activeGame) return;
    try {
      showToast('Restarting GTA: San Andreas to apply mod changes...');
      await api.killGameProcess(activeGame.id);
      setIsGameRunning(false);
      // Brief pause to allow OS file handles and memory to release cleanly
      await new Promise((r) => setTimeout(r, 600));
      const pid = await api.launchGame(activeGame.id, customArgs);
      setIsGameRunning(true);
      showToast(`GTA: San Andreas restarted successfully (PID: ${pid})`);
    } catch (e: any) {
      showToast(`Restart failed: ${e?.message || e}`, 'error');
    }
  };

  const handleToggleMod = async (modId: string, enabled: boolean) => {
    if (!activeGame) return;
    try {
      await api.toggleModState(modId, enabled, activeGame.id);
      await loadData();
      if (isGameRunning) {
        showToast(
          `Mod ${enabled ? 'enabled' : 'disabled'} on disk. Restart game to apply changes.`,
          'success'
        );
      } else {
        showToast(`Mod ${enabled ? 'Enabled' : 'Disabled'}`);
      }
    } catch (e: any) {
      showToast(e?.toString() || 'Failed to toggle mod', 'error');
    }
  };

  const handleUninstallMod = async (modId: string) => {
    if (!activeGame) return;
    try {
      await api.uninstallMod(modId, activeGame.id);
      await loadData();
      if (isGameRunning) {
        showToast(
          'Mod uninstalled. Restart game to remove active files from memory.',
          'success'
        );
      } else {
        showToast('Mod uninstalled and original files restored successfully.');
      }
    } catch (e: any) {
      showToast(e?.toString() || 'Uninstall failed', 'error');
    }
  };

  const handleStartDownload = async (mod: ModRecord) => {
    if (!mod.downloadUrl) {
      showToast('Direct download not supported by source website.', 'error');
      return;
    }

    try {
      const job = await api.startDownload(
        mod.id,
        mod.title,
        mod.sourceId,
        mod.sourceId,
        mod.downloadUrl
      );
      showToast(`Download started: ${mod.title}`);
      setSelectedModForDetails(null);
      setCurrentTab('downloads');
      setDownloadJobs((prev) => [job, ...prev]);
    } catch (e: any) {
      showToast(e?.toString() || 'Failed to start download', 'error');
    }
  };

  const handleOneClickInstall = async (mod: ModRecord) => {
    if (!activeGame) {
      showToast('Please select or locate your GTA: San Andreas installation folder first!', 'error');
      setCurrentTab('installations');
      return;
    }

    if (!mod.downloadUrl) {
      showToast('Direct download not supported for this source repository.', 'error');
      return;
    }

    cancelledModIdsRef.current.delete(mod.id);

    // Step 1: Initialize status
    setAutoInstallStates((prev) => ({
      ...prev,
      [mod.id]: { step: 'downloading', progress: 5, message: 'Initiating download...' },
    }));

    try {
      const job = await api.startDownload(
        mod.id,
        mod.title,
        mod.sourceId,
        mod.sourceId,
        mod.downloadUrl
      );

      activeOneClickJobIdsRef.current.set(mod.id, job.id);
      setDownloadJobs((prev) => [job, ...prev]);

      // Poll until download finishes
      const startTime = Date.now();
      let completedJob: DownloadJob | null = null;

      while (Date.now() - startTime < 180000) {
        if (cancelledModIdsRef.current.has(mod.id)) {
          throw new Error('Cancelled by user');
        }

        await new Promise((r) => setTimeout(r, 600));

        if (cancelledModIdsRef.current.has(mod.id)) {
          throw new Error('Cancelled by user');
        }

        const jobs = await api.getDownloadJobs();
        setDownloadJobs(jobs);
        const current = jobs.find((j) => j.id === job.id);

        if (current) {
          if (current.status === 'completed') {
            completedJob = current;
            break;
          } else if (current.status === 'failed') {
            throw new Error(current.errorMessage || 'Download failed');
          } else if (current.totalBytes > 0) {
            const pct = Math.min(99, Math.round((current.downloadedBytes / current.totalBytes) * 100));
            setAutoInstallStates((prev) => ({
              ...prev,
              [mod.id]: {
                step: 'downloading',
                progress: pct,
                message: `Downloading (${pct}%)...`,
              },
            }));
          }
        }
      }

      if (!completedJob) {
        throw new Error('Download timed out');
      }

      if (cancelledModIdsRef.current.has(mod.id)) {
        throw new Error('Cancelled by user');
      }

      // Step 2: Planning & Conflict Analysis & Backup
      setAutoInstallStates((prev) => ({
        ...prev,
        [mod.id]: { step: 'planning', progress: 100, message: 'Inspecting archive & backing up files...' },
      }));

      const plan = await api.generateInstallationPlan(
        completedJob.tempFilePath,
        activeGame.id,
        mod.id,
        mod.title
      );

      if (cancelledModIdsRef.current.has(mod.id)) {
        throw new Error('Cancelled by user');
      }

      // Step 3: Transactional Deployment into GTA SA
      setAutoInstallStates((prev) => ({
        ...prev,
        [mod.id]: { step: 'installing', progress: 100, message: 'Deploying into GTA: San Andreas...' },
      }));

      const profileId = activeProfile?.id || 'default_profile';
      await api.installMod(
        completedJob.tempFilePath,
        plan,
        activeGame.id,
        profileId
      );

      activeOneClickJobIdsRef.current.delete(mod.id);

      // Step 4: Completed
      setAutoInstallStates((prev) => ({
        ...prev,
        [mod.id]: { step: 'completed', progress: 100, message: 'Installed successfully!' },
      }));

      showToast(`Installed '${mod.title}' successfully.`, 'success');
      await loadData();
    } catch (e: any) {
      const msg = typeof e === 'string' ? e : e?.message || 'Auto-install failed';
      activeOneClickJobIdsRef.current.delete(mod.id);

      if (cancelledModIdsRef.current.has(mod.id) || msg.includes('Cancelled by user')) {
        cancelledModIdsRef.current.delete(mod.id);
        setAutoInstallStates((prev) => {
          const next = { ...prev };
          delete next[mod.id];
          return next;
        });
        return;
      }

      setAutoInstallStates((prev) => ({
        ...prev,
        [mod.id]: { step: 'failed', error: msg },
      }));
      showToast(`Install error: ${msg}`, 'error');
    }
  };

  const handleCancelOneClickInstall = async (modId: string) => {
    cancelledModIdsRef.current.add(modId);
    setAutoInstallStates((prev) => {
      const next = { ...prev };
      delete next[modId];
      return next;
    });

    const jobId = activeOneClickJobIdsRef.current.get(modId);
    if (jobId) {
      try {
        await api.cancelDownload(jobId);
        const updated = await api.getDownloadJobs();
        setDownloadJobs(updated);
      } catch (err) {
        console.error('Failed to cancel download:', err);
      }
      activeOneClickJobIdsRef.current.delete(modId);
    }
    showToast('Cancelled mod loading and download.', 'success');
  };

  const handleCancelDownload = async (jobId: string) => {
    try {
      await api.cancelDownload(jobId);
      const jobs = await api.getDownloadJobs();
      setDownloadJobs(jobs);
      showToast('Download cancelled successfully.', 'success');
    } catch (e: any) {
      showToast(typeof e === 'string' ? e : e?.message || 'Failed to cancel download', 'error');
    }
  };

  const handleDeleteDownload = async (jobId: string) => {
    try {
      await api.deleteDownload(jobId);
      const jobs = await api.getDownloadJobs();
      setDownloadJobs(jobs);
      showToast('Removed from download history.', 'success');
    } catch (e: any) {
      showToast(typeof e === 'string' ? e : e?.message || 'Failed to delete download', 'error');
    }
  };

  const handleClearCompletedDownloads = async () => {
    try {
      await api.clearCompletedDownloads();
      const jobs = await api.getDownloadJobs();
      setDownloadJobs(jobs);
      showToast('Cleared finished and failed downloads.', 'success');
    } catch (e: any) {
      showToast(typeof e === 'string' ? e : e?.message || 'Failed to clear downloads', 'error');
    }
  };

  const handleRetryDownload = async (job: DownloadJob) => {
    try {
      await api.deleteDownload(job.id);
      const newJob = await api.startDownload(
        job.modId,
        job.modTitle,
        job.sourceId,
        job.sourceName,
        job.url,
        job.expectedSha256 || undefined
      );
      setDownloadJobs((prev) => [newJob, ...prev.filter((j) => j.id !== job.id)]);
      showToast(`Retrying download for '${job.modTitle}'...`, 'success');
    } catch (e: any) {
      showToast(typeof e === 'string' ? e : e?.message || 'Failed to retry download', 'error');
    }
  };

  const handlePlanReadyFromImport = async (archivePath: string, modTitle: string) => {
    if (!activeGame) {
      showToast('Please locate and activate a game installation first.', 'error');
      return;
    }

    try {
      const plan = await api.generateInstallationPlan(
        archivePath,
        activeGame.id,
        `mod_${Date.now()}`,
        modTitle
      );
      setActivePlan(plan);
      setActivePlanArchivePath(archivePath);
      setShowArchiveImportModal(false);
    } catch (e: any) {
      showToast(e?.toString() || 'Failed to generate installation plan', 'error');
    }
  };

  const handleInstallDownloadedJob = async (job: DownloadJob) => {
    if (!activeGame) {
      showToast('Please activate a game installation first.', 'error');
      return;
    }

    try {
      const plan = await api.generateInstallationPlan(
        job.tempFilePath,
        activeGame.id,
        job.modId,
        job.modTitle
      );
      setActivePlan(plan);
      setActivePlanArchivePath(job.tempFilePath);
    } catch (e: any) {
      showToast(e?.toString() || 'Failed to generate plan for downloaded file', 'error');
    }
  };

  const handleRescanExistingMods = async () => {
    if (!activeGame) {
      showToast('No active game selected to scan.', 'error');
      return;
    }
    try {
      const synced = await api.syncExistingMods(activeGame.id);
      await loadData();
      showToast(`Detected and synchronized ${synced.length} mods from game directory!`);
    } catch (e: any) {
      showToast(e?.toString() || 'Failed to scan existing mods', 'error');
    }
  };

  const handleConfirmInstall = async () => {
    if (!activePlan || !activePlanArchivePath || !activeGame) return;
    setIsInstalling(true);

    const profileId = activeProfile?.id || 'default_profile';

    try {
      await api.installMod(
        activePlanArchivePath,
        activePlan,
        activeGame.id,
        profileId
      );
      showToast(`Successfully installed '${activePlan.modTitle}' with backup protection!`);
      setActivePlan(null);
      setActivePlanArchivePath('');
      await loadData();
      setCurrentTab('installed');
    } catch (e: any) {
      showToast(e?.toString() || 'Installation failed. Rollback executed.', 'error');
    } finally {
      setIsInstalling(false);
    }
  };

  return (
    <div className="flex h-screen bg-bg-main text-neutral-200 select-none overflow-hidden font-sans">
      {/* Toast Notification */}
      {globalMessage && (
        <div
          className={`fixed bottom-5 right-5 z-50 px-4 py-2.5 rounded shadow-xl text-xs font-semibold border flex items-center gap-2 animate-in fade-in slide-in-from-bottom-2 duration-200 ${
            globalMessage.type === 'error'
              ? 'bg-rose-950 text-rose-200 border-rose-800'
              : 'bg-emerald-950 text-emerald-200 border-emerald-800'
          }`}
        >
          <span>{globalMessage.text}</span>
        </div>
      )}

      {/* Main Sidebar */}
      <Sidebar
        currentTab={currentTab}
        onSelectTab={setCurrentTab}
        installedCount={installedMods.length}
        downloadCount={downloadJobs.filter((j) => j.status === 'downloading').length}
        conflictCount={conflicts.length}
      />

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0 overflow-hidden">
        <Header
          activeGame={activeGame}
          activeProfile={activeProfile}
          isGameRunning={isGameRunning}
          onLaunchGame={() => handleLaunchGame(['-novid'])}
          onKillGame={handleKillGame}
          onRestartGame={() => handleRestartGame(['-novid'])}
          onOpenGameFolder={handleOpenGameFolder}
          onImportArchive={() => setShowArchiveImportModal(true)}
          onSelectGameClick={() => setCurrentTab('installations')}
        />

        {/* Global Game Running Notice Banner */}
        {isGameRunning && (
          <div className="bg-amber-950/70 border-b border-amber-800/80 px-4 md:px-6 py-2 flex items-center justify-between text-xs text-amber-200 flex-wrap gap-2 animate-in fade-in select-none">
            <div className="flex items-center gap-2">
              <span className="relative flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-amber-400"></span>
              </span>
              <span>
                <strong>GTA: San Andreas is currently running.</strong> Changes to installed mods will take effect after restarting the game.
              </span>
            </div>
            <div className="flex items-center gap-2 flex-shrink-0">
              <button
                onClick={() => handleRestartGame()}
                className="px-2.5 py-1 rounded bg-amber-500 hover:bg-amber-400 text-black font-bold text-xs flex items-center gap-1 shadow-sm transition-colors cursor-pointer"
                title="Restart game now to reload all modified files into memory"
              >
                <RotateCcw className="w-3 h-3" />
                <span>Restart Game</span>
              </button>
              <button
                onClick={handleKillGame}
                className="px-2.5 py-1 rounded bg-neutral-800 hover:bg-neutral-700 text-neutral-300 text-xs border border-neutral-700 transition-colors cursor-pointer"
                title="Terminate game process"
              >
                <span>Exit Game</span>
              </button>
            </div>
          </div>
        )}

        <main className="flex-1 overflow-y-auto">
          {currentTab === 'home' && (
            <Home
              activeGame={activeGame}
              activeProfile={activeProfile}
              installedMods={installedMods}
              conflictsCount={conflicts.length}
              backupsCount={backups.length}
              isGameRunning={isGameRunning}
              onNavigate={setCurrentTab}
              onLaunchGame={handleLaunchGame}
              onKillGame={handleKillGame}
              onToast={showToast}
            />
          )}

          {currentTab === 'discover' && (
            <DiscoverMods
              sources={sources}
              installedMods={installedMods}
              autoInstallStates={autoInstallStates}
              onSelectMod={setSelectedModForDetails}
              onOneClickInstall={handleOneClickInstall}
              onCancelOneClickInstall={handleCancelOneClickInstall}
              onImportArchive={() => setShowArchiveImportModal(true)}
            />
          )}

          {currentTab === 'installed' && (
            <InstalledMods
              mods={installedMods}
              activeGame={activeGame}
              onToggleState={handleToggleMod}
              onUninstall={handleUninstallMod}
              onInspectFiles={setInspectingManifestMod}
              onRescanFolder={handleRescanExistingMods}
              onToast={showToast}
            />
          )}

          {currentTab === 'downloads' && (
            <Downloads
              jobs={downloadJobs}
              onInstallDownloaded={handleInstallDownloadedJob}
              onRefresh={loadData}
              onCancelDownload={handleCancelDownload}
              onDeleteDownload={handleDeleteDownload}
              onClearCompleted={handleClearCompletedDownloads}
              onRetryDownload={handleRetryDownload}
            />
          )}

          {currentTab === 'profiles' && (
            <Profiles
              profiles={profiles}
              onSetActiveProfile={async (pid) => {
                if (activeGame) {
                  await api.setActiveProfile(pid, activeGame.id);
                  await loadData();
                  showToast('Active profile switched.');
                }
              }}
              onCreateProfile={async (name, desc) => {
                if (activeGame) {
                  await api.createProfile(activeGame.id, name, desc);
                  await loadData();
                  showToast('Profile created.');
                }
              }}
              onDeleteProfile={async (pid) => {
                await api.deleteProfile(pid);
                await loadData();
                showToast('Profile deleted.');
              }}
              onExportProfile={async (pid) => {
                if (activeGame) {
                  const json = await api.exportProfile(pid, activeGame.id);
                  navigator.clipboard.writeText(json);
                  showToast('Profile manifest copied to clipboard.');
                }
              }}
              onImportProfile={async (json) => {
                if (activeGame) {
                  await api.importProfile(json, activeGame.id);
                  await loadData();
                  showToast('Profile imported successfully.');
                }
              }}
            />
          )}

          {currentTab === 'installations' && (
            <GameInstallations
              installations={installations}
              onSelectActive={handleSelectActiveGame}
              onRefresh={loadData}
              onNavigateToInstalled={() => setCurrentTab('installed')}
            />
          )}

          {currentTab === 'conflicts' && (
            <Conflicts
              conflicts={conflicts}
              activeGame={activeGame}
              onRefresh={loadData}
              onToast={showToast}
            />
          )}

          {currentTab === 'backups' && (
            <Backups backups={backups} onRefresh={loadData} />
          )}

          {currentTab === 'diagnostics' && <Diagnostics />}

          {currentTab === 'settings' && <Settings />}
        </main>

        {/* Professional High-Density Status Bar (MO2 / IDE Style) */}
        <footer className="h-7 bg-bg-card border-t border-border-subtle px-4 flex items-center justify-between text-[11px] text-neutral-400 select-none flex-shrink-0">
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-1.5">
              <span className={`w-2 h-2 rounded-full ${activeGame ? 'bg-emerald-400' : 'bg-amber-400'}`} />
              <span className="font-semibold text-neutral-300">
                {activeGame ? `${activeGame.name} (${activeGame.isClassic10 ? 'Classic 1.0 US Verified' : activeGame.detectedEdition})` : 'No Game Configured'}
              </span>
              {activeGame && (
                <span className="font-mono text-neutral-500 truncate max-w-sm text-[10px]">
                  {activeGame.rootPath}
                </span>
              )}
            </div>
          </div>

          <div className="flex items-center gap-4">
            <div className="flex items-center gap-1.5 text-neutral-400">
              <span className="text-[10px] uppercase font-mono text-neutral-500">Profile:</span>
              <span className="text-neutral-300 font-semibold">{activeProfile?.name || 'Default'}</span>
            </div>

            <div className="flex items-center gap-1.5 text-neutral-400">
              <span className="text-[10px] uppercase font-mono text-neutral-500">Mods:</span>
              <span className="text-amber-400 font-mono font-bold">{installedMods.filter((m) => m.enabled).length}</span>
              <span className="text-neutral-500">/</span>
              <span className="text-neutral-300 font-mono">{installedMods.length}</span>
            </div>

            <div className="flex items-center gap-1 text-emerald-400 font-medium text-[10px]">
              <span>Auto-Backup & Rollback Ready</span>
            </div>
          </div>
        </footer>
      </div>

      {/* Mod Details Modal */}
      {selectedModForDetails && (
        <ModDetailsModal
          mod={selectedModForDetails}
          onClose={() => setSelectedModForDetails(null)}
          onDownload={handleStartDownload}
          onOneClickInstall={handleOneClickInstall}
        />
      )}

      {/* File Manifest Modal */}
      {inspectingManifestMod && (
        <FileManifestModal
          mod={inspectingManifestMod}
          onClose={() => setInspectingManifestMod(null)}
        />
      )}

      {/* Archive Import Modal */}
      {showArchiveImportModal && (
        <ArchiveImportModal
          onClose={() => setShowArchiveImportModal(false)}
          onPlanReady={handlePlanReadyFromImport}
        />
      )}

      {/* Pre-installation Plan Preview Modal */}
      {activePlan && (
        <PlanPreviewModal
          plan={activePlan}
          archivePath={activePlanArchivePath}
          onConfirm={handleConfirmInstall}
          onCancel={() => {
            setActivePlan(null);
            setActivePlanArchivePath('');
          }}
          isInstalling={isInstalling}
        />
      )}
    </div>
  );
};
