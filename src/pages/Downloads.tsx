import React from 'react';
import { DownloadJob } from '../types';
import {
  DownloadCloud,
  CheckCircle2,
  AlertCircle,
  RefreshCw,
  FolderOpen,
  Play,
  ShieldCheck,
} from 'lucide-react';

interface DownloadsProps {
  jobs: DownloadJob[];
  onInstallDownloaded: (job: DownloadJob) => void;
  onRefresh: () => void;
}

export const Downloads: React.FC<DownloadsProps> = ({
  jobs,
  onInstallDownloaded,
  onRefresh,
}) => {
  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-lg font-bold text-white">Download Manager</h1>
          <p className="text-xs text-neutral-400">
            Real-time download queues, resume capabilities, and cryptographic SHA-256 integrity verification.
          </p>
        </div>

        <button
          onClick={onRefresh}
          className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1.5 transition-colors"
        >
          <RefreshCw className="w-3.5 h-3.5" />
          <span>Refresh</span>
        </button>
      </div>

      {jobs.length === 0 ? (
        <div className="bg-bg-card border border-border rounded-lg p-12 text-center text-xs text-neutral-400 space-y-2">
          <DownloadCloud className="w-8 h-8 text-neutral-500 mx-auto" />
          <div className="text-sm font-semibold text-neutral-300">No Active or Recent Downloads</div>
          <div>Download mods from the Discover tab or import a local archive.</div>
        </div>
      ) : (
        <div className="space-y-3">
          {jobs.map((job) => {
            const percent = job.totalBytes > 0
              ? Math.min(100, Math.round((job.downloadedBytes / job.totalBytes) * 100))
              : 0;

            const speedKb = (job.speedBytesPerSec / 1024).toFixed(1);
            const downloadedMb = (job.downloadedBytes / (1024 * 1024)).toFixed(2);
            const totalMb = (job.totalBytes / (1024 * 1024)).toFixed(2);

            return (
              <div
                key={job.id}
                className="bg-bg-card border border-border rounded-lg p-4 space-y-3"
              >
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-bold text-xs text-white flex items-center gap-2">
                      <span>{job.modTitle}</span>
                      <span className="text-[10px] text-neutral-400 font-normal">
                        ({job.sourceName})
                      </span>
                    </h3>
                    <div className="text-[11px] text-neutral-400 font-mono truncate max-w-md">
                      {job.url}
                    </div>
                  </div>

                  {/* Status Badge */}
                  <div>
                    {job.status === 'completed' && (
                      <span className="flex items-center gap-1 text-[11px] text-emerald-400 font-bold bg-emerald-950/60 border border-emerald-800 px-2 py-0.5 rounded">
                        <CheckCircle2 className="w-3.5 h-3.5" /> Completed
                      </span>
                    )}
                    {job.status === 'downloading' && (
                      <span className="flex items-center gap-1 text-[11px] text-accent font-bold bg-amber-950/60 border border-amber-800 px-2 py-0.5 rounded">
                        <RefreshCw className="w-3.5 h-3.5 animate-spin" /> Downloading ({speedKb} KB/s)
                      </span>
                    )}
                    {job.status === 'failed' && (
                      <span className="flex items-center gap-1 text-[11px] text-rose-400 font-bold bg-rose-950/60 border border-rose-800 px-2 py-0.5 rounded">
                        <AlertCircle className="w-3.5 h-3.5" /> Failed
                      </span>
                    )}
                  </div>
                </div>

                {/* Progress Bar */}
                {job.status === 'downloading' && (
                  <div className="space-y-1">
                    <div className="w-full bg-neutral-800 h-1.5 rounded-full overflow-hidden">
                      <div
                        className="bg-accent h-full transition-all duration-300"
                        style={{ width: `${percent}%` }}
                      ></div>
                    </div>
                    <div className="flex justify-between text-[10px] text-neutral-400 font-mono">
                      <span>{percent}%</span>
                      <span>{downloadedMb} MB / {totalMb} MB</span>
                    </div>
                  </div>
                )}

                {/* Verification & Actions */}
                <div className="pt-2 border-t border-border-subtle flex items-center justify-between text-xs">
                  <div className="text-[11px] font-mono text-neutral-400 flex items-center gap-2">
                    {job.computedSha256 && (
                      <span className="flex items-center gap-1 text-emerald-400">
                        <ShieldCheck className="w-3.5 h-3.5" />
                        SHA-256 Verified: {job.computedSha256.substring(0, 16)}...
                      </span>
                    )}
                    {job.errorMessage && (
                      <span className="text-rose-400">{job.errorMessage}</span>
                    )}
                  </div>

                  {job.status === 'completed' && (
                    <button
                      onClick={() => onInstallDownloaded(job)}
                      className="px-4 py-1.5 rounded bg-accent hover:bg-accent-hover text-bg-main text-xs font-bold transition-colors shadow"
                    >
                      Inspect & Install
                    </button>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
};
