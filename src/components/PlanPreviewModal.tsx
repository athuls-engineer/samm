import React from 'react';
import { InstallationPlan } from '../types';
import { ShieldCheck, AlertTriangle, CheckCircle, FileCode, X } from 'lucide-react';

interface PlanPreviewModalProps {
  plan: InstallationPlan | null;
  archivePath: string;
  onConfirm: () => void;
  onCancel: () => void;
  isInstalling: boolean;
}

export const PlanPreviewModal: React.FC<PlanPreviewModalProps> = ({
  plan,
  archivePath,
  onConfirm,
  onCancel,
  isInstalling,
}) => {
  if (!plan) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="bg-bg-subtle border border-border rounded-lg shadow-2xl max-w-3xl w-full max-h-[85vh] flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-150">
        {/* Modal Header */}
        <div className="p-4 border-b border-border flex items-center justify-between bg-bg-card">
          <div className="flex items-center gap-2.5">
            <ShieldCheck className="w-5 h-5 text-accent" />
            <div>
              <h2 className="text-sm font-bold text-white">Pre-Installation Safety Inspection</h2>
              <p className="text-[11px] text-neutral-400">
                Review proposed file modifications before changes are applied to your game directory.
              </p>
            </div>
          </div>
          <button
            onClick={onCancel}
            disabled={isInstalling}
            className="text-neutral-400 hover:text-white p-1 rounded"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content Body */}
        <div className="p-5 overflow-y-auto space-y-4 flex-1">
          {/* Summary Strip */}
          <div className="grid grid-cols-4 gap-3">
            <div className="bg-bg-card border border-border p-3 rounded">
              <div className="text-[11px] text-neutral-400">Strategy</div>
              <div className="text-xs font-bold text-white uppercase mt-0.5">{plan.strategy}</div>
            </div>
            <div className="bg-bg-card border border-border p-3 rounded">
              <div className="text-[11px] text-neutral-400">Total Files</div>
              <div className="text-xs font-bold text-white mt-0.5">{plan.totalFiles} files</div>
            </div>
            <div className="bg-bg-card border border-border p-3 rounded">
              <div className="text-[11px] text-neutral-400">New Files</div>
              <div className="text-xs font-bold text-emerald-400 mt-0.5">{plan.newFilesCount} files</div>
            </div>
            <div className="bg-bg-card border border-border p-3 rounded">
              <div className="text-[11px] text-neutral-400">Files to Backup</div>
              <div className="text-xs font-bold text-amber-400 mt-0.5">{plan.overwriteFilesCount} files</div>
            </div>
          </div>

          {/* Missing Prerequisites Warning */}
          {plan.missingPrerequisites.length > 0 && (
            <div className="bg-amber-950/40 border border-amber-800/80 p-3.5 rounded text-xs text-amber-200 space-y-1">
              <div className="font-semibold flex items-center gap-1.5 text-amber-300">
                <AlertTriangle className="w-4 h-4" />
                Missing Prerequisites Detected
              </div>
              <ul className="list-disc list-inside space-y-0.5 text-[11px] pl-1">
                {plan.missingPrerequisites.map((p, idx) => (
                  <li key={idx}>{p}</li>
                ))}
              </ul>
            </div>
          )}

          {/* Warnings */}
          {plan.warnings.length > 0 && (
            <div className="bg-neutral-800 border border-neutral-700 p-3 rounded text-xs text-neutral-300 space-y-1">
              <div className="font-semibold text-neutral-200">Installation Notices</div>
              <ul className="list-disc list-inside text-[11px] text-neutral-400">
                {plan.warnings.map((w, idx) => (
                  <li key={idx}>{w}</li>
                ))}
              </ul>
            </div>
          )}

          {/* Operations File Table */}
          <div>
            <div className="text-xs font-semibold text-white mb-2 flex items-center justify-between">
              <span>Planned File Operations</span>
              <span className="text-[11px] text-neutral-400 font-normal">
                Archive: {archivePath.split(/[\\/]/).pop()}
              </span>
            </div>
            <div className="border border-border rounded overflow-hidden max-h-56 overflow-y-auto">
              <table className="w-full text-left text-[11px]">
                <thead className="bg-bg-card text-neutral-400 border-b border-border sticky top-0">
                  <tr>
                    <th className="py-2 px-3">Archive File</th>
                    <th className="py-2 px-3">Destination Path</th>
                    <th className="py-2 px-3">Action</th>
                    <th className="py-2 px-3">Backup</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-border-subtle bg-bg-main/50 font-mono">
                  {plan.fileOperations.map((op, i) => (
                    <tr key={i} className="hover:bg-bg-hover">
                      <td className="py-1.5 px-3 truncate max-w-[200px] text-neutral-300">
                        {op.archiveRelPath}
                      </td>
                      <td className="py-1.5 px-3 truncate max-w-[240px] text-neutral-200 font-semibold">
                        {op.destinationRelPath}
                      </td>
                      <td className="py-1.5 px-3 text-[10px]">
                        <span
                          className={`px-1.5 py-0.5 rounded font-bold ${
                            op.destinationState === 'overwrite'
                              ? 'bg-amber-950 text-amber-300 border border-amber-800'
                              : 'bg-emerald-950 text-emerald-300 border border-emerald-800'
                          }`}
                        >
                          {op.destinationState.toUpperCase()}
                        </span>
                      </td>
                      <td className="py-1.5 px-3 text-[10px] text-neutral-400">
                        {op.backupRequired ? 'Yes (Stored)' : 'None needed'}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Transaction Guarantee note */}
          <div className="p-3 bg-neutral-900 border border-neutral-800 rounded flex items-center gap-2 text-[11px] text-neutral-400">
            <CheckCircle className="w-4 h-4 text-emerald-400 flex-shrink-0" />
            <span>
              <strong>Transactional Recovery:</strong> All overwritten files will be archived before changes are committed. If any error occurs, automatic rollback restores the original files.
            </span>
          </div>
        </div>

        {/* Modal Footer */}
        <div className="p-4 border-t border-border bg-bg-card flex items-center justify-between">
          <button
            onClick={onCancel}
            disabled={isInstalling}
            className="px-4 py-1.5 rounded text-xs font-medium text-neutral-300 hover:text-white hover:bg-neutral-800 transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={onConfirm}
            disabled={isInstalling}
            className={`px-5 py-2 rounded text-xs font-bold transition-colors shadow ${
              isInstalling
                ? 'bg-neutral-700 text-neutral-400 cursor-not-allowed'
                : 'bg-accent hover:bg-accent-hover text-bg-main'
            }`}
          >
            {isInstalling ? 'Installing & Backing up...' : 'Confirm & Apply Installation'}
          </button>
        </div>
      </div>
    </div>
  );
};
