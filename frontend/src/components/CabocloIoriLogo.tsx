import React, { useState, useEffect } from 'react';
import { Zap } from 'lucide-react';

interface CabocloIoriLogoProps {
  collapsed?: boolean;
}

export const CabocloIoriLogo: React.FC<CabocloIoriLogoProps> = ({ collapsed = false }) => {
  const [energized, setEnergized] = useState(true);
  const [flashLightning, setFlashLightning] = useState(true);

  // Trigger lightning flash on mount & periodically
  useEffect(() => {
    const timer = setTimeout(() => {
      setFlashLightning(false);
    }, 1200);

    const energizeTimer = setTimeout(() => {
      setEnergized(false);
    }, 3500);

    const interval = setInterval(() => {
      setFlashLightning(true);
      setEnergized(true);

      setTimeout(() => {
        setFlashLightning(false);
      }, 1000);

      setTimeout(() => {
        setEnergized(false);
      }, 4000);
    }, 12000);

    return () => {
      clearTimeout(timer);
      clearTimeout(energizeTimer);
      clearInterval(interval);
    };
  }, []);

  return (
    <div className="relative flex items-center justify-center group cursor-pointer">
      {/* Blue Lightning Bolt Top Flash - Active in BOTH collapsed & expanded modes */}
      {flashLightning && (
        <div className={`absolute z-30 pointer-events-none animate-lightning ${
          collapsed ? '-top-10' : '-top-12'
        }`}>
          <Zap className={`${collapsed ? 'w-8 h-8' : 'w-12 h-12'} text-cyan-300 drop-shadow-[0_0_22px_rgba(6,182,212,1)]`} />
        </div>
      )}

      {/* Secondary Electric Sparks Overlay */}
      {energized && (
        <>
          <div className="absolute -left-3 top-1/2 -translate-y-1/2 z-30 pointer-events-none animate-pulse">
            <Zap className={`${collapsed ? 'w-4 h-4' : 'w-6 h-6'} text-cyan-400 drop-shadow-[0_0_12px_rgba(6,182,212,0.9)]`} />
          </div>
          <div className="absolute -right-3 top-1/2 -translate-y-1/2 z-30 pointer-events-none animate-pulse">
            <Zap className={`${collapsed ? 'w-4 h-4' : 'w-6 h-6'} text-cyan-400 drop-shadow-[0_0_12px_rgba(6,182,212,0.9)]`} />
          </div>
        </>
      )}

      {/* Black Square Logo Container */}
      <div className={`relative bg-black ${
        collapsed ? 'w-28 h-28' : 'w-44 h-44'
      } rounded-2xl transition-all duration-700 ease-in-out flex items-center justify-center overflow-hidden border-2 ${
        energized
          ? 'border-cyan-400 shadow-[0_0_40px_rgba(6,182,212,0.95)] scale-105 brightness-110 ring-4 ring-cyan-500/40 animate-electric'
          : 'border-amber-500/80 shadow-[0_0_25px_rgba(245,158,11,0.4)] group-hover:border-cyan-400 group-hover:shadow-[0_0_30px_rgba(6,182,212,0.7)]'
      }`}>
        {/* Expanded Image: Caboclo-Iori (Enlarged & High Visibility inside black square) */}
        <img
          src="/caboclo-iori.jpg"
          alt="Caboclo-Iori"
          className={`absolute inset-0 w-full h-full object-cover object-top scale-125 transition-all duration-700 ease-in-out ${
            collapsed ? 'opacity-0 scale-95 rotate-6' : 'opacity-100 scale-125 rotate-0'
          } filter contrast-125 brightness-110`}
        />

        {/* Collapsed Image: Caboclo-Pena-Branca (Object-contain to show whole cockade & attributes) */}
        <img
          src="/caboclo-pena-branca.png"
          alt="Caboclo-Pena-Branca"
          className={`absolute inset-0 w-full h-full object-contain object-center transition-all duration-700 ease-in-out ${
            collapsed ? 'opacity-100 scale-100 rotate-0' : 'opacity-0 scale-95 -rotate-6'
          } filter contrast-125 brightness-110`}
        />

        {/* Blue Lightning Glow Overlay when energized */}
        {energized && (
          <div className="absolute inset-0 bg-cyan-400/20 mix-blend-color-dodge flex items-center justify-center pointer-events-none">
            <div className="w-full h-full border-2 border-cyan-300 animate-ping rounded-2xl opacity-80"></div>
          </div>
        )}
      </div>
    </div>
  );
};
