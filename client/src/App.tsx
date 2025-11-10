import { useState } from 'react';
import { ApolloProvider } from '@apollo/client';
import { apolloClient } from './lib/apollo-client';
import { Home } from './pages/customer/Home';
import { AdminDashboard } from './pages/AdminDashboard';
import { VendorDashboardNew } from './pages/VendorDashboardNew';
import { Login } from './pages/Login';

type View = 'home' | 'admin' | 'vendor' | 'login' | string;

function App() {
  const [currentView, setCurrentView] = useState<View>('home');
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  const handleNavigate = (path: View) => {
    console.log('Navigating to:', path);
    setCurrentView(path);
  };

  const renderView = () => {
    // Handle dynamic routes
    if (currentView.startsWith('/product/') ||
        currentView.startsWith('/category/') ||
        currentView.startsWith('/cart') ||
        currentView.startsWith('/account/')) {
      // TODO: Implement routing for these pages
      return <Home onNavigate={handleNavigate} />;
    }

    switch (currentView) {
      case 'home':
      case '/':
        return <Home onNavigate={handleNavigate} />;
      case 'admin':
        return <AdminDashboard onNavigate={handleNavigate} />;
      case 'vendor':
        return <VendorDashboardNew onNavigate={handleNavigate} />;
      case 'login':
        return <Login onNavigate={handleNavigate} onLogin={() => setIsAuthenticated(true)} />;
      default:
        return <Home onNavigate={handleNavigate} />;
    }
  };

  return (
    <ApolloProvider client={apolloClient}>
      <div className="min-h-screen bg-background">
        {renderView()}
      </div>
    </ApolloProvider>
  );
}

export default App;
