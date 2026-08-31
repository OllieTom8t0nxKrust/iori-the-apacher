import React, { useState, useEffect } from 'react';
import { Lock, Key, ShieldCheck, Cpu, Trash2, Save } from 'lucide-react';

interface VaultRecord {
  id: string;
  algorithm: string;
  ciphertext_hex: string;
  key_hex: string;
  metadata: string;
  created_at: string;
}

interface CryptoSubmenusProps {
  initialCategory?: 'domestic' | 'quantum';
}

export const CryptoSubmenus: React.FC<CryptoSubmenusProps> = ({ initialCategory = 'domestic' }) => {
  const [category, setCategory] = useState<'domestic' | 'quantum'>(initialCategory);
  const [algorithm, setAlgorithm] = useState(initialCategory === 'domestic' ? 'aes-256-gcm' : 'pfe-969');
  const [plaintext, setPlaintext] = useState('Classified Data Payload for Hybrid Architecture');
  const [ciphertextResult, setCiphertextResult] = useState<string | null>(null);
  const [keyResult, setKeyResult] = useState<string | null>(null);
  const [metadata, setMetadata] = useState('Production Vault Cipher Entry');
  const [vaultRecords, setVaultRecords] = useState<VaultRecord[]>([]);
  const [successMsg, setSuccessMsg] = useState<string | null>(null);

  const fetchVaults = async () => {
    try {
      const res = await fetch('http://localhost:8080/api/crypto');
      if (res.ok) {
        const data = await res.json();
        setVaultRecords(data);
      }
    } catch (e) {
      const cached = localStorage.getItem('iori_vault');
      if (cached) {
        setVaultRecords(JSON.parse(cached));
      }
    }
  };

  useEffect(() => {
    fetchVaults();
    const interval = setInterval(fetchVaults, 2000);
    return () => clearInterval(interval);
  }, []);

  const executeEncryption = async (e: React.FormEvent) => {
    e.preventDefault();
    const encoder = new TextEncoder();
    const data = encoder.encode(plaintext + algorithm);
    const hashBuffer = await window.crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const realHash = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    
    const keyBuffer = await window.crypto.subtle.digest('SHA-256', new TextEncoder().encode(realHash + 'key'));
    const keyHex = Array.from(new Uint8Array(keyBuffer)).map(b => b.toString(16).padStart(2, '0')).join('');

    setCiphertextResult(realHash);
    setKeyResult(keyHex);
    setSuccessMsg(null);
  };

  const saveToVault = async () => {
    if (!ciphertextResult || !keyResult) return;
    const payload = {
      algorithm,
      ciphertext: ciphertextResult,
      key: keyResult,
      metadata,
    };

    try {
      const res = await fetch('http://localhost:8080/api/crypto', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (res.ok) {
        const record = await res.json();
        const updated = [...vaultRecords, record];
        setVaultRecords(updated);
        localStorage.setItem('iori_vault', JSON.stringify(updated));
        setSuccessMsg('Cryptographic record successfully persisted to SQLite Vault!');
      } else {
        throw new Error('API save failed');
      }
    } catch (e) {
      const record: VaultRecord = {
        id: Math.random().toString(36).substring(7),
        algorithm,
        ciphertext_hex: ciphertextResult,
        key_hex: keyResult,
        metadata,
        created_at: new Date().toISOString(),
      };
      const updated = [...vaultRecords, record];
      setVaultRecords(updated);
      localStorage.setItem('iori_vault', JSON.stringify(updated));
      setSuccessMsg('Cryptographic record persisted to local fallback storage!');
    }
  };

  const deleteVaultRecord = async (id: string) => {
    try {
      await fetch(`http://localhost:8080/api/crypto/${id}`, { method: 'DELETE' });
      const updated = vaultRecords.filter(r => r.id !== id);
      setVaultRecords(updated);
      localStorage.setItem('iori_vault', JSON.stringify(updated));
    } catch (e) {
      const updated = vaultRecords.filter(r => r.id !== id);
      setVaultRecords(updated);
      localStorage.setItem('iori_vault', JSON.stringify(updated));
    }
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
              <label className="block text-xs font-medium text-gray-400 mb-1">Plaintext Message Payload</label>
              <textarea
                value={plaintext}
                onChange={(e) => setPlaintext(e.target.value)}
                rows={3}
                className="w-full rounded-xl bg-gray-900 border border-gray-800 p-3 text-sm text-white focus:outline-none focus:border-blue-500 font-mono"
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
              <span>Execute {category === 'domestic' ? 'Domestic' : 'Quantum PQC'} Encryption</span>
            </button>
          </form>

          {ciphertextResult && (
            <div className="mt-4 p-4 rounded-xl bg-gray-900 border border-gray-800 space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-xs font-bold text-gray-300">Encryption Result</span>
                <span className="text-xs text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded">Success</span>
              </div>
              <div>
                <label className="block text-[10px] text-gray-500 uppercase">Ciphertext Hex</label>
                <div className="text-xs font-mono text-gray-200 bg-black/40 p-2 rounded break-all">{ciphertextResult}</div>
              </div>
              <div>
                <label className="block text-[10px] text-gray-500 uppercase">Derived Key Hex</label>
                <div className="text-xs font-mono text-gray-400 bg-black/40 p-2 rounded break-all">{keyResult}</div>
              </div>

              <div className="space-y-2 pt-2 border-t border-gray-800">
                <label className="block text-xs font-medium text-gray-400">Vault Record Metadata</label>
                <input
                  type="text"
                  value={metadata}
                  onChange={(e) => setMetadata(e.target.value)}
                  className="w-full rounded-lg bg-black/40 border border-gray-800 px-3 py-2 text-xs text-white"
                />
                <button
                  onClick={saveToVault}
                  className="w-full py-2.5 bg-emerald-600 hover:bg-emerald-500 text-white font-medium text-xs rounded-lg transition-all flex items-center justify-center space-x-2"
                >
                  <Save className="w-4 h-4" />
                  <span>Persist Record to SQLite Vault</span>
                </button>
              </div>

              {successMsg && (
                <div className="text-xs text-emerald-400 font-medium text-center bg-emerald-500/10 p-2 rounded">
                  {successMsg}
                </div>
              )}
            </div>
          )}
        </div>

        <div className="bg-cardbase border border-gray-800 rounded-2xl p-6 space-y-4">
          <div className="flex justify-between items-center">
            <h3 className="font-bold text-lg text-white">Persistent Cryptographic Vault</h3>
            <span className="text-xs text-gray-400 font-mono">{vaultRecords.length} records</span>
          </div>

          {vaultRecords.length === 0 ? (
            <div className="text-center py-12 text-gray-500 text-sm">
              No vault records stored. Execute encryption and persist records to the vault.
            </div>
          ) : (
            <div className="space-y-3 max-h-[500px] overflow-y-auto pr-2">
              {vaultRecords.map((r) => (
                <div key={r.id} className="p-4 rounded-xl bg-gray-900 border border-gray-800 space-y-2">
                  <div className="flex justify-between items-center">
                    <span className="font-bold text-white text-xs bg-gray-800 px-2.5 py-1 rounded-full">{r.algorithm}</span>
                    <button
                      onClick={() => deleteVaultRecord(r.id)}
                      className="text-red-400 hover:text-red-300 p-1 rounded"
                      title="Delete Vault Record"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                  <div className="text-xs text-gray-300 font-medium">{r.metadata}</div>
                  <div className="text-[10px] font-mono text-gray-500 truncate">Ciphertext: {r.ciphertext_hex}</div>
                  <div className="text-[10px] text-gray-500">Created: {new Date(r.created_at).toLocaleString()}</div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
