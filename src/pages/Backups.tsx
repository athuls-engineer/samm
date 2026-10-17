import React from 'react';
import { BackupRecord } from '../types';
import { History, ShieldCheck, CheckCircle2, RotateCcw } from 'lucide-react';

interface BackupsProps {
  backups: BackupRecord[];
  onRefresh: () => void;
}

export const Backups: React.FC<BackupsProps> = ({ backups, onRefresh }) => {
  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      <div>
        <h1 className="text-lg font-bold text-white">File Backups & History</h1>
        <p className="text-xs text-neutral-400">
          Original game files replaced by mods are stored safely and can be restored at any time.
        </p>
      </div>

      {backups.length === 0 ? (
        <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-2">
          <ShieldCheck className="w-8 h-8 text-emerald-400 mx-auto" />
          <div className="text-sm font-semibold text-neutral-200">No Backups Needed Yet</div>
          <div>All installed mods were newly placed files or no files have been overwritten yet.</div>
        </div>
      ) : (
        <div className="bg-bg-card border border-border rounded-lg overflow-hidden">
          <table className="w-full text-left text-xs">
            <thead className="bg-bg-subtle text-neutral-400 border-b border-border">
              <tr>
                <th className="py-2.5 px-4 font-semibold">Replaced Game File</th>
                <th className="py-2.5 px-4 font-semibold">Overwritten By Mod</th>
                <th className="py-2.5 px-4 font-semibold">Original Size</th>
                <th className="py-2.5 px-4 font-semibold">Original SHA-256</th>
                <th className="py-2.5 px-4 font-semibold">Backup Date</th>
                <th className="py-2.5 px-4 font-semibold">Status</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-border-subtle font-mono text-[11px]">
              {backups.map((b) => (
                <tr key={b.id} className="hover:bg-bg-hover">
                  <td className="py-2 px-4 font-semibold text-white truncate max-w-xs">
                    {b.destinationRelPath}
                  </td>
                  <td className="py-2 px-4 font-sans text-neutral-300">
                    {b.modName}
                  </td>
                  <td className="py-2 px-4 text-neutral-400">
                    {(b.originalSizeBytes / 1024).toFixed(1)} KB
                  </td>
                  <td className="py-2 px-4 text-neutral-400 truncate max-w-[120px]">
                    {b.originalSha256 || 'N/A'}
                  </td>
                  <td className="py-2 px-4 text-neutral-400 font-sans">
                    {new Date(b.timestamp).toLocaleString()}
                  </td>
                  <td className="py-2 px-4 font-sans">
                    {b.restored ? (
                      <span className="flex items-center gap-1 text-neutral-400 text-[10px]">
                        <RotateCcw className="w-3 h-3 text-neutral-500" /> Restored
                      </span>
                    ) : (
                      <span className="flex items-center gap-1 text-emerald-400 font-bold text-[10px]">
                        <CheckCircle2 className="w-3 h-3" /> Preserved
                      </span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};
