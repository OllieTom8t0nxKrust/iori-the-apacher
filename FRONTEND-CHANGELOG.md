# Frontend Changelog - IORI THE APACHER // Web Management Console

All notable changes to the React/TypeScript/Vite web management console panel are documented in this file.

## [1.2.0] - 2026-08-31

### Added
- **Collapsible Sidebar**: Implemented sidebar collapsing/expanding functionality.
- **Logo Transformation**: Logo now acts as the toggle button. When collapsed, it transforms into a white skull with glowing red eyes.
- **Responsiveness**: Improved UI responsiveness to ensure a seamless experience on all smartphone screen sizes.

## [1.1.0] - 2026-08-30

### Removed
- **Forensic Intelligence UI**: Removed `ForensicTracker.tsx` and associated tracking submenus to align with the core security suite focus on secure tunneling and post-quantum cryptography.

### Added & Refined
- Official Caboclo-Iori photo logo integration with automated 30-second lightning bolt energization and Raiden eyes glowing animation (`CabocloIoriLogo.tsx`).
- Extracted Caboclo-Iori color palette applied across Tailwind configuration and UI components.
- Consolidated single "Crypto Vault" menu replacing redundant separate crypto tabs.
- Integration with backend REST API for real-time tunnel management, server launch monitoring, and post-quantum cryptographic vault execution.

## [1.0.0] - 2026-08-28

### Added
- Initial setup of React + Vite + TypeScript frontend application.
- Modern dark-mode glassmorphism UI inspired by NGROK control panel.
- Submenu navigation for Tunnels, Server Launch Manager, and Quantum Ciphers (including PFE-969 live cipher validator & metrics).
- Interactive dashboard statistics and live tunnel status indicators.
