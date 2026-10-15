import React from 'react';
import {
  Home,
  Compass,
  Package,
  DownloadCloud,
  Layers,
  Gamepad2,
  AlertTriangle,
  History,
  Activity,
  Settings,
} from 'lucide-react';

export type NavTab =
  | 'home'
  | 'discover'
  | 'installed'
  | 'downloads'
  | 'profiles'
  | 'installations'
  | 'conflicts'
  | 'backups'
  | 'diagnostics'
  | 'settings';

interface SidebarProps {
  currentTab: NavTab;
  onSelectTab: (tab: NavTab) => void;
  installedCount: number;
  downloadCount: number;
  conflictCount: number;
}

export const Sidebar: React.FC<SidebarProps> = ({
  currentTab,
  onSelectTab,
  installedCount,
  downloadCount,
  conflictCount,
}) => {
  const navItems = [
    { id: 'home' as NavTab, label: 'Overview', icon: Home },
    { id: 'discover' as NavTab, label: 'Discover Mods', icon: Compass },
    { id: 'installed' as NavTab, label: 'Installed Mods', icon: Package, badge: installedCount > 0 ? installedCount : undefined },
    { id: 'downloads' as NavTab, label: 'Downloads', icon: DownloadCloud, badge: downloadCount > 0 ? downloadCount : undefined },
    { id: 'profiles' as NavTab, label: 'Profiles', icon: Layers },
    { id: 'installations' as NavTab, label: 'Game Versions', icon: Gamepad2 },
    { id: 'conflicts' as NavTab, label: 'Conflicts', icon: AlertTriangle, badge: conflictCount > 0 ? conflictCount : undefined, badgeColor: 'bg-amber-600' },
    { id: 'backups' as NavTab, label: 'Backups & Restore', icon: History },
    { id: 'diagnostics' as NavTab, label: 'Diagnostics', icon: Activity },
    { id: 'settings' as NavTab, label: 'Settings', icon: Settings },
  ];

  return (
    <aside className="w-64 bg-bg-subtle border-r border-border-subtle flex flex-col justify-between select-none">
      <div>
        {/* Brand Header */}
        <div className="h-16 flex items-center px-5 border-b border-border-subtle gap-3">
          <div className="w-8 h-8 rounded bg-accent flex items-center justify-center font-black text-bg-main text-base shadow-sm">
            SA
          </div>
          <div>
            <div className="font-bold tracking-tight text-white flex items-center gap-1.5">
              <span>SAMM</span>
              <span className="text-[10px] font-mono px-1 py-0.2 bg-neutral-800 text-neutral-400 rounded">v0.1</span>
            </div>
            <div className="text-[11px] text-neutral-400 tracking-tight">San Andreas Mod Manager</div>
          </div>
        </div>

        {/* Navigation items */}
        <nav className="p-3 space-y-1">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = currentTab === item.id;
            return (
              <button
                key={item.id}
                onClick={() => onSelectTab(item.id)}
                className={`w-full flex items-center justify-between px-3 py-2 rounded text-xs font-medium transition-colors ${
                  isActive
                    ? 'bg-neutral-800 text-accent font-semibold border-l-2 border-accent shadow-sm'
                    : 'text-neutral-300 hover:bg-neutral-800/60 hover:text-white'
                }`}
              >
                <div className="flex items-center gap-3">
                  <Icon className={`w-4 h-4 ${isActive ? 'text-accent' : 'text-neutral-400'}`} />
                  <span>{item.label}</span>
                </div>
                {item.badge !== undefined && (
                  <span
                    className={`text-[10px] px-1.5 py-0.5 rounded-full font-mono font-bold text-white ${
                      item.badgeColor || 'bg-neutral-700'
                    }`}
                  >
                    {item.badge}
                  </span>
                )}
              </button>
            );
          })}
        </nav>
      </div>

      {/* Safety Notice & Baseline Status */}
      <div className="p-4 border-t border-border-subtle bg-bg-card/40 m-2 rounded text-[11px] text-neutral-400">
        <div className="font-semibold text-neutral-300 mb-1 flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
          Engine Guard Active
        </div>
        <div>Transactions isolated with automatic recovery journal and backups.</div>
      </div>
    </aside>
  );
};
