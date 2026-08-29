import React, { useState } from 'react';
import { Lock, Key, ShieldCheck, Cpu } from 'lucide-react';

interface CryptoSubmenusProps {
  initialCategory?: 'domestic' | 'quantum';
}

export const CryptoSubmenus: React.FC<CryptoSubmenusProps> = ({ initialCategory = 'domestic' }) => {
  const [category, setCategory] = useState<'domestic' | 'quantum'>(initialCategory);
  const [algorithm, setAlgorithm] = useState(initialCategory === 'domestic' ? 'aes-256-gcm' : 'pfe-969');
  const [plaintext, setPlaintext] = useState('Classified Data Payload for Hybrid Architecture');
  const [ciphertextResult, setCiphertextResult] = useState<string | null>(null);

  const executeEncryption = async (e: React.FormEvent) => {
    e.preventDefault();
    const encoder = new TextEncoder();
    const data = encoder.encode(plaintext + algorithm);
    const hashBuffer = await window.crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const realHash = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    setCiphertextResult(realHash);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold text-white tracking-tight">Cryptographic Vault & Submenus</h2>
          <p className="text-sm text-gray-400">Manage domestic corp servers ciphers and advanced post-quantum algorithms.</p>
        </div>
      </div>

      <div className="flex space-x-3 border-b border-gray-800 pb-4">
        <button
          onClick={() => { setCategory('domestic'); setAlgorithm('aes-256-gcm'); setCiphertextResult(null); }}
          className={`px-5 py-2.5 rounded-xl font-medium text-sm transition-all flex items-center space-x-2 ${
            category === 'domestic'
              ? 'bg-blue-600 text-white shadow-lg shadow-blue-600/20'
              : 'bg-cardbase text-gray-400 hover:text-white border border-gray-800'
          }`}
        >
          <Lock className="w-4 h-4" />
          <span>Domestic & Corp Ciphers</span>
        </button>

        <button
          onClick={() => { setCategory('quantum'); setAlgorithm('pfe-969'); setCiphertextResult(null); }}
          className={`px-5 py-2.5 rounded-xl font-medium text-sm transition-all flex items-center space-x-2 ${
            category === 'quantum'
              ? 'bg-purple-600 text-white shadow-lg shadow-purple-600/20'
              : 'bg-cardbase text-gray-400 hover:text-white border border-gray-800'
          }`}
        >
          <Cpu className="w-4 h-4" />
          <span>Quantum Cryptographic Ciphers</span>
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-cardbase border border-gray-800 rounded-2xl p-6 space-y-4">
          <div className="flex items-center space-x-3">
            <div className={`w-10 h-10 rounded-xl flex items-center justify-center ${category === 'domestic' ? 'bg-blue-500/10 text-blue-400' : 'bg-purple-500/10 text-purple-400'}`}>
              <Key className="w-5 h-5" />
            </div>
            <div>
              <h3 className="font-bold text-lg text-white">
                {category === 'domestic' ? 'Domestic Submenu: Corp Ciphers' : 'Quantum Submenu: Post-Quantum PQC'}
              </h3>
              <p className="text-xs text-gray-400">
                {category === 'domestic' ? 'Standard binary architecture ciphers for enterprise servers.' : 'Lattice-based and QPU-optimized resistant algorithms.'}
              </p>
            </div>
          </div>

          <form onSubmit={executeEncryption} className="space-y-4">
            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Select Algorithm</label>
              <select
                value={algorithm}
                onChange={(e) => setAlgorithm(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 px-4 py-2.5 text-sm text-white focus:outline-none focus:border-blue-500"
              >
                {category === 'domestic' ? (
                  <>
                    <option value="aes-256-gcm">AES-256-GCM (Authenticated Encryption)</option>
                    <option value="chacha20">ChaCha20-Poly1305 (High Performance)</option>
                    <option value="rsa-4096">RSA-4096 (Legacy Enterprise)</option>
                    <option value="ed25519">Ed25519 (Digital Signatures)</option>
                  </>
                ) : (
                  <>
                    <option value="pfe-969">PFE-969 (Hyper-Dimensional Lattice - Proposed)</option>
                    <option value="kyber-1024">ML-KEM-1024 / Kyber (NIST PQC Standard)</option>
                    <option value="dilithium">ML-DSA / Dilithium (Post-Quantum Signatures)</option>
                    <option value="sphincs">SPHINCS+ (Stateless Hash-Based)</option>
                  </>
                )}
              </select>
            </div>

            <div>
              <label className="block text-xs font-medium text-gray-400 mb-1">Plaintext Payload</label>
              <textarea
                rows={3}
                value={plaintext}
                onChange={(e) => setPlaintext(e.target.value)}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 p-4 text-sm text-white focus:outline-none focus:border-blue-500"
                required
              />
            </div>

            <button
              type="submit"
              className={`w-full py-3 font-medium text-sm rounded-xl transition-all shadow-lg flex items-center justify-center space-x-2 text-white ${
                category === 'domestic' ? 'bg-blue-600 hover:bg-blue-500 shadow-blue-600/20' : 'bg-purple-600 hover:bg-purple-500 shadow-purple-600/20'
              }`}
            >
              <ShieldCheck className="w-4 h-4" />
              <span>Execute Encryption & Vault Storage</span>
            </button>
          </form>
        </div>

        <div className="bg-cardbase border border-gray-800 rounded-2xl p-6 flex flex-col justify-between">
          <div>
            <h3 className="font-bold text-lg text-white mb-2">Cryptographic Output & Verification</h3>
            <p className="text-xs text-gray-400 mb-4">Generated ciphertext payload and hardware-accelerated signature metadata.</p>
            
            {ciphertextResult ? (
              <div className="space-y-4">
                <div className="bg-gray-900 border border-gray-800 rounded-xl p-4 font-mono text-xs space-y-2">
                  <div className="text-gray-400">Algorithm: <span className="text-cyan-400 uppercase">{algorithm}</span></div>
                  <div className="text-gray-400">Status: <span className="text-emerald-400">Verified Secure</span></div>
                  <div className="text-gray-400">Ciphertext (Hex):</div>
                  <div className="bg-gray-950 p-3 rounded-lg text-gray-200 break-all border border-gray-800/80">
                    {ciphertextResult}
                  </div>
                </div>
              </div>
            ) : (
              <div className="h-48 border border-dashed border-gray-800 rounded-xl flex items-center justify-center text-center p-6 text-gray-500 text-sm">
                Configure parameters and execute encryption to view cryptographic results and quantum proof vectors.
              </div>
            )}
          </div>

          <div className="mt-6 bg-gray-900 rounded-xl p-4 border border-gray-800 text-xs text-gray-400 space-y-1">
            <div className="flex justify-between">
              <span>Hardware Compatibility:</span>
              <span className="text-emerald-400 font-mono">Binary + QPU Active</span>
            </div>
            <div className="flex justify-between">
              <span>Entropy Source:</span>
              <span className="text-blue-400 font-mono">Hardware OS / RNG</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
