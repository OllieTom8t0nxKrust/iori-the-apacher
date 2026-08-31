import React, { useState, useEffect } from 'react';
import { Zap } from 'lucide-react';

export const CabocloIoriLogo: React.FC<{ collapsed?: boolean }> = ({ collapsed = false }) => {
  const [energized, setEnergized] = useState(false);
  const [flashLightning, setFlashLightning] = useState(false);

  useEffect(() => {
    // Trigger lightning strike and energize every 30 seconds
    const interval = setInterval(() => {
      setFlashLightning(true);
      setEnergized(true);

      setTimeout(() => {
        setFlashLightning(false);
      }, 900); // lightning flash duration

      setTimeout(() => {
        setEnergized(false);
      }, 4000); // total energized duration
    }, 30000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="relative flex items-center justify-center">
      {/* Lightning Bolt From Above */}
      {flashLightning && !collapsed && (
        <div className="absolute -top-14 left-1/2 transform -translate-x-1/2 z-30 pointer-events-none animate-bounce">
          <Zap className="w-10 h-10 text-cyan-300 drop-shadow-[0_0_18px_rgba(6,182,212,0.95)]" />
        </div>
      )}

      {/* Logo Image Container */}
      <div className={`relative w-12 h-12 rounded-2xl transition-all duration-700 flex items-center justify-center overflow-hidden border-2 ${
        collapsed
          ? 'bg-black border-white'
          : energized
          ? 'border-cyan-400 shadow-[0_0_35px_rgba(6,182,212,0.9)] scale-110 brightness-125'
          : 'border-amber-500/50 shadow-lg shadow-amber-600/30'
      }`}>
        {collapsed ? (
            <svg viewBox="0 0 24 24" className="w-8 h-8 text-white">
                <path fill="currentColor" d="M12 2C8.69 2 6 4.69 6 8c0 2.13 1.15 4 2.87 5.04L8 16c0 1.1.9 2 2 2h4c1.1 0 2-.9 2-2l-.87-2.96C16.85 12 18 10.13 18 8c0-3.31-2.69-6-6-6zm0 10c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z"/>
                <circle cx="9.5" cy="8" r="1" className="fill-red-500 animate-pulse" />
                <circle cx="14.5" cy="8" r="1" className="fill-red-500 animate-pulse" />
            </svg>
        ) : (
        <img
          src="/caboclo-iori.jpg"
          alt="Caboclo-Iori Logo"
          className="w-full h-full object-cover"
        />
        )}

        {/* Raiden Lightning Bolt Glow Overlay when energized */}
        {energized && !collapsed && (
          <div className="absolute inset-0 bg-cyan-400/30 mix-blend-color-dodge flex items-center justify-center pointer-events-none">
            <div className="w-full h-full border-2 border-cyan-200 animate-ping rounded-xl opacity-90"></div>
          </div>
        )}
      </div>
    </div>
  );
};
