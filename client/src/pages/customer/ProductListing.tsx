import { useState } from 'react';
import { gql, useQuery } from '@apollo/client';
import { CustomerLayout } from '../../components/layout/CustomerLayout';
import { ProductCard } from '../../components/ProductCard';
import { Search, Grid, List, SlidersHorizontal } from 'lucide-react';

const GET_PRODUCTS = gql`
  query GetProducts($search: String, $limit: Int, $offset: Int) {
    allProducts(limit: $limit, offset: $offset) {
      id
      productName
      productSlug
      prdPrice
      prdDiscountPrice
      productImage
      productVendor {
        displayName
      }
    }
  }
`;

interface ProductListingProps {
  onNavigate?: (path: string) => void;
  searchQuery?: string;
  categorySlug?: string;
}

export function ProductListing({ onNavigate, searchQuery, categorySlug }: ProductListingProps) {
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [sortBy, setSortBy] = useState('featured');
  const [itemsPerPage, setItemsPerPage] = useState(20);
  const [localSearch, setLocalSearch] = useState(searchQuery || '');

  const { data, loading } = useQuery(GET_PRODUCTS, {
    variables: {
      search: searchQuery,
      limit: itemsPerPage,
      offset: 0
    }
  });

  const products = data?.allProducts || [];

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Searching for:', localSearch);
    // TODO: Implement search with GraphQL
  };

  const handleAddToCart = (productId: string) => {
    console.log('Add to cart:', productId);
    // TODO: Implement add to cart mutation
  };

  const handleAddToWishlist = (productId: string) => {
    console.log('Add to wishlist:', productId);
    // TODO: Implement add to wishlist
  };

  const handleQuickView = (productId: string) => {
    console.log('Quick view:', productId);
    // TODO: Implement quick view
  };

  const handleProductClick = (slug: string) => {
    onNavigate?.(`/product/${slug}`);
  };

  return (
    <CustomerLayout onNavigate={onNavigate}>
      <main className="main pages mb-80">
        {/* Breadcrumb */}
        <div className="page-header breadcrumb-wrap bg-gray-100 py-6">
          <div className="container mx-auto px-4">
            <div className="breadcrumb flex items-center space-x-2 text-sm">
              <a href="/" onClick={(e) => { e.preventDefault(); onNavigate?.('/'); }} className="text-brand hover:underline flex items-center">
                <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                </svg>
                Home
              </a>
              <span>/</span>
              <span className="text-gray-600">Products List</span>
            </div>
          </div>
        </div>

        {/* Page Content */}
        <div className="page-content pt-50">
          <div className="container mx-auto px-4">
            {/* Search Header */}
            <div className="archive-header-2 text-center mb-12">
              <h1 className="display-2 text-4xl font-bold mb-8">Products List</h1>
              <div className="row">
                <div className="col-lg-5 mx-auto max-w-2xl">
                  <div className="sidebar-widget-2 widget_search mb-50">
                    <div className="search-form">
                      <form onSubmit={handleSearch} className="flex">
                        <input
                          type="text"
                          value={localSearch}
                          onChange={(e) => setLocalSearch(e.target.value)}
                          placeholder="Search Items (by name)..."
                          className="flex-1 px-6 py-4 border border-gray-300 rounded-l-full focus:outline-none focus:border-brand"
                          required
                        />
                        <button
                          type="submit"
                          className="bg-brand hover:bg-brand-dark text-white px-8 rounded-r-full transition-colors"
                        >
                          <Search className="w-5 h-5" />
                        </button>
                      </form>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Filter and Sort Bar */}
            <div className="row mb-50">
              <div className="col-12 col-lg-8 mx-auto max-w-6xl">
                <div className="shop-product-fillter bg-gray-50 rounded-lg p-4">
                  <div className="flex flex-col md:flex-row justify-between items-center gap-4">
                    {/* Total Products */}
                    <div className="totall-product">
                      {searchQuery ? (
                        <p className="text-gray-700">
                          We have <strong className="text-brand">{products.length}</strong> items now for
                          <strong className="text-brand ml-1">"{searchQuery}"</strong>
                        </p>
                      ) : (
                        <p className="text-gray-700">
                          Showing <strong className="text-brand">{products.length}</strong> products
                        </p>
                      )}
                    </div>

                    {/* View Mode and Sort */}
                    <div className="flex items-center space-x-4">
                      {/* View Mode Toggle */}
                      <div className="flex items-center space-x-2 bg-white rounded-lg p-1">
                        <button
                          onClick={() => setViewMode('grid')}
                          className={`p-2 rounded ${viewMode === 'grid' ? 'bg-brand text-white' : 'text-gray-600 hover:bg-gray-100'}`}
                        >
                          <Grid className="w-5 h-5" />
                        </button>
                        <button
                          onClick={() => setViewMode('list')}
                          className={`p-2 rounded ${viewMode === 'list' ? 'bg-brand text-white' : 'text-gray-600 hover:bg-gray-100'}`}
                        >
                          <List className="w-5 h-5" />
                        </button>
                      </div>

                      {/* Sort Dropdown */}
                      <select
                        value={sortBy}
                        onChange={(e) => setSortBy(e.target.value)}
                        className="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:border-brand"
                      >
                        <option value="featured">Featured</option>
                        <option value="newest">Newest</option>
                        <option value="price-low">Price: Low to High</option>
                        <option value="price-high">Price: High to Low</option>
                        <option value="rating">Highest Rated</option>
                      </select>

                      {/* Items per page */}
                      <select
                        value={itemsPerPage}
                        onChange={(e) => setItemsPerPage(Number(e.target.value))}
                        className="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:border-brand"
                      >
                        <option value={20}>20 items</option>
                        <option value={50}>50 items</option>
                        <option value={100}>100 items</option>
                      </select>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Products Grid */}
            <div id="vendors-list" className="vendor-grid">
              {loading ? (
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                  {[...Array(12)].map((_, i) => (
                    <div key={i} className="animate-pulse">
                      <div className="bg-gray-200 h-64 rounded-lg mb-4"></div>
                      <div className="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
                      <div className="h-4 bg-gray-200 rounded w-1/2"></div>
                    </div>
                  ))}
                </div>
              ) : viewMode === 'grid' ? (
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                  {products.map((product: any) => (
                    <ProductCard
                      key={product.id}
                      id={product.id}
                      name={product.productName}
                      slug={product.productSlug}
                      price={parseFloat(product.prdPrice)}
                      discountPrice={product.prdDiscountPrice ? parseFloat(product.prdDiscountPrice) : undefined}
                      image={product.productImage}
                      vendor={product.productVendor}
                      rating={4}
                      onAddToCart={handleAddToCart}
                      onAddToWishlist={handleAddToWishlist}
                      onQuickView={handleQuickView}
                      onClick={handleProductClick}
                    />
                  ))}
                </div>
              ) : (
                <div className="space-y-4">
                  {products.map((product: any) => (
                    <div key={product.id} className="bg-white rounded-lg shadow-sm p-6 flex gap-6 hover:shadow-lg transition-shadow">
                      <img
                        src={product.productImage || '/assets/imgs/shop/product-placeholder.jpg'}
                        alt={product.productName}
                        className="w-48 h-48 object-cover rounded-lg"
                      />
                      <div className="flex-1">
                        <h3 className="text-xl font-semibold mb-2">{product.productName}</h3>
                        <p className="text-gray-600 mb-4">By {product.productVendor?.displayName}</p>
                        <div className="flex items-center gap-4">
                          {product.prdDiscountPrice ? (
                            <>
                              <span className="text-brand font-bold text-2xl">${product.prdDiscountPrice}</span>
                              <span className="text-gray-400 line-through text-lg">${product.prdPrice}</span>
                            </>
                          ) : (
                            <span className="text-brand font-bold text-2xl">${product.prdPrice}</span>
                          )}
                          <button
                            onClick={() => handleAddToCart(product.id)}
                            className="ml-auto bg-brand hover:bg-brand-dark text-white px-6 py-3 rounded-lg transition-colors"
                          >
                            Add to Cart
                          </button>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}

              {!loading && products.length === 0 && (
                <div className="text-center py-20">
                  <p className="text-gray-500 text-lg">No products found.</p>
                  <p className="text-gray-400 mt-2">Try adjusting your search or filters.</p>
                </div>
              )}
            </div>

            {/* Pagination */}
            {products.length > 0 && (
              <div className="pagination-area mt-20 mb-20">
                <nav>
                  <ul className="flex justify-center items-center space-x-2">
                    <li>
                      <button className="px-4 py-2 border rounded hover:bg-brand hover:text-white transition-colors">
                        Previous
                      </button>
                    </li>
                    <li>
                      <button className="px-4 py-2 border rounded bg-brand text-white">1</button>
                    </li>
                    <li>
                      <button className="px-4 py-2 border rounded hover:bg-brand hover:text-white transition-colors">2</button>
                    </li>
                    <li>
                      <button className="px-4 py-2 border rounded hover:bg-brand hover:text-white transition-colors">3</button>
                    </li>
                    <li>
                      <button className="px-4 py-2 border rounded hover:bg-brand hover:text-white transition-colors">
                        Next
                      </button>
                    </li>
                  </ul>
                </nav>
              </div>
            )}
          </div>
        </div>
      </main>
    </CustomerLayout>
  );
}
