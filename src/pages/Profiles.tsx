import React, { useState } from 'react';
import { Profile } from '../types';
import {
  Layers,
  PlusCircle,
  Download,
  Upload,
  CheckCircle2,
  Trash2,
  Check,
  X,
} from 'lucide-react';

interface ProfilesProps {
  profiles: Profile[];
  onSetActiveProfile: (profileId: string) => void;
  onCreateProfile: (name: string, description: string) => void;
  onDeleteProfile: (profileId: string) => void;
  onExportProfile: (profileId: string) => void;
  onImportProfile: (manifestJson: string) => void;
}

export const Profiles: React.FC<ProfilesProps> = ({
  profiles,
  onSetActiveProfile,
  onCreateProfile,
  onDeleteProfile,
  onExportProfile,
  onImportProfile,
}) => {
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showImportModal, setShowImportModal] = useState(false);
  const [newName, setNewName] = useState('');
  const [newDesc, setNewDesc] = useState('');
  const [importJson, setImportJson] = useState('');

  const handleCreate = () => {
    if (!newName.trim()) return;
    onCreateProfile(newName.trim(), newDesc.trim());
    setNewName('');
    setNewDesc('');
    setShowCreateModal(false);
  };

  const handleImport = () => {
    if (!importJson.trim()) return;
    onImportProfile(importJson.trim());
    setImportJson('');
    setShowImportModal(false);
  };

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h1 className="text-lg font-bold text-white">Mod Profiles</h1>
          <p className="text-xs text-neutral-400">
            Maintain independent mod configurations for the same game installation (e.g. Vanilla Baseline, Visual Enhancements, Total Conversion).
          </p>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => setShowImportModal(true)}
            className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1.5 transition-colors"
          >
            <Upload className="w-3.5 h-3.5" />
            <span>Import Profile</span>
          </button>

          <button
            onClick={() => setShowCreateModal(true)}
            className="px-4 py-1.5 rounded bg-accent hover:bg-accent-hover text-bg-main text-xs font-bold flex items-center gap-1.5 transition-colors shadow"
          >
            <PlusCircle className="w-4 h-4" />
            <span>New Profile</span>
          </button>
        </div>
      </div>

      {/* Profiles Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {profiles.map((profile) => (
          <div
            key={profile.id}
            className={`bg-bg-card border rounded-lg p-5 flex flex-col justify-between space-y-4 transition-colors ${
              profile.isActive
                ? 'border-accent shadow-sm'
                : 'border-border hover:border-neutral-700'
            }`}
          >
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <h3 className="font-bold text-sm text-white">{profile.name}</h3>
                  {profile.isDefault && (
                    <span className="text-[10px] font-mono px-1.5 py-0.2 rounded bg-neutral-800 text-neutral-400 border border-neutral-700 font-semibold">
                      DEFAULT
                    </span>
                  )}
                </div>

                {profile.isActive ? (
                  <span className="flex items-center gap-1 text-[11px] font-bold text-emerald-400 bg-emerald-950/60 border border-emerald-800 px-2 py-0.5 rounded">
                    <CheckCircle2 className="w-3.5 h-3.5" /> Active Profile
                  </span>
                ) : (
                  <button
                    onClick={() => onSetActiveProfile(profile.id)}
                    className="px-3 py-1 bg-neutral-800 hover:bg-neutral-700 text-xs font-medium text-neutral-300 rounded transition-colors"
                  >
                    Activate
                  </button>
                )}
              </div>

              <p className="text-xs text-neutral-300 leading-relaxed">
                {profile.description || 'Custom mod configuration profile.'}
              </p>

              <div className="text-[11px] text-neutral-400 font-mono">
                Created: {new Date(profile.createdAt).toLocaleDateString()}
              </div>
            </div>

            {/* Actions */}
            <div className="pt-3 border-t border-border-subtle flex items-center justify-between">
              <button
                onClick={() => onExportProfile(profile.id)}
                className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1.5 transition-colors"
              >
                <Download className="w-3.5 h-3.5" />
                <span>Export JSON Manifest</span>
              </button>

              {!profile.isDefault && (
                <button
                  onClick={() => onDeleteProfile(profile.id)}
                  className="p-1.5 text-neutral-400 hover:text-rose-400 rounded transition-colors"
                  title="Delete Profile"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* Create Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
          <div className="bg-bg-subtle border border-border rounded-lg shadow-2xl max-w-md w-full p-5 space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-sm font-bold text-white flex items-center gap-2">
                <Layers className="w-4 h-4 text-accent" />
                Create New Mod Profile
              </h3>
              <button onClick={() => setShowCreateModal(false)} className="text-neutral-400 hover:text-white">
                <X className="w-4 h-4" />
              </button>
            </div>

            <div className="space-y-3">
              <div>
                <label className="block text-xs font-semibold text-neutral-300 mb-1">
                  Profile Name
                </label>
                <input
                  type="text"
                  value={newName}
                  onChange={(e) => setNewName(e.target.value)}
                  placeholder="e.g. Visual Overhaul, Speedrun Setup"
                  className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent"
                />
              </div>

              <div>
                <label className="block text-xs font-semibold text-neutral-300 mb-1">
                  Description
                </label>
                <textarea
                  value={newDesc}
                  onChange={(e) => setNewDesc(e.target.value)}
                  placeholder="Notes about which mods belong in this configuration..."
                  rows={3}
                  className="w-full bg-bg-main border border-border rounded px-3 py-1.5 text-xs text-white focus:outline-none focus:border-accent resize-none"
                />
              </div>
            </div>

            <div className="flex justify-end gap-2 pt-2">
              <button
                onClick={() => setShowCreateModal(false)}
                className="px-3 py-1.5 text-xs text-neutral-400 hover:text-white"
              >
                Cancel
              </button>
              <button
                onClick={handleCreate}
                disabled={!newName.trim()}
                className="px-4 py-1.5 bg-accent hover:bg-accent-hover text-bg-main font-bold text-xs rounded transition-colors shadow"
              >
                Create Profile
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Import Modal */}
      {showImportModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
          <div className="bg-bg-subtle border border-border rounded-lg shadow-2xl max-w-lg w-full p-5 space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-sm font-bold text-white flex items-center gap-2">
                <Upload className="w-4 h-4 text-accent" />
                Import Profile Manifest JSON
              </h3>
              <button onClick={() => setShowImportModal(false)} className="text-neutral-400 hover:text-white">
                <X className="w-4 h-4" />
              </button>
            </div>

            <div>
              <label className="block text-xs font-semibold text-neutral-300 mb-1">
                Paste JSON Manifest
              </label>
              <textarea
                value={importJson}
                onChange={(e) => setImportJson(e.target.value)}
                placeholder='{"formatVersion": "1.0", "profileName": "...", "mods": [...]}'
                rows={8}
                className="w-full bg-bg-main border border-border rounded p-3 text-xs text-white font-mono focus:outline-none focus:border-accent"
              />
            </div>

            <div className="flex justify-end gap-2 pt-2">
              <button
                onClick={() => setShowImportModal(false)}
                className="px-3 py-1.5 text-xs text-neutral-400 hover:text-white"
              >
                Cancel
              </button>
              <button
                onClick={handleImport}
                disabled={!importJson.trim()}
                className="px-4 py-1.5 bg-accent hover:bg-accent-hover text-bg-main font-bold text-xs rounded transition-colors shadow"
              >
                Import Profile
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
