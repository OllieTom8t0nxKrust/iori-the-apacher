import React, { useState } from 'react';
import { Activity, BarChart3, CheckCircle2, Sliders } from 'lucide-react';

export const ObservabilityDashboard: React.FC = () => {
  const [activeTool, setActiveTool] = useState('grafana');
  const [pluginStatus, setPluginStatus] = useState<Record<string, boolean>>({
    grafana: true,
    prometheus: true,
    jaeger: true,
    opentelemetry: true,
    elk: true,
    zabbix: false,
    netdata: true,
    loki: true,
    victoriametrics: false,
    fluentd: true,
  });

  const togglePlugin = (key: string) => {
    setPluginStatus(prev => ({ ...prev, [key]: !prev[key] }));
  };

  const fossTools = [
    { id: 'grafana', name: 'Grafana Dashboards', desc: 'Enterprise metric visualization & alerts', category: 'Visualization' },
    { id: 'prometheus', name: 'Prometheus Exporter', desc: 'Time-series metrics & scraping engine', category: 'Metrics' },
    { id: 'jaeger', name: 'Jaeger APM Tracing', desc: 'Distributed transaction tracing & latency analysis', category: 'Tracing' },
    { id: 'opentelemetry', name: 'OpenTelemetry Collector', desc: 'Vendor-agnostic telemetry ingestion pipeline', category: 'Telemetry' },
    { id: 'elk', name: 'ELK Stack / Kibana', desc: 'Elasticsearch log aggregation & search', category: 'Logging' },
    { id: 'zabbix', name: 'Zabbix Enterprise Monitor', desc: 'Infrastructure & network health monitoring', category: 'Monitoring' },
    { id: 'netdata', name: 'Netdata Realtime Agent', desc: 'High-resolution node performance monitoring', category: 'Metrics' },
    { id: 'loki', name: 'Grafana Loki Logs', desc: 'Cost-effective log aggregation system', category: 'Logging' },
    { id: 'victoriametrics', name: 'VictoriaMetrics Cluster', desc: 'Scalable long-term time-series database', category: 'Storage' },
    { id: 'fluentd', name: 'Fluentd Log Collector', desc: 'Unified logging layer for container streams', category: 'Pipeline' },
  ];

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">FOSS Observability & Data Visualization Hub</h2>
          <p className="text-sm text-gray-400">Power-plug modular architecture for open-source telemetry and monitoring tools.</p>
        </div>
        <div className="flex items-center space-x-2 bg-blue-500/10 border border-blue-500/30 text-blue-400 px-3 py-1.5 rounded-xl text-xs font-medium">
          <Activity className="w-4 h-4 animate-pulse" />
          <span>10 FOSS Adapters Ready</span>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
        {fossTools.map(tool => {
          const isPlugged = pluginStatus[tool.id];
          return (
            <div 
              key={tool.id}
              onClick={() => setActiveTool(tool.id)}
              className={`bg-cardbase border rounded-xl p-4 cursor-pointer transition-all ${
                activeTool === tool.id 
                  ? 'border-blue-500 ring-2 ring-blue-500/20 shadow-lg shadow-blue-500/10' 
                  : 'border-gray-800 hover:border-gray-700'
              }`}
            >
              <div className="flex justify-between items-start mb-3">
                <span className="text-[10px] font-mono uppercase tracking-wider px-2 py-0.5 rounded bg-gray-800 text-gray-300">
                  {tool.category}
                </span>
                <button
                  onClick={(e) => { e.stopPropagation(); togglePlugin(tool.id); }}
                  className={`w-4 h-4 rounded-full flex items-center justify-center transition-all ${
                    isPlugged ? 'bg-emerald-500 text-black' : 'bg-gray-700 text-gray-400'
                  }`}
                  title={isPlugged ? "Plugin Plugged In (Active)" : "Plugin Unplugged (Inactive)"}
                >
                  {isPlugged && <CheckCircle2 className="w-3 h-3 text-white" />}
                </button>
              </div>
              <h3 className="font-bold text-sm text-white">{tool.name}</h3>
              <p className="text-xs text-gray-400 mt-1 line-clamp-2">{tool.desc}</p>
              <div className="mt-4 flex items-center justify-between text-xs">
                <span className={isPlugged ? 'text-emerald-400 font-semibold' : 'text-gray-500'}>
                  {isPlugged ? '● Plugged In' : '○ Unplugged'}
                </span>
                <span className="text-blue-400 font-mono text-[10px]">Power-Plug v2.4</span>
              </div>
            </div>
          );
        })}
      </div>

      <div className="bg-cardbase border border-gray-800 rounded-2xl p-6">
        <div className="flex justify-between items-center mb-6">
          <div className="flex items-center space-x-3">
            <div className="w-10 h-10 rounded-xl bg-blue-500/10 text-blue-400 flex items-center justify-center">
              <BarChart3 className="w-5 h-5" />
            </div>
            <div>
              <h3 className="font-bold text-lg text-white capitalize">{activeTool} Visualization Panel</h3>
              <p className="text-xs text-gray-400">Live telemetry stream powered by secure IPC adapter bridge.</p>
            </div>
          </div>
          <div className="flex items-center space-x-3">
            <button 
              onClick={() => togglePlugin(activeTool)}
              className={`px-4 py-2 rounded-xl text-xs font-semibold transition-all ${
                pluginStatus[activeTool]
                  ? 'bg-red-500/10 border border-red-500/30 text-red-400 hover:bg-red-500/20'
                  : 'bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 hover:bg-emerald-500/20'
              }`}
            >
              {pluginStatus[activeTool] ? 'Unplug Module' : 'Plug In Module'}
            </button>
            <div className="bg-gray-900 border border-gray-800 px-3 py-2 rounded-xl text-xs font-mono text-gray-300">
              Status: <span className={pluginStatus[activeTool] ? 'text-emerald-400' : 'text-amber-400'}>
                {pluginStatus[activeTool] ? 'STREAMING ACTIVE' : 'STANDBY (UNPLUGGED)'}
              </span>
            </div>
          </div>
        </div>

        {pluginStatus[activeTool] ? (
          <div className="space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div className="bg-gray-900/80 border border-gray-800 rounded-xl p-4">
                <span className="text-xs text-gray-400">Ingestion Rate</span>
                <p className="text-2xl font-bold text-white mt-1">24.8K <span className="text-xs text-emerald-400 font-normal">req/sec</span></p>
              </div>
              <div className="bg-gray-900/80 border border-gray-800 rounded-xl p-4">
                <span className="text-xs text-gray-400">p99 Latency</span>
                <p className="text-2xl font-bold text-white mt-1">1.42 <span className="text-xs text-blue-400 font-normal">ms</span></p>
              </div>
              <div className="bg-gray-900/80 border border-gray-800 rounded-xl p-4">
                <span className="text-xs text-gray-400">Buffer Utilization</span>
                <p className="text-2xl font-bold text-white mt-1">14.2% <span className="text-xs text-emerald-400 font-normal">optimal</span></p>
              </div>
              <div className="bg-gray-900/80 border border-gray-800 rounded-xl p-4">
                <span className="text-xs text-gray-400">Active Traces</span>
                <p className="text-2xl font-bold text-white mt-1">1,840 <span className="text-xs text-purple-400 font-normal">spans</span></p>
              </div>
            </div>

            <div className="h-72 border border-gray-800 rounded-xl bg-gray-900/50 p-6 flex flex-col justify-between">
              <div className="flex justify-between items-center">
                <span className="text-xs font-mono text-gray-400">Telemetry Stream // {activeTool.toUpperCase()} // Real-time Graph</span>
                <span className="w-2 h-2 rounded-full bg-emerald-400 animate-ping"></span>
              </div>
              <div className="flex items-end space-x-2 h-44 px-2">
                {[40, 65, 30, 85, 95, 45, 70, 80, 60, 90, 75, 55, 88, 92, 48, 76, 84, 62, 98, 85].map((val, idx) => (
                  <div key={idx} className="flex-1 bg-blue-600/30 hover:bg-blue-500 rounded-t transition-all" style={{ height: `${val}%` }}></div>
                ))}
              </div>
              <div className="flex justify-between text-[11px] text-gray-500 font-mono">
                <span>T-20s</span>
                <span>T-15s</span>
                <span>T-10s</span>
                <span>T-5s</span>
                <span>NOW (LIVE)</span>
              </div>
            </div>
          </div>
        ) : (
          <div className="h-64 flex flex-col items-center justify-center border border-dashed border-gray-800 rounded-xl bg-gray-900/20 space-y-3">
            <div className="w-12 h-12 rounded-full bg-gray-800 flex items-center justify-center text-gray-400">
              <Sliders className="w-6 h-6" />
            </div>
            <div className="text-center">
              <p className="text-sm font-medium text-white">Module "{activeTool}" is currently unplugged</p>
              <p className="text-xs text-gray-400 mt-1">Click "Plug In Module" above to activate power-plug stream.</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
