import React, { useState, useEffect } from 'react';
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
} from './types';
import { api } from './api/client';
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

  // Modals state
  const [selectedModForDetails, setSelectedModForDetails] = useState<ModRecord | null>(null);
  const [inspectingManifestMod, setInspectingManifestMod] = useState<InstalledMod | null>(null);
  const [showArchiveImportModal, setShowArchiveImportModal] = useState(false);
  const [activePlan, setActivePlan] = useState<InstallationPlan | null>(null);
  const [activePlanArchivePath, setActivePlanArchivePath] = useState<string>('');
  const [isInstalling, setIsInstalling] = useState(false);
  const [globalMessage, setGlobalMessage] = useState<{ text: string; type: 'success' | 'error' } | null>(null);

  const showToast = (text: string, type: 'success' | 'error' = 'success') => {
    setGlobalMessage({ text, type });
    setTimeout(() => setGlobalMessage(null), 4000);
  };

  const loadData = async () => {
    try {
      // 1. Installations
      const insts = await api.getInstallations();
      setInstallations(insts);
      const active = insts.find((i) => i.isActive) || insts[0] || null;
      setActiveGame(active);

      // 2. Sources
      const srcs = await api.listSources();
      setSources(srcs);

      // 3. Downloads
      const dls = await api.getDownloadJobs();
      setDownloadJobs(dls);

      if (active) {
        // 4. Profiles
        const profs = await api.getProfiles(active.id);
        setProfiles(profs);
        const activeProf = profs.find((p) => p.isActive) || profs[0] || null;
        setActiveProfile(activeProf);

        // 5. Installed Mods
        const mods = await api.getInstalledMods(active.id);
        setInstalledMods(mods);

        // 6. Conflicts
        const confs = await api.getConflicts(active.id);
        setConflicts(confs);

        // 7. Backups
        const bks = await api.getBackups(active.id);
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

  // Poll active downloads every 2 seconds if any are downloading
  useEffect(() => {
    const hasActiveDownloads = downloadJobs.some((j) => j.status === 'downloading');
    if (!hasActiveDownloads) return;

    const interval = setInterval(async () => {
      try {
        const dls = await api.getDownloadJobs();
        setDownloadJobs(dls);
      } catch (e) {
        console.error(e);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [downloadJobs]);

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
      showToast(`GTA: San Andreas launched successfully (PID: ${pid})`);
    } catch (e: any) {
      showToast(`Launch failed: ${e?.toString()}`, 'error');
    }
  };

  const handleToggleMod = async (modId: string, enabled: boolean) => {
    if (!activeGame) return;
    try {
      await api.toggleModState(modId, enabled, activeGame.id);
      await loadData();
      showToast(`Mod state updated: ${enabled ? 'Enabled' : 'Disabled'}`);
    } catch (e: any) {
      showToast(e?.toString() || 'Failed to toggle mod', 'error');
    }
  };

  const handleUninstallMod = async (modId: string) => {
    if (!activeGame) return;
    try {
      await api.uninstallMod(modId, activeGame.id);
      await loadData();
      showToast('Mod uninstalled and original backed-up files restored successfully.');
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
          onLaunchGame={() => handleLaunchGame(['-novid'])}
          onImportArchive={() => setShowArchiveImportModal(true)}
          onSelectGameClick={() => setCurrentTab('installations')}
        />

        <main className="flex-1 overflow-y-auto">
          {currentTab === 'home' && (
            <Home
              activeGame={activeGame}
              activeProfile={activeProfile}
              installedMods={installedMods}
              conflictsCount={conflicts.length}
              backupsCount={backups.length}
              onNavigate={setCurrentTab}
              onLaunchGame={handleLaunchGame}
            />
          )}

          {currentTab === 'discover' && (
            <DiscoverMods
              sources={sources}
              onSelectMod={setSelectedModForDetails}
              onDownloadMod={handleStartDownload}
            />
          )}

          {currentTab === 'installed' && (
            <InstalledMods
              mods={installedMods}
              onToggleState={handleToggleMod}
              onUninstall={handleUninstallMod}
              onInspectFiles={setInspectingManifestMod}
            />
          )}

          {currentTab === 'downloads' && (
            <Downloads
              jobs={downloadJobs}
              onInstallDownloaded={handleInstallDownloadedJob}
              onRefresh={loadData}
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
            />
          )}

          {currentTab === 'conflicts' && (
            <Conflicts conflicts={conflicts} onRefresh={loadData} />
          )}

          {currentTab === 'backups' && (
            <Backups backups={backups} onRefresh={loadData} />
          )}

          {currentTab === 'diagnostics' && <Diagnostics />}

          {currentTab === 'settings' && <Settings />}
        </main>
      </div>

      {/* Mod Details Modal */}
      {selectedModForDetails && (
        <ModDetailsModal
          mod={selectedModForDetails}
          onClose={() => setSelectedModForDetails(null)}
          onDownload={handleStartDownload}
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
