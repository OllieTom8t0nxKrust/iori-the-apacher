import React, { useState } from 'react';
import { ShieldAlert, Terminal } from 'lucide-react';

interface TelemetryHit {
  id: string;
  ip: string;
  userAgent: string;
  hardware: string;
  geo: string;
  timestamp: string;
}

export const ForensicTracker: React.FC = () => {
  const [hits, setHits] = useState<TelemetryHit[]>([]);

  const [ip, setIp] = useState('');
  const [userAgent, setUserAgent] = useState('Mozilla/5.0 (X11; Linux x86_64)');
  const [hardware, setHardware] = useState('ARM64 / QPU Co-processor');
  const [geo, setGeo] = useState('Tokyo, JP');

  const recordHit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!ip) return;
    const newHit: TelemetryHit = {
      id: Math.random().toString(36).substring(7),
      ip,
      userAgent,
      hardware,
      geo,
      timestamp: new Date().toISOString().replace('T', ' ').substring(0, 19),
    };
    setHits([newHit, ...hits]);
    setIp('');
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Forensic Intelligence & Grabify Suite</h2>
          <p className="text-sm text-gray-400">High-precision IP telemetry, hardware fingerprinting, and behavioral analysis.</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="bg-cardbase border border-gray-800 rounded-2xl p-6">
          <h3 className="font-bold text-lg text-white mb-4">Simulate Forensic Ingestion</h3>
          <form onSubmit={recordHit} className="space-y-4">
            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Source IP Address</label>
              <input
                type="text"
                value={ip}
                onChange={(e) => setIp(e.target.value)}
                placeholder="203.0.113.195"
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500 font-mono"
                required
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">User Agent</label>
              <input
                type="text"
                value={userAgent}
                onChange={(e) => setUserAgent(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Hardware Fingerprint</label>
              <input
                type="text"
                value={hardware}
                onChange={(e) => setHardware(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
              />
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Geolocation</label>
              <input
                type="text"
                value={geo}
                onChange={(e) => setGeo(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
              />
            </div>

            <button
              type="submit"
              className="w-full py-3 bg-amber-600 hover:bg-amber-500 text-white font-medium text-sm rounded-xl transition-all shadow-lg shadow-amber-600/20 flex items-center justify-center space-x-2"
            >
              <ShieldAlert className="w-4 h-4" />
              <span>Capture Telemetry</span>
            </button>
          </form>
        </div>

        <div className="lg:col-span-2 bg-cardbase border border-gray-800 rounded-2xl p-6">
          <div className="flex justify-between items-center mb-4">
            <h3 className="font-bold text-lg text-white">Live Forensic Telemetry Stream</h3>
            <span className="text-xs text-gray-400 flex items-center space-x-1">
              <Terminal className="w-3.5 h-3.5 text-amber-400" />
              <span>{hits.length} Total Hits Logged</span>
            </span>
          </div>

          <div className="space-y-3">
            {hits.map((hit) => (
              <div key={hit.id} className="bg-gray-900 border border-gray-800 rounded-xl p-4 hover:border-gray-700 transition-all space-y-2">
                <div className="flex justify-between items-center">
                  <div className="flex items-center space-x-2">
                    <span className="font-mono text-sm text-amber-400 font-bold">{hit.ip}</span>
                    <span className="text-xs px-2 py-0.5 rounded-full bg-gray-800 text-gray-300">
                      {hit.geo}
                    </span>
                  </div>
                  <span className="text-xs text-gray-500 font-mono">{hit.timestamp}</span>
                </div>

                <div className="text-xs text-gray-400 space-y-1 bg-gray-950/60 p-2.5 rounded-lg border border-gray-800/80 font-mono">
                  <div>UA: <span className="text-gray-300">{hit.userAgent}</span></div>
                  <div>HW: <span className="text-cyan-400">{hit.hardware}</span></div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};
