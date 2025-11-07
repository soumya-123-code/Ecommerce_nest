import { useState } from 'react';
import { Store, Mail, Lock } from 'lucide-react';

interface LoginProps {
  onNavigate: (view: 'home' | 'admin' | 'vendor' | 'login') => void;
  onLogin: () => void;
}

export function Login({ onNavigate, onLogin }: LoginProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // TODO: Implement actual login logic with GraphQL
    onLogin();
    onNavigate('home');
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary/5 to-secondary/5">
      <div className="w-full max-w-md p-8 bg-background rounded-lg shadow-lg border">
        <div className="text-center mb-8">
          <div className="flex justify-center mb-4">
            <Store className="h-12 w-12 text-primary" />
          </div>
          <h1 className="text-3xl font-bold">Welcome Back</h1>
          <p className="text-muted-foreground mt-2">Sign in to your account</p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-2">Email</label>
            <div className="relative">
              <Mail className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-muted-foreground" />
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="w-full pl-10 pr-4 py-2 border rounded-md bg-background"
                placeholder="your@email.com"
                required
              />
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">Password</label>
            <div className="relative">
              <Lock className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-muted-foreground" />
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="w-full pl-10 pr-4 py-2 border rounded-md bg-background"
                placeholder="••••••••"
                required
              />
            </div>
          </div>

          <button
            type="submit"
            className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-semibold hover:bg-primary/90"
          >
            Sign In
          </button>
        </form>

        <div className="mt-6 text-center">
          <button
            onClick={() => onNavigate('home')}
            className="text-sm text-muted-foreground hover:text-foreground"
          >
            Back to Home
          </button>
        </div>
      </div>
    </div>
  );
}
