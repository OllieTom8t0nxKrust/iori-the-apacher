import React, { useState } from 'react';
import { Network, Plus, Globe, Trash2 } from 'lucide-react';

interface Tunnel {
  id: string;
  subdomain: string;
  port: number;
  protocol: string;
  url: string;
  status: string;
}

export const TunnelManager: React.FC = () => {
  const [tunnels, setTunnels] = useState<Tunnel[]>([]);

  const [subdomain, setSubdomain] = useState('');
  const [port, setPort] = useState('8080');
  const [protocol, setProtocol] = useState('https');

  const createTunnel = (e: React.FormEvent) => {
    e.preventDefault();
    if (!subdomain) return;
    const newTunnel: Tunnel = {
      id: Math.random().toString(36).substring(7),
      subdomain,
      port: parseInt(port) || 80,
      protocol,
      url: `${protocol}://${subdomain}.iori.apacher.net`,
      status: 'Active',
    };
    setTunnels([...tunnels, newTunnel]);
    setSubdomain('');
  };

  const removeTunnel = (id: string) => {
    setTunnels(tunnels.filter(t => t.id !== id));
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Tunnel & Custom DNS Management</h2>
          <p className="text-sm text-gray-400">NGINX & NGROK inspired secure tunneling for binary and quantum endpoints.</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="bg-cardbase border border-gray-800 rounded-2xl p-6">
          <h3 className="font-bold text-lg text-white mb-4">Create New Tunnel</h3>
          <form onSubmit={createTunnel} className="space-y-4">
            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Subdomain</label>
              <div className="flex rounded-xl bg-gray-900 border border-gray-800 overflow-hidden">
                <input
                  type="text"
                  value={subdomain}
                  onChange={(e) => setSubdomain(e.target.value)}
                  placeholder="my-service"
                  className="w-full bg-transparent px-4 py-2.5 text-sm text-white focus:outline-none"
                  required
                />
                <span className="bg-gray-800 px-3 py-2.5 text-xs text-gray-400 flex items-center">.iori.net</span>
              </div>
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Local Target Port</label>
              <input
                type="number"
                value={port}
                onChange={(e) => setPort(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
                required
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Protocol</label>
              <select
                value={protocol}
                onChange={(e) => setProtocol(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
              >
                <option value="https">HTTPS (Secure Proxy)</option>
                <option value="http">HTTP (Standard)</option>
                <option value="quic">QUIC / HTTP/3 (Quantum Optimized)</option>
                <option value="tcp">Raw TCP</option>
              </select>
            </div>

            <button
              type="submit"
              className="w-full py-3 bg-blue-600 hover:bg-blue-500 text-white font-medium text-sm rounded-xl transition-all shadow-lg shadow-blue-600/20 flex items-center justify-center space-x-2"
            >
              <Plus className="w-4 h-4" />
              <span>Initialize Tunnel</span>
            </button>
          </form>
        </div>

        <div className="lg:col-span-2 bg-cardbase border border-gray-800 rounded-2xl p-6">
          <h3 className="font-bold text-lg text-white mb-4">Active Tunnel Endpoints</h3>
          <div className="space-y-3">
            {tunnels.map((tunnel) => (
              <div key={tunnel.id} className="bg-gray-900 border border-gray-800 rounded-xl p-4 flex items-center justify-between hover:border-gray-700 transition-all">
                <div className="flex items-center space-x-3">
                  <div className="w-10 h-10 rounded-xl bg-blue-500/10 text-blue-400 flex items-center justify-center">
                    <Network className="w-5 h-5" />
                  </div>
                  <div>
                    <div className="flex items-center space-x-2">
                      <span className="font-semibold text-white text-sm">{tunnel.subdomain}</span>
                      <span className="text-xs px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 font-medium">
                        {tunnel.status}
                      </span>
                    </div>
                    <a href={tunnel.url} target="_blank" rel="noreferrer" className="text-xs text-blue-400 hover:underline flex items-center space-x-1 mt-0.5">
                      <Globe className="w-3 h-3" />
                      <span>{tunnel.url}</span>
                    </a>
                  </div>
                </div>

                <div className="flex items-center space-x-4">
                  <div className="text-right text-xs text-gray-400">
                    <div>Local: <span className="text-white font-mono">:{tunnel.port}</span></div>
                    <div>Proto: <span className="text-cyan-400 font-mono uppercase">{tunnel.protocol}</span></div>
                  </div>
                  <button
                    onClick={() => removeTunnel(tunnel.id)}
                    className="p-2 rounded-lg bg-gray-800 text-gray-400 hover:text-red-400 hover:bg-red-500/10 transition-all"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};
