import React, { useState } from 'react';
import { LayoutDashboard, Network, Server, Lock, Cpu, Terminal } from 'lucide-react';
import { CabocloIoriLogo } from './CabocloIoriLogo';

interface SidebarProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ activeTab, setActiveTab }) => {
  const [collapsed, setCollapsed] = useState(false);
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'tunnels', label: 'Tunnels & DNS', icon: Network },
    { id: 'servers', label: 'Secure Servers', icon: Server },
    { id: 'crypto-vault', label: 'Crypto Vault', icon: Lock },
    { id: 'pfe969', label: 'PFE-969 Lattice Core', icon: Cpu },
    { id: 'terminal', label: 'CLI Terminal', icon: Terminal },
  ];

  return (
    <aside className={`${collapsed ? 'w-20' : 'w-72'} bg-cardbase border-r border-gray-800 flex flex-col h-auto md:h-screen sticky top-0 transition-all duration-300`}>
      <div className={`p-6 border-b border-gray-800 flex items-center ${collapsed ? 'justify-center' : 'space-x-3'}`}>
        <button onClick={() => setCollapsed(!collapsed)} className="focus:outline-none">
          <CabocloIoriLogo collapsed={collapsed} />
        </button>
        {!collapsed && (
          <div>
            <h1 className="font-bold text-lg tracking-tight text-white">IORI THE APACHER</h1>
            <p className="text-xs text-amber-400/80 font-medium">Quantum & Forensic Console</p>
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
              className={`w-full flex items-center ${collapsed ? 'justify-center' : 'space-x-3'} px-4 py-3 rounded-xl font-medium text-sm transition-all duration-200 ${
                isActive
                  ? 'bg-blue-600 text-white shadow-lg shadow-blue-600/30'
                  : 'text-gray-400 hover:text-gray-200 hover:bg-gray-800/60'
              }`}
            >
              <Icon className="w-5 h-5" />
              {!collapsed && <span>{item.label}</span>}
            </button>
          );
        })}
      </nav>

      {!collapsed && (
        <div className="p-4 border-t border-gray-800">
          <div className="bg-gray-900 border border-gray-800 rounded-xl p-3.5 space-y-2">
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
