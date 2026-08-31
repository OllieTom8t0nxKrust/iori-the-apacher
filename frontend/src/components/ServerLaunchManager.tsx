import React, { useState, useEffect } from 'react';
import { Plus, Trash2 } from 'lucide-react';

interface ServerLaunch {
  id: string;
  subdomain: string;
  target_port: number;
  protocol: string;
  crypto_requirement: string;
  multi_hop_nodes: string[];
  proxychains_enabled: boolean;
  public_internet_launch: boolean;
  status: string;
  created_at: string;
}

export const ServerLaunchManager: React.FC = () => {
  const [servers, setServers] = useState<ServerLaunch[]>([]);
  const [subdomain, setSubdomain] = useState('');
  const [port, setPort] = useState('8080');
  const [protocol, setProtocol] = useState('https');
  const [crypto, setCrypto] = useState('pfe969');
  const [publicLaunch, setPublicLaunch] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchServers = async () => {
    try {
      const res = await fetch('http://localhost:8080/api/servers');
      if (res.ok) {
        const data = await res.json();
        setServers(data);
        setError(null);
      }
    } catch (e) {
      // Fallback to localStorage if backend API is offline
      const cached = localStorage.getItem('iori_servers');
      if (cached) {
        setServers(JSON.parse(cached));
      }
    }
  };

  useEffect(() => {
    fetchServers();
    const interval = setInterval(fetchServers, 2000);
    return () => clearInterval(interval);
  }, []);

  const handleLaunch = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    const payload = {
      subdomain,
      port: parseInt(port) || 8080,
      protocol,
      crypto,
      public: publicLaunch
    };

    try {
      const res = await fetch('http://localhost:8080/api/servers', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (res.ok) {
        const newServer = await res.json();
        const updated = [...servers, newServer];
        setServers(updated);
        localStorage.setItem('iori_servers', JSON.stringify(updated));
        setSubdomain('');
      } else {
        const errJson = await res.json();
        setError(errJson.error || 'Failed to launch server');
      }
    } catch (e) {
      setError('Backend API unreachable. Ensure `iori-the-apacher api-server` is running.');
    }
  };

  const deleteServer = async (id: string) => {
    try {
      await fetch(`http://localhost:8080/api/servers/${id}`, { method: 'DELETE' });
      const updated = servers.filter(s => s.id !== id);
      setServers(updated);
      localStorage.setItem('iori_servers', JSON.stringify(updated));
    } catch (e) {
      const updated = servers.filter(s => s.id !== id);
      setServers(updated);
      localStorage.setItem('iori_servers', JSON.stringify(updated));
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Secure Server & Routing Manager</h2>
          <p className="text-sm text-gray-400">Launch enterprise servers with Tor, I2P, Freenet, proxychains, and PQC verification.</p>
        </div>
      </div>

      {error && (
        <div className="p-4 rounded-xl bg-red-500/10 border border-red-500/30 text-red-400 text-sm">
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="bg-cardbase border border-gray-800 rounded-2xl p-6">
          <h3 className="font-bold text-lg text-white mb-4">Launch Secure Server</h3>
          <form onSubmit={handleLaunch} className="space-y-4">
            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Subdomain</label>
              <input
                type="text"
                value={subdomain}
                onChange={(e) => setSubdomain(e.target.value)}
                placeholder="quantum-node"
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
                required
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Target Port</label>
              <input
                type="number"
                value={port}
                onChange={(e) => setPort(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
                required
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Protocol / Network</label>
              <select
                value={protocol}
                onChange={(e) => setProtocol(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
              >
                <option value="https">HTTPS (Secure TLS)</option>
                <option value="quic">QUIC / HTTP/3 (Quantum)</option>
                <option value="tor">Tor Onion v3</option>
                <option value="i2p">I2P Stream</option>
                <option value="freenet">Freenet SST</option>
                <option value="tcp">Raw TCP</option>
              </select>
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Crypto Requirement</label>
              <select
                value={crypto}
                onChange={(e) => setCrypto(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
              >
                <option value="pfe969">PFE-969 Lattice (Mandatory PQC)</option>
                <option value="kyber">ML-KEM-1024 / Kyber</option>
                <option value="dilithium">ML-DSA / Dilithium</option>
                <option value="aes">AES-256-GCM (Domestic)</option>
                <option value="chacha">ChaCha20-Poly1305</option>
              </select>
            </div>

            <div className="flex items-center space-x-3 pt-2">
              <input
                type="checkbox"
                id="publicLaunch"
                checked={publicLaunch}
                onChange={(e) => setPublicLaunch(e.target.checked)}
                className="rounded bg-gray-900 border-gray-800 text-blue-600 focus:ring-blue-500"
              />
              <label htmlFor="publicLaunch" className="text-xs text-gray-300">Public Internet Launch (Enforces Crypto Policy)</label>
            </div>

            <button
              type="submit"
              className="w-full py-3 bg-purple-600 hover:bg-purple-500 text-white font-medium text-sm rounded-xl transition-all shadow-lg shadow-purple-600/20 flex items-center justify-center space-x-2"
            >
              <Plus className="w-4 h-4" />
              <span>Launch Secure Server</span>
            </button>
          </form>
        </div>

        <div className="lg:col-span-2 bg-cardbase border border-gray-800 rounded-2xl p-6">
          <h3 className="font-bold text-lg text-white mb-4">Launched Servers & Routing Registry</h3>
          {servers.length === 0 ? (
            <div className="text-center py-12 text-gray-500 text-sm">
              No servers launched yet. Initialize a secure server using the form or CLI.
            </div>
          ) : (
            <div className="space-y-3">
              {servers.map((s) => (
                <div key={s.id} className="p-4 rounded-xl bg-gray-900 border border-gray-800 flex items-center justify-between">
                  <div className="space-y-1">
                    <div className="flex items-center space-x-2">
                      <span className="font-bold text-white text-sm">{s.subdomain}</span>
                      <span className="text-xs bg-purple-500/10 text-purple-400 px-2.5 py-0.5 rounded-full font-mono">{s.protocol}</span>
                      <span className="text-xs bg-blue-500/10 text-blue-400 px-2.5 py-0.5 rounded-full">Port {s.target_port}</span>
                    </div>
                    <div className="text-xs text-gray-400 flex items-center space-x-3">
                      <span>Crypto: <strong className="text-gray-200">{s.crypto_requirement}</strong></span>
                      <span>Public: <strong className="text-gray-200">{s.public_internet_launch ? 'Yes' : 'No'}</strong></span>
                      <span>Status: <strong className="text-emerald-400">{s.status}</strong></span>
                    </div>
                  </div>
                  <button
                    onClick={() => deleteServer(s.id)}
                    className="p-2 rounded-lg bg-red-500/10 text-red-400 hover:bg-red-500/20 transition-all"
                    title="Stop/Delete Server"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
