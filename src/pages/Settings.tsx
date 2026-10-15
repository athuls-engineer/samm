import React, { useState, useEffect } from 'react';
import { AppSettings } from '../types';
import { api } from '../api/client';
import { Settings as SettingsIcon, Save, Check } from 'lucide-react';

export const Settings: React.FC = () => {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [saved, setSaved] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    api.getSettings().then(setSettings).catch(console.error);
  }, []);

  const handleSave = async () => {
    if (!settings) return;
    setLoading(true);
    try {
      await api.saveSettings(settings);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  if (!settings) return null;

  return (
    <div className="p-8 space-y-6 max-w-4xl mx-auto">
      <div>
        <h1 className="text-lg font-bold text-white">Application Settings</h1>
        <p className="text-xs text-neutral-400">
          Configure local paths, download concurrency limits, and filesystem backup repositories.
        </p>
      </div>

      <div className="bg-bg-card border border-border rounded-lg p-6 space-y-5">
        <div>
          <label className="block text-xs font-semibold text-neutral-300 mb-1">
            Downloads Storage Directory
          </label>
          <input
            type="text"
            value={settings.downloadDirectory}
            onChange={(e) => setSettings({ ...settings, downloadDirectory: e.target.value })}
            className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white font-mono focus:outline-none focus:border-accent"
          />
          <p className="text-[11px] text-neutral-500 mt-1">
            Temporary and finalized archive downloads are kept isolated outside the game directory.
          </p>
        </div>

        <div>
          <label className="block text-xs font-semibold text-neutral-300 mb-1">
            Backup Storage Directory
          </label>
          <input
            type="text"
            value={settings.backupDirectory}
            onChange={(e) => setSettings({ ...settings, backupDirectory: e.target.value })}
            className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white font-mono focus:outline-none focus:border-accent"
          />
          <p className="text-[11px] text-neutral-500 mt-1">
            Preserves original files replaced by mods for safe rollback.
          </p>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-xs font-semibold text-neutral-300 mb-1">
              Maximum Concurrent Downloads
            </label>
            <input
              type="number"
              min="1"
              max="10"
              value={settings.maxConcurrentDownloads}
              onChange={(e) =>
                setSettings({ ...settings, maxConcurrentDownloads: parseInt(e.target.value) || 3 })
              }
              className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent"
            />
          </div>

          <div>
            <label className="block text-xs font-semibold text-neutral-300 mb-1">
              Visual Theme
            </label>
            <select
              value={settings.theme}
              onChange={(e) => setSettings({ ...settings, theme: e.target.value as any })}
              className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent cursor-pointer"
            >
              <option value="dark">Charcoal Dark (Default)</option>
              <option value="light">Classic Light</option>
            </select>
          </div>
        </div>

        <div className="pt-2 border-t border-border-subtle flex items-center justify-between">
          <label className="flex items-center gap-2 cursor-pointer text-xs text-neutral-300">
            <input
              type="checkbox"
              checked={settings.requireInstallConfirmation}
              onChange={(e) =>
                setSettings({ ...settings, requireInstallConfirmation: e.target.checked })
              }
              className="accent-amber-500 rounded"
            />
            <span>Require pre-installation plan inspection before modifying files (Recommended)</span>
          </label>

          <button
            onClick={handleSave}
            disabled={loading}
            className="px-5 py-2 bg-accent hover:bg-accent-hover text-bg-main font-bold text-xs rounded flex items-center gap-1.5 transition-colors shadow"
          >
            {saved ? <Check className="w-4 h-4" /> : <Save className="w-4 h-4" />}
            <span>{saved ? 'Settings Saved' : 'Save Settings'}</span>
          </button>
        </div>
      </div>
    </div>
  );
};
