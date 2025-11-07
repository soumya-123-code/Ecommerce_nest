import { Home, Package, Users, ShoppingCart, Settings, BarChart3 } from 'lucide-react';

interface AdminDashboardProps {
  onNavigate: (view: 'home' | 'admin' | 'vendor' | 'login') => void;
}

export function AdminDashboard({ onNavigate }: AdminDashboardProps) {
  const stats = [
    { label: 'Total Revenue', value: '$45,231.89', change: '+20.1%', icon: BarChart3 },
    { label: 'Total Orders', value: '2,543', change: '+12.5%', icon: ShoppingCart },
    { label: 'Active Vendors', value: '156', change: '+8.2%', icon: Users },
    { label: 'Products Listed', value: '8,429', change: '+15.3%', icon: Package },
  ];

  return (
    <div className="min-h-screen bg-muted/20">
      {/* Header */}
      <header className="bg-background border-b">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold">Admin Dashboard</h1>
            <button
              onClick={() => onNavigate('home')}
              className="flex items-center gap-2 px-4 py-2 rounded-md hover:bg-secondary"
            >
              <Home className="h-5 w-5" />
              Back to Home
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div className="container mx-auto px-4 py-8">
        {/* Stats Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          {stats.map((stat) => {
            const Icon = stat.icon;
            return (
              <div key={stat.label} className="bg-background p-6 rounded-lg border shadow-sm">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm font-medium text-muted-foreground">{stat.label}</span>
                  <Icon className="h-5 w-5 text-muted-foreground" />
                </div>
                <div className="text-2xl font-bold mb-1">{stat.value}</div>
                <div className="text-sm text-green-600">{stat.change} from last month</div>
              </div>
            );
          })}
        </div>

        {/* Management Sections */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <Users className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">User Management</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              Manage customers, vendors, and user permissions
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              Manage Users
            </button>
          </div>

          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <Package className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">Product Management</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              Review and manage all products and categories
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              Manage Products
            </button>
          </div>

          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <ShoppingCart className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">Order Management</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              Track and manage all platform orders
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              View Orders
            </button>
          </div>

          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <BarChart3 className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">Analytics & Reports</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              View detailed analytics and generate reports
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              View Analytics
            </button>
          </div>

          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <Settings className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">Platform Settings</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              Configure site settings and payment gateways
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              Settings
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
