import { ReactNode } from 'react';
import { Header } from './Header';
import { Footer } from './Footer';

interface CustomerLayoutProps {
  children: ReactNode;
  cartCount?: number;
  onNavigate?: (path: string) => void;
}

export function CustomerLayout({ children, cartCount = 0, onNavigate }: CustomerLayoutProps) {
  return (
    <div className="min-h-screen flex flex-col">
      <Header cartCount={cartCount} onNavigate={onNavigate} />
      <main className="flex-1">
        {children}
      </main>
      <Footer />
    </div>
  );
}
