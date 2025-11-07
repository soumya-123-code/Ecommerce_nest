import { useState } from 'react';
import { Home } from './pages/Home';
import { AdminDashboard } from './pages/AdminDashboard';
import { VendorDashboard } from './pages/VendorDashboard';
import { Login } from './pages/Login';

type View = 'home' | 'admin' | 'vendor' | 'login';

function App() {
  const [currentView, setCurrentView] = useState<View>('home');
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const renderView = () => {
    switch (currentView) {
      case 'home':
        return <Home onNavigate={setCurrentView} />;
      case 'admin':
        return <AdminDashboard onNavigate={setCurrentView} />;
      case 'vendor':
        return <VendorDashboard onNavigate={setCurrentView} />;
      case 'login':
        return <Login onNavigate={setCurrentView} onLogin={() => setIsAuthenticated(true)} />;
      default:
        return <Home onNavigate={setCurrentView} />;
    }
  };

  return (
    <div className="min-h-screen bg-background">
      {renderView()}
    </div>
  );
}

export default App;
