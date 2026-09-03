import React, { useState } from 'react';
import { LayoutDashboard, Network, Server, Lock, Cpu, Terminal, Activity, Layers } from 'lucide-react';
import { CabocloIoriLogo } from './CabocloIoriLogo';

interface SidebarProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ activeTab, setActiveTab }) => {
  const [collapsed, setCollapsed] = useState(false);
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'observability', label: 'Observability & FOSS', icon: Activity },
    { id: 'apache-bigdata', label: 'Apache & Big Data', icon: Layers },
    { id: 'tunnels', label: 'Tunnels & DNS', icon: Network },
    { id: 'servers', label: 'Secure Servers', icon: Server },
    { id: 'crypto-vault', label: 'Crypto Vault', icon: Lock },
    { id: 'pfe969', label: 'PFE-969 Lattice Core', icon: Cpu },
    { id: 'terminal', label: 'CLI Terminal', icon: Terminal },
  ];

  return (
    <aside className={`${collapsed ? 'w-24' : 'w-80'} bg-cardbase border-r border-gray-800 flex flex-col h-auto md:h-screen sticky top-0 transition-all duration-300`}>
      <div className={`border-b border-gray-800 relative ${collapsed ? 'p-3 py-4 flex justify-center' : 'p-6 flex flex-col items-center text-center'}`}>
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="focus:outline-none flex flex-col items-center justify-center shrink-0 transition-transform active:scale-95"
          title={collapsed ? "Click logo to expand sidebar" : "Click logo to collapse sidebar"}
        >
          <CabocloIoriLogo collapsed={collapsed} />
        </button>

        {!collapsed && (
          <div className="flex flex-col items-center text-center mt-4 w-full">
            <p className="text-xs font-semibold tracking-wider text-amber-400 uppercase text-center">
              Quantum & Forensic Console
            </p>
            <h1 className="font-extrabold text-lg tracking-widest text-white uppercase text-center mt-1 drop-shadow-[0_2px_4px_rgba(0,0,0,0.8)]">
              IORI THE APACHER
            </h1>
          </div>
        )}
      </div>

      <nav className="flex-1 p-4 space-y-1.5 overflow-y-auto">
        {menuItems.map((item) => {
          const Icon = item.icon;
          const isActive = activeTab === item.id;
          return (
            <button
              key={item.id}
              onClick={() => setActiveTab(item.id)}
              className={`w-full flex items-center ${collapsed ? 'justify-center px-2' : 'space-x-3 px-4'} py-3 rounded-xl font-medium text-sm transition-all duration-200 ${
                isActive
                  ? 'bg-blue-600 text-white shadow-lg shadow-blue-600/30 font-semibold'
                  : 'text-gray-400 hover:text-gray-200 hover:bg-gray-800/60'
              }`}
              title={collapsed ? item.label : undefined}
            >
              <Icon className="w-5 h-5 shrink-0" />
              {!collapsed && <span className="truncate">{item.label}</span>}
            </button>
          );
        })}
      </nav>

      {!collapsed && (
        <div className="p-4 border-t border-gray-800">
          <div className="bg-gray-900/90 border border-gray-800 rounded-xl p-3.5 space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-xs font-semibold text-gray-300">SQLite Storage Sync</span>
              <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            </div>
            <p className="text-[11px] text-gray-500">Connected to iori_apacher.db & API server</p>
          </div>
        </div>
      )}
    </aside>
  );
};
