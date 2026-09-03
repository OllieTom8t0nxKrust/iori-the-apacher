import React, { useState } from 'react';
import { Layers, Database, Cpu, Terminal, CheckCircle2, Server, Workflow, Flame, ShieldAlert, CpuIcon } from 'lucide-react';

export const ApacheBigDataHub: React.FC = () => {
  const [selectedTool, setSelectedTool] = useState('kafka');

  const apacheEcosystem = [
    { id: 'kafka', name: 'Apache Kafka', category: 'Streaming & Messaging', status: 'Active Cluster', throughput: '1.2M msgs/sec' },
    { id: 'spark', name: 'Apache Spark', category: 'Distributed Compute', status: 'Worker Pool Ready', throughput: '64 Cores Active' },
    { id: 'hadoop', name: 'Apache Hadoop HDFS', category: 'Distributed Storage', status: 'Replication 3x', throughput: '4.8 PB Capacity' },
    { id: 'flink', name: 'Apache Flink', category: 'Stream Processing', status: 'Low Latency Engine', throughput: 'sub-ms latency' },
    { id: 'airflow', name: 'Apache Airflow', category: 'Workflow Orchestration', status: 'DAG Scheduler Running', throughput: '142 Active DAGs' },
    { id: 'hive', name: 'Apache Hive', category: 'Data Warehouse', status: 'Metastore Online', throughput: 'SQL LLAP Ready' },
    { id: 'hbase', name: 'Apache HBase', category: 'NoSQL Big Table', status: 'RegionServers Online', throughput: '99.99% Availability' },
    { id: 'cassandra', name: 'Apache Cassandra', category: 'Distributed NoSQL', status: 'Ring Synchronized', throughput: 'Multi-DC Replication' },
    { id: 'nifi', name: 'Apache NiFi', category: 'Data Integration', status: 'Flows Processing', throughput: '8.4 GB/min' },
    { id: 'solr', name: 'Apache Solr', category: 'Enterprise Search', status: 'Cloud Shards Active', throughput: 'Full-text Indexing' },
    { id: 'superset', name: 'Apache Superset', category: 'Business Intelligence', status: 'Dashboards Serving', throughput: 'Visual Analytics' },
    { id: 'pulsar', name: 'Apache Pulsar', category: 'Pub-Sub Messaging', status: 'BookKeeper Connected', throughput: 'Georeplicated' },
    { id: 'iceberg', name: 'Apache Iceberg', category: 'Table Format', status: 'Catalog Synchronized', throughput: 'ACID Lakehouse' },
    { id: 'tomcat', name: 'Apache Tomcat', category: 'Servlet Container', status: 'Catalina Engine Running', throughput: 'HTTP/2 Connector' },
    { id: 'httpd', name: 'Apache HTTP Server', category: 'Web Server', status: 'Reverse Proxy Active', throughput: 'mod_proxy_balancer' },
    { id: 'arrow', name: 'Apache Arrow', category: 'In-Memory Data', status: 'Zero-Copy Shared', throughput: 'Columnar Vector' },
    { id: 'camel', name: 'Apache Camel', category: 'Integration Framework', status: 'Routes Active', throughput: 'EIP Pattern Engine' },
    { id: 'druid', name: 'Apache Druid', category: 'Real-time Analytics', status: 'Historical Nodes Online', throughput: 'Sub-second OLAP' },
  ];

  const currentToolInfo = apacheEcosystem.find(t => t.id === selectedTool) || apacheEcosystem[0];

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Apache Ecosystem & Big Data Super-Cluster Hub</h2>
          <p className="text-sm text-gray-400">Comprehensive management and integration adapter for all Apache software and Big Data frameworks.</p>
        </div>
        <div className="flex items-center space-x-2 bg-purple-500/10 border border-purple-500/30 text-purple-400 px-3 py-1.5 rounded-xl text-xs font-medium">
          <Layers className="w-4 h-4" />
          <span>18+ Apache Tools Integrated</span>
        </div>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3">
        {apacheEcosystem.map(tool => {
          const isSelected = selectedTool === tool.id;
          return (
            <div
              key={tool.id}
              onClick={() => setSelectedTool(tool.id)}
              className={`bg-cardbase border rounded-xl p-3.5 cursor-pointer transition-all ${
                isSelected 
                  ? 'border-purple-500 ring-2 ring-purple-500/20 bg-purple-500/5 shadow-lg shadow-purple-500/10' 
                  : 'border-gray-800 hover:border-gray-700'
              }`}
            >
              <div className="flex justify-between items-start mb-2">
                <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                <span className="text-[9px] font-mono uppercase text-gray-500 tracking-wider">Online</span>
              </div>
              <h3 className="font-bold text-xs text-white truncate">{tool.name}</h3>
              <p className="text-[11px] text-gray-400 mt-0.5 truncate">{tool.category}</p>
            </div>
          );
        })}
      </div>

      <div className="bg-cardbase border border-gray-800 rounded-2xl p-6">
        <div className="flex justify-between items-center mb-6">
          <div className="flex items-center space-x-3">
            <div className="w-12 h-12 rounded-xl bg-purple-500/10 text-purple-400 flex items-center justify-center">
              <Server className="w-6 h-6" />
            </div>
            <div>
              <div className="flex items-center space-x-2">
                <h3 className="font-bold text-xl text-white">{currentToolInfo.name}</h3>
                <span className="text-xs font-mono px-2 py-0.5 rounded bg-purple-500/10 text-purple-400">
                  {currentToolInfo.category}
                </span>
              </div>
              <p className="text-xs text-gray-400 mt-1">Status: <span className="text-emerald-400 font-semibold">{currentToolInfo.status}</span> | Metric: <span className="text-blue-400 font-mono">{currentToolInfo.throughput}</span></p>
            </div>
          </div>
          <div className="flex space-x-3">
            <button className="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white font-medium text-xs rounded-xl transition-all border border-gray-700">
              Restart Adapter
            </button>
            <button className="px-4 py-2 bg-purple-600 hover:bg-purple-500 text-white font-medium text-xs rounded-xl transition-all shadow-lg shadow-purple-600/20">
              Configure Cluster
            </button>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2 bg-gray-900/80 border border-gray-800 rounded-xl p-5 space-y-4">
            <h4 className="text-sm font-semibold text-white">Cluster Topology & Node Telemetry</h4>
            <div className="grid grid-cols-3 gap-3">
              <div className="bg-gray-950 p-3.5 rounded-lg border border-gray-800">
                <span className="text-xs text-gray-400">Node Instances</span>
                <p className="text-xl font-bold text-white mt-1">16 Nodes</p>
              </div>
              <div className="bg-gray-950 p-3.5 rounded-lg border border-gray-800">
                <span className="text-xs text-gray-400">Network Latency</span>
                <p className="text-xl font-bold text-emerald-400 mt-1">0.82 ms</p>
              </div>
              <div className="bg-gray-950 p-3.5 rounded-lg border border-gray-800">
                <span className="text-xs text-gray-400">Error Rate</span>
                <p className="text-xl font-bold text-white mt-1">0.00%</p>
              </div>
            </div>
            <div className="p-4 rounded-lg bg-gray-950 border border-gray-800 space-y-2">
              <div className="flex justify-between items-center text-xs">
                <span className="text-gray-400 font-mono">Adapter Protocol Bridge</span>
                <span className="text-purple-400 font-mono">gRPC / TLS 1.3 / Thrift</span>
              </div>
              <div className="flex justify-between items-center text-xs">
                <span className="text-gray-400 font-mono">Security Hardening</span>
                <span className="text-emerald-400 font-mono">Post-Quantum PFE-969 Encrypted</span>
              </div>
            </div>
          </div>

          <div className="bg-gray-900/80 border border-gray-800 rounded-xl p-5 flex flex-col justify-between">
            <div>
              <h4 className="text-sm font-semibold text-white mb-3">Enterprise Operations</h4>
              <ul className="space-y-2 text-xs text-gray-300">
                <li className="flex items-center space-x-2">
                  <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
                  <span>Automatic failover orchestration enabled</span>
                </li>
                <li className="flex items-center space-x-2">
                  <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
                  <span>Zero-leak frontend isolation enforced</span>
                </li>
                <li className="flex items-center space-x-2">
                  <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
                  <span>Real-time Prometheus metric exporter sink</span>
                </li>
              </ul>
            </div>
            <div className="mt-4 pt-4 border-t border-gray-800">
              <span className="text-[11px] text-gray-500 font-mono">IORI THE APACHER // APACHE SUITE v0.1.0</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
