import { Store, ShoppingCart, User, Search } from 'lucide-react';

interface HomeProps {
  onNavigate: (view: 'home' | 'admin' | 'vendor' | 'login') => void;
}

export function Home({ onNavigate }: HomeProps) {
  return (
    <div className="min-h-screen">
      {/* Header */}
      <header className="border-b">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Store className="h-8 w-8 text-primary" />
              <h1 className="text-2xl font-bold">eCommerce Platform</h1>
            </div>
            <div className="flex items-center gap-4">
              <button
                onClick={() => onNavigate('login')}
                className="flex items-center gap-2 px-4 py-2 rounded-md hover:bg-secondary"
              >
                <User className="h-5 w-5" />
                Sign In
              </button>
              <button className="flex items-center gap-2 px-4 py-2 rounded-md hover:bg-secondary">
                <ShoppingCart className="h-5 w-5" />
                Cart (0)
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section className="bg-gradient-to-r from-primary/10 to-secondary/10 py-20">
        <div className="container mx-auto px-4">
          <div className="max-w-3xl mx-auto text-center">
            <h2 className="text-5xl font-bold mb-6">
              Welcome to Your Multi-Vendor Marketplace
            </h2>
            <p className="text-xl text-muted-foreground mb-8">
              Discover thousands of products from verified vendors worldwide
            </p>
            <div className="flex gap-4 max-w-2xl mx-auto">
              <div className="flex-1 relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-muted-foreground" />
                <input
                  type="text"
                  placeholder="Search for products..."
                  className="w-full pl-10 pr-4 py-3 rounded-lg border bg-background"
                />
              </div>
              <button className="px-8 py-3 bg-primary text-primary-foreground rounded-lg font-semibold hover:bg-primary/90">
                Search
              </button>
            </div>
          </div>
        </div>
      </section>

      {/* Quick Links */}
      <section className="py-16">
        <div className="container mx-auto px-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div
              onClick={() => onNavigate('home')}
              className="p-8 border rounded-lg hover:shadow-lg transition-shadow cursor-pointer"
            >
              <Store className="h-12 w-12 text-primary mb-4" />
              <h3 className="text-2xl font-bold mb-2">Shop</h3>
              <p className="text-muted-foreground">
                Browse our extensive collection of products
              </p>
            </div>
            <div
              onClick={() => onNavigate('vendor')}
              className="p-8 border rounded-lg hover:shadow-lg transition-shadow cursor-pointer"
            >
              <User className="h-12 w-12 text-primary mb-4" />
              <h3 className="text-2xl font-bold mb-2">Vendor Panel</h3>
              <p className="text-muted-foreground">
                Manage your store and products
              </p>
            </div>
            <div
              onClick={() => onNavigate('admin')}
              className="p-8 border rounded-lg hover:shadow-lg transition-shadow cursor-pointer"
            >
              <ShoppingCart className="h-12 w-12 text-primary mb-4" />
              <h3 className="text-2xl font-bold mb-2">Admin Panel</h3>
              <p className="text-muted-foreground">
                Manage the entire marketplace
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 bg-muted/50">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl font-bold text-center mb-12">Platform Features</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {[
              { title: 'Multi-Vendor', description: 'Support for multiple vendors' },
              { title: 'GraphQL API', description: 'Modern and efficient API' },
              { title: 'Secure Payments', description: 'Stripe, PayPal, Razorpay' },
              { title: 'Real-time Updates', description: 'Live order tracking' },
              { title: '4-Level Categories', description: 'Organized product hierarchy' },
              { title: 'Product Ratings', description: 'Customer reviews & ratings' },
              { title: 'Vendor Dashboard', description: 'Complete vendor management' },
              { title: 'Admin Controls', description: 'Full platform administration' },
            ].map((feature) => (
              <div key={feature.title} className="p-6 bg-background rounded-lg border">
                <h3 className="font-semibold mb-2">{feature.title}</h3>
                <p className="text-sm text-muted-foreground">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t py-8">
        <div className="container mx-auto px-4 text-center text-muted-foreground">
          <p>&copy; 2024 eCommerce Platform. Built with NestJS, GraphQL, React & TypeScript.</p>
        </div>
      </footer>
    </div>
  );
}
