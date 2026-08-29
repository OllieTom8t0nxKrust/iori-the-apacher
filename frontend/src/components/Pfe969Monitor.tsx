import React from 'react';
import { Cpu, Atom, Shield, ArrowUpRight } from 'lucide-react';

export const Pfe969Monitor: React.FC = () => {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">PFE-969 Quantum Lattice Core Monitor</h2>
          <p className="text-sm text-gray-400">Advanced mathematical proof, LWE ring modules, and quantum security metrics.</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
        <div className="bg-cardbase border border-gray-800 rounded-2xl p-5">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 rounded-xl bg-purple-500/10 text-purple-400 flex items-center justify-center">
              <Atom className="w-5 h-5" />
            </div>
            <div>
              <h4 className="text-sm font-semibold text-white">Lattice Dimension (n)</h4>
              <p className="text-xs text-gray-400">Polynomial ring degree</p>
            </div>
          </div>
          <div className="text-3xl font-bold text-white font-mono">2048</div>
        </div>

        <div className="bg-cardbase border border-gray-800 rounded-2xl p-5">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 rounded-xl bg-cyan-500/10 text-cyan-400 flex items-center justify-center">
              <Cpu className="w-5 h-5" />
            </div>
            <div>
              <h4 className="text-sm font-semibold text-white">Modulus Prime (q)</h4>
              <p className="text-xs text-gray-400">Ring coefficient space</p>
            </div>
          </div>
          <div className="text-3xl font-bold text-white font-mono">8,380,417</div>
        </div>

        <div className="bg-cardbase border border-gray-800 rounded-2xl p-5">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 rounded-xl bg-emerald-500/10 text-emerald-400 flex items-center justify-center">
              <Shield className="w-5 h-5" />
            </div>
            <div>
              <h4 className="text-sm font-semibold text-white">Security Hardness</h4>
              <p className="text-xs text-gray-400">Shortest Vector Problem</p>
            </div>
          </div>
          <div className="text-3xl font-bold text-emerald-400 font-mono">NP-Hard</div>
        </div>
      </div>

      <div className="bg-cardbase border border-gray-800 rounded-2xl p-6 space-y-4">
        <h3 className="font-bold text-lg text-white">Mathematical & Physical Specification Summary</h3>
        <p className="text-sm text-gray-400 leading-relaxed">
          PFE-969 is designed to withstand both polynomial-time quantum attacks (such as Shor's algorithm) and exponential speedups on unstructured search (Grover's algorithm). By utilizing Learning With Errors (LWE) over polynomial rings R_q = Z_q[x] / (x^n + 1), PFE-969 guarantees absolute cryptographic longevity for domestic and QPU research infrastructures.
        </p>
        <div className="pt-2 flex items-center space-x-4">
          <a
            href="#study"
            className="px-4 py-2 bg-purple-600 hover:bg-purple-500 text-white text-xs font-medium rounded-xl transition-all shadow-lg shadow-purple-600/20 flex items-center space-x-1.5"
          >
            <span>Read PFE-969-CRYPTO-CYPHER-STUDY.md</span>
            <ArrowUpRight className="w-3.5 h-3.5" />
          </a>
        </div>
      </div>
    </div>
  );
};
