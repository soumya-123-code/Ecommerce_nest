import { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import {
  Home,
  Package,
  ShoppingCart,
  DollarSign,
  TrendingUp,
  Settings,
  User,
  LogOut,
  Menu,
  Bell,
  Search,
  Plus,
  Eye,
  Edit,
  MessageSquare,
} from 'lucide-react';

// GraphQL Queries
const GET_VENDOR_STATS = gql`
  query GetVendorStats {
    me {
      id
      username
      profile {
        displayName
        balance
      }
    }
  }
`;

interface VendorDashboardNewProps {
  onNavigate: (view: 'home' | 'admin' | 'vendor' | 'login') => void;
}

export function VendorDashboardNew({ onNavigate }: VendorDashboardNewProps) {
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const { data, loading } = useQuery(GET_VENDOR_STATS);

  // Mock data (replace with real data from GraphQL)
  const stats = [
    {
      title: 'Revenue',
      value: '$' + (data?.me?.profile?.balance || '0.00'),
      icon: DollarSign,
      color: 'bg-blue-500',
      lightColor: 'bg-blue-50',
      textColor: 'text-blue-600',
    },
    {
      title: 'Underway Orders',
      value: '8',
      icon: ShoppingCart,
      color: 'bg-green-500',
      lightColor: 'bg-green-50',
      textColor: 'text-green-600',
    },
    {
      title: 'Products',
      value: '42',
      icon: Package,
      color: 'bg-yellow-500',
      lightColor: 'bg-yellow-50',
      textColor: 'text-yellow-600',
    },
    {
      title: 'Orders',
      value: '156',
      icon: TrendingUp,
      color: 'bg-purple-500',
      lightColor: 'bg-purple-50',
      textColor: 'text-purple-600',
    },
  ];

  const recentProducts = [
    {
      id: 1,
      name: 'Wireless Headphones Premium Quality',
      category: 'Electronics',
      image: '/api/placeholder/80/80',
    },
    {
      id: 2,
      name: 'Smart Watch Series 5 Latest Model',
      category: 'Accessories',
      image: '/api/placeholder/80/80',
    },
    {
      id: 3,
      name: 'Laptop Stand Aluminum Adjustable',
      category: 'Office',
      image: '/api/placeholder/80/80',
    },
  ];

  const recentOrders = [
    {
      id: '#ORD-2024-001',
      customer: 'John Doe',
      product: 'Wireless Headphones',
      amount: '$99.99',
      status: 'Pending',
      date: '2024-01-15',
    },
    {
      id: '#ORD-2024-002',
      customer: 'Jane Smith',
      product: 'Smart Watch',
      amount: '$249.99',
      status: 'Shipped',
      date: '2024-01-14',
    },
    {
      id: '#ORD-2024-003',
      customer: 'Bob Johnson',
      product: 'Laptop Stand',
      amount: '$79.99',
      status: 'Delivered',
      date: '2024-01-13',
    },
    {
      id: '#ORD-2024-004',
      customer: 'Alice Williams',
      product: 'USB Cable',
      amount: '$19.99',
      status: 'Pending',
      date: '2024-01-12',
    },
  ];

  const menuItems = [
    { icon: Home, label: 'Dashboard', active: true, href: '/vendor' },
    { icon: ShoppingCart, label: 'Orders', href: '/vendor/orders' },
    { icon: Package, label: 'Products', href: '/vendor/products', submenu: ['Products List', 'Add Product'] },
    { icon: DollarSign, label: 'Payments', href: '/vendor/payments' },
    { icon: MessageSquare, label: 'Reviews', href: '/vendor/reviews' },
  ];

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Sidebar */}
      <aside
        className={`${
          sidebarOpen ? 'w-64' : 'w-20'
        } bg-white border-r transition-all duration-300 flex flex-col`}
      >
        {/* Logo */}
        <div className="p-4 border-b flex items-center justify-between">
          {sidebarOpen && (
            <div className="flex items-center gap-2">
              <Package className="h-8 w-8 text-blue-600" />
              <span className="font-bold text-xl">VendorHub</span>
            </div>
          )}
          <button
            onClick={() => setSidebarOpen(!sidebarOpen)}
            className="p-2 rounded-lg hover:bg-gray-100"
          >
            <Menu className="h-5 w-5" />
          </button>
        </div>

        {/* Menu Items */}
        <nav className="flex-1 p-4 space-y-2">
          {menuItems.map((item) => {
            const Icon = item.icon;
            return (
              <div key={item.label}>
                <button
                  className={`w-full flex items-center gap-3 p-3 rounded-lg transition-colors ${
                    item.active
                      ? 'bg-blue-50 text-blue-600'
                      : 'hover:bg-gray-100 text-gray-700'
                  }`}
                >
                  <Icon className="h-5 w-5 flex-shrink-0" />
                  {sidebarOpen && <span className="font-medium">{item.label}</span>}
                </button>
                {item.submenu && sidebarOpen && (
                  <div className="ml-11 mt-2 space-y-1">
                    {item.submenu.map((sub) => (
                      <a
                        key={sub}
                        href="#"
                        className="block p-2 text-sm text-gray-600 hover:text-blue-600"
                      >
                        {sub}
                      </a>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
        </nav>

        {/* Settings & Logout */}
        <div className="p-4 border-t space-y-2">
          <button className="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-gray-100 text-gray-700">
            <Settings className="h-5 w-5" />
            {sidebarOpen && <span>Settings</span>}
          </button>
          <button
            onClick={() => onNavigate('home')}
            className="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-gray-100 text-gray-700"
          >
            <User className="h-5 w-5" />
            {sidebarOpen && <span>Customer View</span>}
          </button>
          <button className="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-red-50 text-red-600">
            <LogOut className="h-5 w-5" />
            {sidebarOpen && <span>Logout</span>}
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Header */}
        <header className="bg-white border-b p-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
              <p className="text-gray-600">Whole data about your business here</p>
            </div>
            <div className="flex items-center gap-4">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
                <input
                  type="text"
                  placeholder="Search..."
                  className="pl-10 pr-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
              <button className="relative p-2 rounded-lg hover:bg-gray-100">
                <Bell className="h-6 w-6 text-gray-600" />
                <span className="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full"></span>
              </button>
              <div className="flex items-center gap-3 pl-4 border-l">
                <div className="w-10 h-10 rounded-full bg-blue-500 flex items-center justify-center text-white font-semibold">
                  {data?.me?.username?.[0]?.toUpperCase() || 'V'}
                </div>
                <div>
                  <p className="font-medium">{data?.me?.profile?.displayName || 'Vendor'}</p>
                  <p className="text-sm text-gray-500">Vendor</p>
                </div>
              </div>
            </div>
          </div>
        </header>

        {/* Content */}
        <main className="flex-1 overflow-y-auto p-6">
          {/* Stats Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
            {stats.map((stat) => {
              const Icon = stat.icon;
              return (
                <div key={stat.title} className="bg-white rounded-lg p-6 shadow-sm">
                  <div className="flex items-start justify-between">
                    <div>
                      <p className="text-gray-600 text-sm font-medium mb-1">{stat.title}</p>
                      <p className="text-2xl font-bold">{stat.value}</p>
                    </div>
                    <div className={`${stat.lightColor} p-3 rounded-lg`}>
                      <Icon className={`h-6 w-6 ${stat.textColor}`} />
                    </div>
                  </div>
                </div>
              );
            })}
          </div>

          {/* Charts & Data */}
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
            {/* Sales Chart */}
            <div className="lg:col-span-2 bg-white rounded-lg p-6 shadow-sm">
              <h3 className="text-lg font-semibold mb-4">Sale Statistics</h3>
              <div className="h-64 flex items-center justify-center bg-gray-50 rounded-lg">
                <p className="text-gray-400">Chart visualization here</p>
              </div>
            </div>

            {/* Recent Products */}
            <div className="bg-white rounded-lg p-6 shadow-sm">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-semibold">New Products</h3>
                <button className="text-blue-600 hover:text-blue-700 text-sm font-medium">
                  View All
                </button>
              </div>
              <div className="space-y-4">
                {recentProducts.map((product) => (
                  <div key={product.id} className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <img
                        src={product.image}
                        alt={product.name}
                        className="w-12 h-12 rounded-lg object-cover"
                      />
                      <div>
                        <p className="font-medium text-sm">{product.name.slice(0, 30)}...</p>
                        <p className="text-xs text-gray-500">{product.category}</p>
                      </div>
                    </div>
                    <button className="p-2 hover:bg-gray-100 rounded-lg">
                      <Eye className="h-4 w-4 text-gray-600" />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Recent Orders */}
          <div className="bg-white rounded-lg shadow-sm">
            <div className="p-6 border-b flex items-center justify-between">
              <h3 className="text-lg font-semibold">Recent Orders</h3>
              <button className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 flex items-center gap-2">
                <Plus className="h-4 w-4" />
                New Product
              </button>
            </div>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="text-left p-4 font-semibold text-sm text-gray-700">Order ID</th>
                    <th className="text-left p-4 font-semibold text-sm text-gray-700">Customer</th>
                    <th className="text-left p-4 font-semibold text-sm text-gray-700">Product</th>
                    <th className="text-left p-4 font-semibold text-sm text-gray-700">Amount</th>
                    <th className="text-left p-4 font-semibold text-sm text-gray-700">Status</th>
                    <th className="text-left p-4 font-semibold text-sm text-gray-700">Date</th>
                    <th className="text-left p-4 font-semibold text-sm text-gray-700">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {recentOrders.map((order) => (
                    <tr key={order.id} className="border-t hover:bg-gray-50">
                      <td className="p-4 font-medium text-sm">{order.id}</td>
                      <td className="p-4 text-sm">{order.customer}</td>
                      <td className="p-4 text-sm">{order.product}</td>
                      <td className="p-4 text-sm font-medium">{order.amount}</td>
                      <td className="p-4">
                        <span
                          className={`px-3 py-1 rounded-full text-xs font-medium ${
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
                      <td className="p-4 text-sm text-gray-600">{order.date}</td>
                      <td className="p-4">
                        <div className="flex items-center gap-2">
                          <button className="p-2 hover:bg-gray-100 rounded-lg">
                            <Eye className="h-4 w-4 text-gray-600" />
                          </button>
                          <button className="p-2 hover:bg-gray-100 rounded-lg">
                            <Edit className="h-4 w-4 text-gray-600" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
