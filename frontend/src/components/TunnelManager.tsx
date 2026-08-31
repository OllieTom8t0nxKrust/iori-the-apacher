import React, { useState, useEffect } from 'react';
import { Plus, Globe, Trash2 } from 'lucide-react';

interface Tunnel {
  id: string;
  subdomain: string;
  target_port: number;
  protocol: string;
  active: boolean;
  created_at: string;
}

export const TunnelManager: React.FC = () => {
  const [tunnels, setTunnels] = useState<Tunnel[]>([]);
  const [subdomain, setSubdomain] = useState('');
  const [port, setPort] = useState('8080');
  const [protocol, setProtocol] = useState('https');
  const [error, setError] = useState<string | null>(null);

  const fetchTunnels = async () => {
    try {
      const res = await fetch('http://localhost:8080/api/tunnels');
      if (res.ok) {
        const data = await res.json();
        setTunnels(data);
        setError(null);
      }
    } catch (e) {
      const cached = localStorage.getItem('iori_tunnels');
      if (cached) {
        setTunnels(JSON.parse(cached));
      }
    }
  };

  useEffect(() => {
    fetchTunnels();
    const interval = setInterval(fetchTunnels, 2000);
    return () => clearInterval(interval);
  }, []);

  const createTunnel = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!subdomain) return;
    setError(null);

    const payload = {
      subdomain,
      port: parseInt(port) || 80,
      protocol,
    };

    try {
      const res = await fetch('http://localhost:8080/api/tunnels', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (res.ok) {
        const newTunnel = await res.json();
        const updated = [...tunnels, newTunnel];
        setTunnels(updated);
        localStorage.setItem('iori_tunnels', JSON.stringify(updated));
        setSubdomain('');
      } else {
        const errJson = await res.json();
        setError(errJson.error || 'Failed to create tunnel');
      }
    } catch (e) {
      // Fallback local creation if API is offline
      const newTunnel: Tunnel = {
        id: Math.random().toString(36).substring(7),
        subdomain,
        target_port: parseInt(port) || 80,
        protocol,
        active: true,
        created_at: new Date().toISOString(),
      };
      const updated = [...tunnels, newTunnel];
      setTunnels(updated);
      localStorage.setItem('iori_tunnels', JSON.stringify(updated));
      setSubdomain('');
    }
  };

  const removeTunnel = async (id: string) => {
    try {
      await fetch(`http://localhost:8080/api/tunnels/${id}`, { method: 'DELETE' });
      const updated = tunnels.filter(t => t.id !== id);
      setTunnels(updated);
      localStorage.setItem('iori_tunnels', JSON.stringify(updated));
    } catch (e) {
      const updated = tunnels.filter(t => t.id !== id);
      setTunnels(updated);
      localStorage.setItem('iori_tunnels', JSON.stringify(updated));
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Tunnel & Custom DNS Management</h2>
          <p className="text-sm text-gray-400">NGINX & NGROK inspired secure tunneling for binary and quantum endpoints.</p>
        </div>
      </div>

      {error && (
        <div className="p-4 rounded-xl bg-red-500/10 border border-red-500/30 text-red-400 text-sm">
          {error}
        </div>
      )}

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
          <h3 className="font-bold text-lg text-white mb-4">Active Tunnels & DNS Records</h3>
          {tunnels.length === 0 ? (
            <div className="text-center py-12 text-gray-500 text-sm">
              No active tunnels found. Create one using the form or CLI command `iori-the-apacher tunnel create`.
            </div>
          ) : (
            <div className="space-y-3">
              {tunnels.map((tunnel) => (
                <div key={tunnel.id} className="p-4 rounded-xl bg-gray-900 border border-gray-800 flex items-center justify-between">
                  <div className="space-y-1">
                    <div className="flex items-center space-x-2">
                      <Globe className="w-4 h-4 text-blue-400" />
                      <a
                        href={`https://${tunnel.subdomain}.iori.net`}
                        target="_blank"
                        rel="noreferrer"
                        className="font-bold text-white text-sm hover:underline"
                      >
                        {tunnel.subdomain}.iori.net
                      </a>
                      <span className="text-xs bg-blue-500/10 text-blue-400 px-2.5 py-0.5 rounded-full uppercase">
                        {tunnel.protocol}
                      </span>
                    </div>
                    <div className="text-xs text-gray-400">
                      Local Target Port: <strong className="text-gray-200">{tunnel.target_port}</strong> | ID: <span className="font-mono">{tunnel.id}</span>
                    </div>
                  </div>
                  <div className="flex items-center space-x-3">
                    <span className="px-2.5 py-1 rounded-full text-xs bg-emerald-500/10 text-emerald-400 font-medium">
                      {tunnel.active ? 'Active' : 'Stopped'}
                    </span>
                    <button
                      onClick={() => removeTunnel(tunnel.id)}
                      className="p-2 rounded-lg bg-red-500/10 text-red-400 hover:bg-red-500/20 transition-all"
                      title="Terminate Tunnel"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
