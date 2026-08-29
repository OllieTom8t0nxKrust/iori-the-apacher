import React from 'react';
import { Network, ShieldAlert, Lock, Atom, ArrowUpRight, Activity } from 'lucide-react';

export const DashboardOverview: React.FC = () => {
  const stats = [
    { label: 'Active Tunnels', value: '4', change: '+100%', icon: Network, color: 'text-blue-400', bg: 'bg-blue-500/10' },
    { label: 'Forensic Hits', value: '1,284', change: '+12.4%', icon: ShieldAlert, color: 'text-amber-400', bg: 'bg-amber-500/10' },
    { label: 'Domestic Ciphers', value: 'AES / ChaCha', change: 'Active', icon: Lock, color: 'text-emerald-400', bg: 'bg-emerald-500/10' },
    { label: 'Quantum PFE-969', value: 'Secured', change: '100% PQC', icon: Atom, color: 'text-purple-400', bg: 'bg-purple-500/10' },
  ];

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">System Dashboard</h2>
          <p className="text-sm text-gray-400">Overview of active tunnels, forensic telemetry, and cryptographic states.</p>
        </div>
        <div className="flex items-center space-x-2 bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 px-3 py-1.5 rounded-xl text-xs font-medium">
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
          <span>System Fully Operational</span>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5">
        {stats.map((stat, i) => {
          const Icon = stat.icon;
          return (
            <div key={i} className="bg-cardbase border border-gray-800 rounded-2xl p-5 shadow-sm hover:border-gray-700 transition-all">
              <div className="flex justify-between items-start">
                <div className={`w-12 h-12 rounded-xl ${stat.bg} ${stat.color} flex items-center justify-center`}>
                  <Icon className="w-6 h-6" />
                </div>
                <span className="text-xs font-semibold px-2.5 py-1 rounded-full bg-gray-800 text-gray-300">
                  {stat.change}
                </span>
              </div>
              <div className="mt-4">
                <h3 className="text-2xl font-bold text-white tracking-tight">{stat.value}</h3>
                <p className="text-xs text-gray-400 mt-1">{stat.label}</p>
              </div>
            </div>
          );
        })}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-cardbase border border-gray-800 rounded-2xl p-6">
          <div className="flex justify-between items-center mb-6">
            <h3 className="font-bold text-lg text-white">Tunnel Bandwidth & Traffic</h3>
            <div className="flex items-center space-x-2 text-xs text-gray-400">
              <Activity className="w-4 h-4 text-blue-400" />
              <span>Real-time Telemetry</span>
            </div>
          </div>
          <div className="h-64 flex items-center justify-center border border-dashed border-gray-800 rounded-xl bg-gray-900/40">
            <div className="text-center space-y-2">
              <p className="text-sm text-gray-400">Interactive Adobe UX/UI compatible visual traffic graph</p>
              <span className="text-xs font-mono text-blue-400 bg-blue-500/10 px-3 py-1 rounded-full">
                HTTP / QUIC / TCP Multistream
              </span>
            </div>
          </div>
        </div>

        <div className="bg-cardbase border border-gray-800 rounded-2xl p-6 flex flex-col justify-between">
          <div>
            <h3 className="font-bold text-lg text-white mb-2">Cryptographic Vault</h3>
            <p className="text-xs text-gray-400 mb-4">Active cipher protection status across nodes.</p>
            <div className="space-y-3">
              <div className="p-3 rounded-xl bg-gray-900 border border-gray-800 flex justify-between items-center">
                <span className="text-sm font-medium text-gray-300">PFE-969 Lattice</span>
                <span className="text-xs text-purple-400 bg-purple-500/10 px-2.5 py-1 rounded-full">Active</span>
              </div>
              <div className="p-3 rounded-xl bg-gray-900 border border-gray-800 flex justify-between items-center">
                <span className="text-sm font-medium text-gray-300">ML-KEM-1024</span>
                <span className="text-xs text-cyan-400 bg-cyan-500/10 px-2.5 py-1 rounded-full">Ready</span>
              </div>
              <div className="p-3 rounded-xl bg-gray-900 border border-gray-800 flex justify-between items-center">
                <span className="text-sm font-medium text-gray-300">AES-256-GCM</span>
                <span className="text-xs text-emerald-400 bg-emerald-500/10 px-2.5 py-1 rounded-full">Active</span>
              </div>
            </div>
          </div>
          <button className="mt-6 w-full py-2.5 bg-blue-600 hover:bg-blue-500 text-white font-medium text-sm rounded-xl transition-all shadow-lg shadow-blue-600/20 flex items-center justify-center space-x-2">
            <span>Export Architecture Spec</span>
            <ArrowUpRight className="w-4 h-4" />
          </button>
        </div>
      </div>
    </div>
  );
};
