import React, { useState, useEffect } from 'react';
import { DiagnosticReport } from '../types';
import { api } from '../api/client';
import { Activity, Copy, Check, ShieldCheck, Database, RefreshCw } from 'lucide-react';

export const Diagnostics: React.FC = () => {
  const [report, setReport] = useState<DiagnosticReport | null>(null);
  const [copied, setCopied] = useState(false);
  const [loading, setLoading] = useState(false);

  const fetchReport = async () => {
    setLoading(true);
    try {
      const rep = await api.getDiagnosticReport();
      setReport(rep);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchReport();
  }, []);

  const handleCopy = () => {
    if (!report) return;
    const text = JSON.stringify(report, null, 2);
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="p-8 space-y-6 max-w-6xl mx-auto">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-lg font-bold text-white">Application Diagnostics & System Health</h1>
          <p className="text-xs text-neutral-400">
            System environment report, database integrity, and operational logs for open-source troubleshooting.
          </p>
        </div>

        <div className="flex gap-2">
          <button
            onClick={fetchReport}
            className="px-3 py-1.5 rounded bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-xs text-neutral-300 font-medium flex items-center gap-1.5 transition-colors"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${loading ? 'animate-spin' : ''}`} />
            <span>Refresh</span>
          </button>

          <button
            onClick={handleCopy}
            disabled={!report}
            className="px-4 py-1.5 rounded bg-accent hover:bg-accent-hover text-bg-main text-xs font-bold flex items-center gap-1.5 transition-colors shadow"
          >
            {copied ? <Check className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
            <span>{copied ? 'Copied Report' : 'Copy Diagnostics'}</span>
          </button>
        </div>
      </div>

      {report && (
        <div className="space-y-4">
          {/* System Specs */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
            <div className="bg-bg-card border border-border p-3.5 rounded">
              <span className="text-[11px] text-neutral-400">SAMM Version</span>
              <div className="text-xs font-bold text-white mt-0.5">v{report.appVersion}</div>
            </div>
            <div className="bg-bg-card border border-border p-3.5 rounded">
              <span className="text-[11px] text-neutral-400">Host OS</span>
              <div className="text-xs font-bold text-white mt-0.5 uppercase">{report.osVersion} 64-bit</div>
            </div>
            <div className="bg-bg-card border border-border p-3.5 rounded">
              <span className="text-[11px] text-neutral-400">Backend Toolchain</span>
              <div className="text-xs font-bold text-white mt-0.5 truncate">{report.rustVersion}</div>
            </div>
            <div className="bg-bg-card border border-border p-3.5 rounded">
              <span className="text-[11px] text-neutral-400">Database Engine</span>
              <div className="text-xs font-bold text-emerald-400 mt-0.5 flex items-center gap-1">
                <Database className="w-3.5 h-3.5" /> SQLite 3 (WAL)
              </div>
            </div>
          </div>

          {/* Active Installation Health */}
          <div className="bg-bg-card border border-border rounded-lg p-5 space-y-2">
            <h3 className="text-xs font-bold text-white uppercase tracking-wider">
              Selected Target Health
            </h3>
            {report.activeGame ? (
              <div className="text-xs space-y-1">
                <div className="text-neutral-200">
                  Installation: <strong className="text-white">{report.activeGame.name}</strong> ({report.activeGame.detectedEdition})
                </div>
                <div className="text-neutral-400 font-mono text-[11px]">
                  Root: {report.activeGame.rootPath}
                </div>
                <div className="text-neutral-400 font-mono text-[11px]">
                  Executable Checksum: {report.activeGame.executableSha256}
                </div>
              </div>
            ) : (
              <div className="text-xs text-amber-300">No active game installation selected.</div>
            )}
          </div>

          {/* Log Stream */}
          <div className="bg-bg-card border border-border rounded-lg p-5 space-y-2">
            <h3 className="text-xs font-bold text-white uppercase tracking-wider">
              Recent Application Logs
            </h3>
            <div className="bg-bg-main p-3 rounded font-mono text-[11px] text-neutral-300 space-y-1 border border-border-subtle max-h-56 overflow-y-auto">
              {report.recentLogs.map((line, i) => (
                <div key={i}>{line}</div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
