import { useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { DashboardOverview } from './components/DashboardOverview';
import { TunnelManager } from './components/TunnelManager';
import { ServerLaunchManager } from './components/ServerLaunchManager';
import { CryptoSubmenus } from './components/CryptoSubmenus';
import { Pfe969Monitor } from './components/Pfe969Monitor';
import { CliTerminal } from './components/CliTerminal';

export function App() {
  const [activeTab, setActiveTab] = useState('dashboard');

  const renderContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return <DashboardOverview />;
      case 'tunnels':
        return <TunnelManager />;
      case 'servers':
        return <ServerLaunchManager />;
      case 'crypto-vault':
        return <CryptoSubmenus />;
      case 'pfe969':
        return <Pfe969Monitor />;
      case 'terminal':
        return <CliTerminal />;
      default:
        return <DashboardOverview />;
    }
  };

  return (
    <div className="flex min-h-screen bg-darkbase">
      <Sidebar activeTab={activeTab} setActiveTab={setActiveTab} />
      <main className="flex-1 p-8 overflow-y-auto">
        <div className="max-w-7xl mx-auto">
          {renderContent()}
        </div>
      </main>
    </div>
  );
}

export default App;
