import React from 'react';
import { Logo } from './Logo';
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
    { id: 'installations' as NavTab, label: 'Game Installations', icon: Gamepad2 },
    { id: 'conflicts' as NavTab, label: 'Conflicts', icon: AlertTriangle, badge: conflictCount > 0 ? conflictCount : undefined, badgeColor: 'bg-amber-600' },
    { id: 'backups' as NavTab, label: 'Backups & Restore', icon: History },
    { id: 'diagnostics' as NavTab, label: 'Diagnostics', icon: Activity },
    { id: 'settings' as NavTab, label: 'Settings', icon: Settings },
  ];

  return (
    <aside className="w-64 bg-bg-subtle border-r border-border-subtle flex flex-col justify-between select-none">
      <div>
        {/* Brand Header */}
        <div className="h-16 flex items-center px-4 border-b border-border-subtle gap-3 bg-bg-main/40">
          <Logo className="w-9 h-9 drop-shadow-md flex-shrink-0" />
          <div className="min-w-0">
            <div className="font-bold tracking-tight text-white flex items-center gap-1.5 leading-none">
              <span className="font-mono text-sm tracking-wide text-amber-400 font-extrabold">SAMM</span>
              <span className="text-[9px] font-mono font-bold px-1.5 py-0.5 bg-amber-950/70 text-amber-300 border border-amber-800/60 rounded">v1.0.0</span>
            </div>
            <div className="text-[10px] text-neutral-400 tracking-tight font-medium mt-1 truncate">San Andreas Mod Manager</div>
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

      {/* System Status Footer */}
      <div className="p-3 border-t border-border-subtle bg-bg-card/30 m-2 rounded text-xs text-neutral-400 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-emerald-500"></span>
          <span className="text-[11px] font-medium text-neutral-300">GTA: SA v1.0</span>
        </div>
        <span className="text-[10px] font-mono text-neutral-500">Ready</span>
      </div>
    </aside>
  );
};
