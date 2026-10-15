import React from 'react';
import { InstalledMod } from '../types';
import { Package, X, CheckCircle2 } from 'lucide-react';

interface FileManifestModalProps {
  mod: InstalledMod | null;
  onClose: () => void;
}

export const FileManifestModal: React.FC<FileManifestModalProps> = ({ mod, onClose }) => {
  if (!mod) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="bg-bg-subtle border border-border rounded-lg shadow-2xl max-w-2xl w-full max-h-[80vh] flex flex-col overflow-hidden animate-in fade-in duration-150">
        <div className="p-4 border-b border-border flex items-center justify-between bg-bg-card">
          <div className="flex items-center gap-2">
            <Package className="w-5 h-5 text-accent" />
            <div>
              <h2 className="text-xs font-bold text-white">{mod.name} - Installed Files Manifest</h2>
              <p className="text-[11px] text-neutral-400">
                Strategy: {mod.strategy} | Installed: {new Date(mod.installedAt).toLocaleString()}
              </p>
            </div>
          </div>
          <button onClick={onClose} className="text-neutral-400 hover:text-white p-1 rounded">
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-4 overflow-y-auto flex-1">
          <div className="border border-border rounded overflow-hidden">
            <table className="w-full text-left text-[11px]">
              <thead className="bg-bg-card text-neutral-400 border-b border-border">
                <tr>
                  <th className="py-2 px-3">Destination Path</th>
                  <th className="py-2 px-3">Size</th>
                  <th className="py-2 px-3">SHA-256 Checksum</th>
                  <th className="py-2 px-3">Backup</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border-subtle bg-bg-main/50 font-mono">
                {mod.files.map((f) => (
                  <tr key={f.id} className="hover:bg-bg-hover">
                    <td className="py-1.5 px-3 font-semibold text-neutral-200 truncate max-w-[220px]">
                      {f.destinationRelPath}
                    </td>
                    <td className="py-1.5 px-3 text-neutral-400">
                      {(f.fileSizeBytes / 1024).toFixed(1)} KB
                    </td>
                    <td className="py-1.5 px-3 text-[10px] text-neutral-400 truncate max-w-[150px]">
                      {f.sha256}
                    </td>
                    <td className="py-1.5 px-3 text-[10px]">
                      {f.hasBackup ? (
                        <span className="text-emerald-400 flex items-center gap-1 font-bold">
                          <CheckCircle2 className="w-3 h-3" /> Backed Up
                        </span>
                      ) : (
                        <span className="text-neutral-500">New File</span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        <div className="p-3 border-t border-border bg-bg-card flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-1.5 bg-neutral-800 hover:bg-neutral-700 text-xs font-medium text-white rounded transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
