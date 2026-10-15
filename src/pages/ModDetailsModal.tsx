import React, { useState } from 'react';
import { ModRecord } from '../types';
import {
  X,
  ExternalLink,
  Download,
  CheckCircle2,
  AlertTriangle,
  Info,
  Shield,
  Layers,
  FileText,
} from 'lucide-react';

interface ModDetailsModalProps {
  mod: ModRecord | null;
  onClose: () => void;
  onDownload: (mod: ModRecord) => void;
}

export const ModDetailsModal: React.FC<ModDetailsModalProps> = ({
  mod,
  onClose,
  onDownload,
}) => {
  if (!mod) return null;

  const [activeTab, setActiveTab] = useState<'overview' | 'requirements' | 'attribution'>('overview');

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="bg-bg-subtle border border-border rounded-lg shadow-2xl max-w-2xl w-full max-h-[85vh] flex flex-col overflow-hidden animate-in fade-in duration-150">
        {/* Header */}
        <div className="p-5 border-b border-border bg-bg-card flex items-start justify-between">
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-base font-bold text-white">{mod.title}</h2>
              <span className="text-[10px] font-mono uppercase px-2 py-0.5 rounded bg-neutral-800 text-neutral-300 font-bold border border-neutral-700">
                {mod.installationStrategy}
              </span>
            </div>
            <div className="text-xs text-neutral-400 mt-1">
              Author: <span className="text-neutral-200 font-semibold">{mod.author}</span> | Category: <span className="text-accent">{mod.category}</span>
            </div>
          </div>
          <button onClick={onClose} className="text-neutral-400 hover:text-white p-1 rounded">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Tab Navigation */}
        <div className="flex border-b border-border bg-bg-main/50 px-5 gap-6 text-xs font-semibold">
          <button
            onClick={() => setActiveTab('overview')}
            className={`py-2.5 border-b-2 transition-colors ${
              activeTab === 'overview'
                ? 'border-accent text-accent'
                : 'border-transparent text-neutral-400 hover:text-white'
            }`}
          >
            Overview
          </button>
          <button
            onClick={() => setActiveTab('requirements')}
            className={`py-2.5 border-b-2 transition-colors ${
              activeTab === 'requirements'
                ? 'border-accent text-accent'
                : 'border-transparent text-neutral-400 hover:text-white'
            }`}
          >
            Requirements & Compatibility
          </button>
          <button
            onClick={() => setActiveTab('attribution')}
            className={`py-2.5 border-b-2 transition-colors ${
              activeTab === 'attribution'
                ? 'border-accent text-accent'
                : 'border-transparent text-neutral-400 hover:text-white'
            }`}
          >
            Source & Legal Attribution
          </button>
        </div>

        {/* Tab Contents */}
        <div className="p-6 overflow-y-auto flex-1 space-y-4">
          {activeTab === 'overview' && (
            <div className="space-y-4">
              <div>
                <h3 className="text-xs font-semibold text-neutral-400 uppercase tracking-wider mb-1.5">
                  Description
                </h3>
                <p className="text-xs text-neutral-200 leading-relaxed bg-bg-card p-3 rounded border border-border">
                  {mod.description}
                </p>
              </div>

              <div className="grid grid-cols-2 gap-3 text-xs">
                <div className="bg-bg-card border border-border p-3 rounded">
                  <span className="text-[11px] text-neutral-400">File Size</span>
                  <div className="font-semibold text-white mt-0.5">{mod.fileSizeFormatted || 'Unknown'}</div>
                </div>
                <div className="bg-bg-card border border-border p-3 rounded">
                  <span className="text-[11px] text-neutral-400">Last Updated</span>
                  <div className="font-semibold text-white mt-0.5">{mod.updateDate || 'Historic Release'}</div>
                </div>
              </div>

              {mod.tags.length > 0 && (
                <div>
                  <h3 className="text-xs font-semibold text-neutral-400 uppercase tracking-wider mb-1.5">
                    Tags
                  </h3>
                  <div className="flex flex-wrap gap-1.5">
                    {mod.tags.map((t, idx) => (
                      <span key={idx} className="text-[11px] px-2 py-0.5 rounded bg-neutral-800 text-neutral-300 font-mono">
                        #{t}
                      </span>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === 'requirements' && (
            <div className="space-y-4">
              <div className="bg-bg-card border border-border p-4 rounded space-y-2">
                <div className="font-semibold text-xs text-white flex items-center gap-2">
                  <Shield className="w-4 h-4 text-emerald-400" />
                  Compatibility Evaluation
                </div>
                <div className="text-xs text-neutral-300">
                  {mod.compatibilityReason}
                </div>
                <div className="pt-2 text-[11px] text-neutral-400 font-mono">
                  Supported Engine Releases: {mod.supportedGameVersions.join(', ')}
                </div>
              </div>

              <div>
                <h3 className="text-xs font-semibold text-neutral-400 uppercase tracking-wider mb-2">
                  Required Modding Frameworks
                </h3>
                {mod.requiredDependencies.length === 0 ? (
                  <div className="text-xs text-neutral-400 italic">No external dependencies required.</div>
                ) : (
                  <div className="space-y-1.5">
                    {mod.requiredDependencies.map((dep, idx) => (
                      <div key={idx} className="p-2.5 bg-neutral-800/80 border border-neutral-700 rounded text-xs text-white flex items-center gap-2">
                        <CheckCircle2 className="w-4 h-4 text-accent" />
                        <span>{dep}</span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTab === 'attribution' && (
            <div className="space-y-4 text-xs">
              <div className="bg-bg-card border border-border p-4 rounded space-y-2">
                <div className="font-semibold text-white">Original Source Reference</div>
                <p className="text-neutral-400">
                  This mod metadata is indexed from legitimate GTA community repositories. Download permissions and redistribution rights are respected.
                </p>
                <div className="pt-2">
                  <a
                    href={mod.pageUrl}
                    target="_blank"
                    rel="noreferrer"
                    className="text-accent hover:underline flex items-center gap-1.5 font-semibold"
                  >
                    <span>View original mod page on {mod.sourceId}</span>
                    <ExternalLink className="w-3.5 h-3.5" />
                  </a>
                </div>
              </div>

              <div className="p-3 bg-neutral-900 border border-neutral-800 rounded text-[11px] text-neutral-400">
                SAMM is an independent open-source tool and is not affiliated with or endorsed by Rockstar Games or Take-Two Interactive.
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-border bg-bg-card flex items-center justify-between">
          <a
            href={mod.pageUrl}
            target="_blank"
            rel="noreferrer"
            className="px-4 py-1.5 rounded text-xs font-semibold text-neutral-300 hover:text-white hover:bg-neutral-800 transition-colors flex items-center gap-1.5"
          >
            <ExternalLink className="w-3.5 h-3.5" />
            <span>Open Website Page</span>
          </a>

          {mod.downloadUrl ? (
            <button
              onClick={() => onDownload(mod)}
              className="px-5 py-2 bg-accent hover:bg-accent-hover text-bg-main text-xs font-bold rounded flex items-center gap-1.5 transition-colors shadow"
            >
              <Download className="w-4 h-4" />
              <span>Download Mod File</span>
            </button>
          ) : (
            <a
              href={mod.pageUrl}
              target="_blank"
              rel="noreferrer"
              className="px-4 py-2 bg-neutral-700 hover:bg-neutral-600 text-white text-xs font-semibold rounded flex items-center gap-1.5 transition-colors"
            >
              <ExternalLink className="w-4 h-4" />
              <span>Download via System Browser</span>
            </a>
          )}
        </div>
      </div>
    </div>
  );
};
