import { Home, Package, ShoppingCart, DollarSign, Plus } from 'lucide-react';

interface VendorDashboardProps {
  onNavigate: (view: 'home' | 'admin' | 'vendor' | 'login') => void;
}

export function VendorDashboard({ onNavigate }: VendorDashboardProps) {
  const stats = [
    { label: 'Total Sales', value: '$12,345.67', icon: DollarSign },
    { label: 'Active Products', value: '42', icon: Package },
    { label: 'Pending Orders', value: '8', icon: ShoppingCart },
    { label: 'Total Orders', value: '156', icon: ShoppingCart },
  ];

  return (
    <div className="min-h-screen bg-muted/20">
      {/* Header */}
      <header className="bg-background border-b">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold">Vendor Dashboard</h1>
            <div className="flex items-center gap-4">
              <button className="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
                <Plus className="h-5 w-5" />
                Add Product
              </button>
              <button
                onClick={() => onNavigate('home')}
                className="flex items-center gap-2 px-4 py-2 rounded-md hover:bg-secondary"
              >
                <Home className="h-5 w-5" />
                Back to Home
              </button>
            </div>
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
                <div className="text-2xl font-bold">{stat.value}</div>
              </div>
            );
          })}
        </div>

        {/* Quick Actions */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <Package className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">My Products</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              Manage your product listings and inventory
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              View Products
            </button>
          </div>

          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <ShoppingCart className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">Orders</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              Track and manage your customer orders
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              View Orders
            </button>
          </div>

          <div className="bg-background p-6 rounded-lg border shadow-sm">
            <div className="flex items-center gap-3 mb-4">
              <DollarSign className="h-8 w-8 text-primary" />
              <h2 className="text-xl font-semibold">Earnings</h2>
            </div>
            <p className="text-muted-foreground mb-4">
              View your earnings and payment history
            </p>
            <button className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90">
              View Earnings
            </button>
          </div>
        </div>

        {/* Recent Orders Table */}
        <div className="mt-8 bg-background rounded-lg border shadow-sm">
          <div className="p-6 border-b">
            <h2 className="text-xl font-semibold">Recent Orders</h2>
          </div>
          <div className="p-6">
            <table className="w-full">
              <thead>
                <tr className="text-left border-b">
                  <th className="pb-3 font-semibold">Order ID</th>
                  <th className="pb-3 font-semibold">Product</th>
                  <th className="pb-3 font-semibold">Customer</th>
                  <th className="pb-3 font-semibold">Amount</th>
                  <th className="pb-3 font-semibold">Status</th>
                </tr>
              </thead>
              <tbody>
                {[
                  { id: '#ORD-001', product: 'Product Name 1', customer: 'John Doe', amount: '$99.99', status: 'Pending' },
                  { id: '#ORD-002', product: 'Product Name 2', customer: 'Jane Smith', amount: '$149.99', status: 'Shipped' },
                  { id: '#ORD-003', product: 'Product Name 3', customer: 'Bob Johnson', amount: '$79.99', status: 'Delivered' },
                ].map((order) => (
                  <tr key={order.id} className="border-b last:border-0">
                    <td className="py-3 font-medium">{order.id}</td>
                    <td className="py-3">{order.product}</td>
                    <td className="py-3">{order.customer}</td>
                    <td className="py-3">{order.amount}</td>
                    <td className="py-3">
                      <span
                        className={`px-2 py-1 rounded-full text-xs font-medium ${
                          order.status === 'Pending'
                            ? 'bg-yellow-100 text-yellow-800'
                            : order.status === 'Shipped'
                            ? 'bg-blue-100 text-blue-800'
                            : 'bg-green-100 text-green-800'
                        }`}
                      >
                        {order.status}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
