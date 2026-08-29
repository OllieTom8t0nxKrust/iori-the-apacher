import React from 'react';
import { LayoutDashboard, Network, ShieldAlert, Lock, Cpu } from 'lucide-react';
import { CabocloIoriLogo } from './CabocloIoriLogo';

interface SidebarProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ activeTab, setActiveTab }) => {
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'tunnels', label: 'Tunnels & DNS', icon: Network },
    { id: 'forensic', label: 'Forensic Intelligence', icon: ShieldAlert },
    { id: 'crypto-vault', label: 'Crypto Vault', icon: Lock },
    { id: 'pfe969', label: 'PFE-969 Lattice Core', icon: Cpu },
  ];

  return (
    <aside className="w-72 bg-cardbase border-r border-gray-800 flex flex-col h-screen sticky top-0">
      <div className="p-6 border-b border-gray-800 flex items-center space-x-3">
        <CabocloIoriLogo />
        <div>
          <h1 className="font-bold text-lg tracking-tight text-white">Caboclo Iori</h1>
          <p className="text-xs text-amber-400/80 font-medium">Quantum & Forensic Console</p>
        </div>
      </div>

      <nav className="flex-1 p-4 space-y-1.5 overflow-y-auto">
        {menuItems.map((item) => {
          const Icon = item.icon;
          const isActive = activeTab === item.id;
          return (
            <button
              key={item.id}
              onClick={() => setActiveTab(item.id)}
              className={`w-full flex items-center space-x-3 px-4 py-3 rounded-xl font-medium text-sm transition-all duration-200 ${
                isActive
                  ? 'bg-blue-600 text-white shadow-lg shadow-blue-600/30'
                  : 'text-gray-400 hover:text-gray-200 hover:bg-gray-800/60'
              }`}
            >
              <Icon className={`w-5 h-5 ${isActive ? 'text-white' : 'text-gray-400'}`} />
              <span>{item.label}</span>
            </button>
          );
        })}
      </nav>

      <div className="p-4 border-t border-gray-800">
        <div className="bg-gray-900/80 rounded-xl p-3 border border-gray-800 text-xs text-gray-400 space-y-1">
          <div className="flex justify-between">
            <span>Architecture:</span>
            <span className="text-cyan-400 font-mono">Binary / QPU</span>
          </div>
          <div className="flex justify-between">
            <span>Security Lvl:</span>
            <span className="text-emerald-400 font-mono">360° PQC</span>
          </div>
        </div>
      </div>
    </aside>
  );
};
