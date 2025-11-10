import { Search, ShoppingCart, User, Menu, Bell } from 'lucide-react';
import { useState } from 'react';
import { gql, useQuery } from '@apollo/client';

const GET_SITE_INFO = gql`
  query GetSiteInfo {
    allSuperCategories {
      id
      name
      slug
    }
    me {
      id
      username
      profile {
        displayName
      }
    }
  }
`;

interface HeaderProps {
  cartCount?: number;
  onNavigate?: (path: string) => void;
}

export function Header({ cartCount = 0, onNavigate }: HeaderProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState('All Categories');
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const { data, loading } = useQuery(GET_SITE_INFO);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Searching for:', searchQuery, 'in category:', selectedCategory);
    // TODO: Implement search navigation
  };

  const handleNavigate = (path: string) => {
    if (onNavigate) {
      onNavigate(path);
    }
  };

  return (
    <header className="header-area header-style-1 header-style-5 header-height-2">
      {/* Mobile Promotion Banner */}
      <div className="mobile-promotion bg-brand text-white text-center py-2">
        <span className="text-sm">
          Grand opening sale! <a href="#" className="text-white font-semibold underline">Shop now</a>
        </span>
      </div>

      {/* Header Top - Desktop Only */}
      <div className="header-top header-top-ptb-1 d-none d-lg-block bg-gray-100 py-3">
        <div className="container mx-auto px-4">
          <div className="flex justify-between items-center">
            {/* Left Links */}
            <div className="header-info">
              <ul className="flex space-x-4 text-sm">
                <li>
                  <a href="#" className="text-gray-600 hover:text-brand">About Us</a>
                </li>
                <li>
                  <a href="#" onClick={() => handleNavigate('/account/dashboard')} className="text-gray-600 hover:text-brand cursor-pointer">
                    My Account
                  </a>
                </li>
              </ul>
            </div>

            {/* Center - News Flash */}
            <div className="text-center">
              <div className="inline-flex items-center text-sm">
                <Bell className="w-4 h-4 mr-2 text-brand" />
                <span className="text-success font-semibold">Ads:</span>
                <span className="ml-2 text-gray-600">Special offers available!</span>
                <a href="#" className="ml-2 text-brand hover:underline">Shop now</a>
              </div>
            </div>

            {/* Right - Currency/Language */}
            <div className="header-info-right">
              <select
                className="text-sm border border-gray-300 rounded px-3 py-1 focus:outline-none focus:border-brand"
                defaultValue="USD"
              >
                <option value="USD">$ USD</option>
                <option value="EUR">€ EUR</option>
                <option value="GBP">£ GBP</option>
                <option value="INR">₹ INR</option>
              </select>
            </div>
          </div>
        </div>
      </div>

      {/* Header Middle - Main Header */}
      <div className="header-middle header-middle-ptb-1 d-none d-lg-block bg-white py-6 border-b">
        <div className="container mx-auto px-4">
          <div className="flex items-center justify-between gap-8">
            {/* Logo */}
            <div className="logo flex-shrink-0">
              <a href="/" onClick={(e) => { e.preventDefault(); handleNavigate('/'); }}>
                <img
                  src="/assets/imgs/theme/logo.svg"
                  alt="eCommerce"
                  className="h-12"
                  onError={(e) => {
                    (e.target as HTMLImageElement).src = 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="120" height="40"><text x="10" y="25" font-size="20" fill="%233BB77E">eCommerce</text></svg>';
                  }}
                />
              </a>
            </div>

            {/* Search Bar */}
            <div className="flex-1 max-w-3xl">
              <form onSubmit={handleSearch} className="search-style-2 flex">
                <select
                  value={selectedCategory}
                  onChange={(e) => setSelectedCategory(e.target.value)}
                  className="select-active category-search border border-r-0 border-gray-300 rounded-l-md px-4 py-3 focus:outline-none focus:border-brand min-w-[180px]"
                >
                  <option>All Categories</option>
                  {!loading && data?.allSuperCategories?.map((category: any) => (
                    <option key={category.id} value={category.slug}>
                      {category.name}
                    </option>
                  ))}
                </select>
                <div className="flex-1 relative">
                  <input
                    type="text"
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    placeholder="Search for items..."
                    className="w-full border border-gray-300 px-4 py-3 focus:outline-none focus:border-brand"
                    required
                  />
                </div>
                <button
                  type="submit"
                  className="bg-brand hover:bg-brand-dark text-white px-6 rounded-r-md transition-colors"
                >
                  <Search className="w-5 h-5" />
                </button>
              </form>
            </div>

            {/* Header Actions */}
            <div className="header-action-right flex items-center space-x-6">
              {/* Cart */}
              <div className="header-action-icon-2 relative">
                <a
                  href="#"
                  onClick={(e) => { e.preventDefault(); handleNavigate('/cart'); }}
                  className="mini-cart-icon flex flex-col items-center hover:text-brand cursor-pointer"
                >
                  <div className="relative">
                    <ShoppingCart className="w-6 h-6" />
                    {cartCount > 0 && (
                      <span className="absolute -top-2 -right-2 bg-brand text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                        {cartCount}
                      </span>
                    )}
                  </div>
                  <span className="text-xs mt-1">Cart</span>
                </a>
              </div>

              {/* User Account */}
              <div className="header-action-icon-2">
                <a
                  href="#"
                  onClick={(e) => { e.preventDefault(); handleNavigate(data?.me ? '/account/dashboard' : '/account/login'); }}
                  className="flex flex-col items-center hover:text-brand cursor-pointer"
                >
                  <User className="w-6 h-6" />
                  <span className="text-xs mt-1">
                    {data?.me ? data.me.profile?.displayName || data.me.username : 'Account'}
                  </span>
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Mobile Header */}
      <div className="header-mobile d-lg-none bg-white py-4 border-b">
        <div className="container mx-auto px-4">
          <div className="flex items-center justify-between">
            <button
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              className="text-gray-600"
            >
              <Menu className="w-6 h-6" />
            </button>

            <div className="logo">
              <a href="/" onClick={(e) => { e.preventDefault(); handleNavigate('/'); }}>
                <img src="/assets/imgs/theme/logo.svg" alt="eCommerce" className="h-8" />
              </a>
            </div>

            <div className="flex items-center space-x-4">
              <a href="#" onClick={(e) => { e.preventDefault(); handleNavigate('/cart'); }} className="relative">
                <ShoppingCart className="w-6 h-6" />
                {cartCount > 0 && (
                  <span className="absolute -top-2 -right-2 bg-brand text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                    {cartCount}
                  </span>
                )}
              </a>
            </div>
          </div>

          {/* Mobile Search */}
          <form onSubmit={handleSearch} className="mt-4">
            <div className="flex">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search for items..."
                className="flex-1 border border-gray-300 rounded-l-md px-4 py-2 focus:outline-none focus:border-brand"
              />
              <button
                type="submit"
                className="bg-brand hover:bg-brand-dark text-white px-4 rounded-r-md"
              >
                <Search className="w-5 h-5" />
              </button>
            </div>
          </form>
        </div>
      </div>

      {/* Mobile Menu Overlay */}
      {mobileMenuOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50" onClick={() => setMobileMenuOpen(false)}>
          <div className="bg-white w-80 h-full overflow-y-auto" onClick={(e) => e.stopPropagation()}>
            <div className="p-4">
              <h3 className="font-semibold text-lg mb-4">Menu</h3>
              <ul className="space-y-3">
                <li>
                  <a href="#" onClick={() => { handleNavigate('/'); setMobileMenuOpen(false); }} className="block py-2">Home</a>
                </li>
                <li>
                  <a href="#" onClick={() => { handleNavigate('/account/dashboard'); setMobileMenuOpen(false); }} className="block py-2">My Account</a>
                </li>
                <li>
                  <a href="#" onClick={() => { handleNavigate('/cart'); setMobileMenuOpen(false); }} className="block py-2">Cart</a>
                </li>
              </ul>
            </div>
          </div>
        </div>
      )}
    </header>
  );
}
