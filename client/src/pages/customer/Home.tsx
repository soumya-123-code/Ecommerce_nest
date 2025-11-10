import { useState } from 'react';
import { gql, useQuery } from '@apollo/client';
import { CustomerLayout } from '../../components/layout/CustomerLayout';
import { CategorySidebar } from '../../components/CategorySidebar';
import { ProductCard } from '../../components/ProductCard';
import { ChevronLeft, ChevronRight, ArrowRight } from 'lucide-react';

const GET_HOME_DATA = gql`
  query GetHomeData {
    allProducts(limit: 20, offset: 0) {
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
    allSuperCategories {
      id
      name
      slug
      categoryImage
    }
    allMainCategories {
      id
      name
      slug
      categoryImage
    }
  }
`;

interface HomeProps {
  onNavigate?: (path: string) => void;
}

export function Home({ onNavigate }: HomeProps) {
  const [currentSlide, setCurrentSlide] = useState(0);
  const { data, loading } = useQuery(GET_HOME_DATA);

  // Carousel slides data
  const carouselSlides = [
    {
      id: 1,
      title: 'Don\'t miss amazing grocery deals',
      subtitle: 'Sign up for the daily newsletter',
      image: '/assets/imgs/slider/slider-1.png',
      url: '#'
    },
    {
      id: 2,
      title: 'Fresh Vegetables Big discount',
      subtitle: 'Save up to 50% off on your first order',
      image: '/assets/imgs/slider/slider-2.png',
      url: '#'
    }
  ];

  // Right side ads
  const sideAds = [
    {
      id: 1,
      title: 'Everyday Fresh & Clean with Our Products',
      image: '/assets/imgs/banner/banner-1.png',
      url: '#'
    },
    {
      id: 2,
      title: 'Make your Breakfast Healthy and Easy',
      image: '/assets/imgs/banner/banner-2.png',
      url: '#'
    }
  ];

  const products = data?.allProducts || [];
  const superCategories = data?.allSuperCategories || [];
  const mainCategories = data?.allMainCategories || [];

  const handlePrevSlide = () => {
    setCurrentSlide((prev) => (prev === 0 ? carouselSlides.length - 1 : prev - 1));
  };

  const handleNextSlide = () => {
    setCurrentSlide((prev) => (prev === carouselSlides.length - 1 ? 0 : prev + 1));
  };

  const handleAddToCart = (productId: string) => {
    console.log('Add to cart:', productId);
    // TODO: Implement add to cart mutation
  };

  const handleAddToWishlist = (productId: string) => {
    console.log('Add to wishlist:', productId);
    // TODO: Implement add to wishlist mutation
  };

  const handleQuickView = (productId: string) => {
    console.log('Quick view:', productId);
    // TODO: Implement quick view modal
  };

  const handleProductClick = (slug: string) => {
    onNavigate?.(`/product/${slug}`);
  };

  const handleCategoryClick = (slug: string) => {
    onNavigate?.(`/category/${slug}`);
  };

  return (
    <CustomerLayout onNavigate={onNavigate}>
      {/* Hero Slider Section */}
      <section className="home-slider position-relative mb-30">
        <div className="container mx-auto px-4 py-8">
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
            {/* Category Sidebar - Desktop */}
            <div className="hidden lg:block lg:col-span-2">
              <CategorySidebar onCategoryClick={handleCategoryClick} />
            </div>

            {/* Main Carousel */}
            <div className="lg:col-span-7">
              <div className="home-slide-cover mt-30 relative">
                <div className="hero-slider-1 relative overflow-hidden rounded-xl">
                  {carouselSlides.map((slide, index) => (
                    <div
                      key={slide.id}
                      className={`single-hero-slider single-animation-wrap bg-gradient-to-r from-brand-light to-brand-lighter p-12 rounded-xl transition-opacity duration-500 ${
                        index === currentSlide ? 'block' : 'hidden'
                      }`}
                      style={{
                        backgroundImage: slide.image ? `url(${slide.image})` : undefined,
                        backgroundSize: 'cover',
                        backgroundPosition: 'right center',
                        minHeight: '400px'
                      }}
                    >
                      <div className="slider-content max-w-md">
                        <h1 className="display-2 text-4xl md:text-5xl font-bold mb-8 text-gray-900">
                          {slide.title}
                        </h1>
                        <p className="mb-8 text-gray-700">{slide.subtitle}</p>
                        <form className="form-subcriber flex selector-1 max-w-md">
                          <input
                            type="email"
                            placeholder="Your email address"
                            className="flex-1 px-6 py-4 rounded-l-full border-0 focus:outline-none"
                            required
                          />
                          <button
                            type="submit"
                            className="bg-brand hover:bg-brand-dark text-white px-8 py-4 rounded-r-full font-semibold transition-colors"
                          >
                            Subscribe
                          </button>
                        </form>
                      </div>
                    </div>
                  ))}
                </div>

                {/* Carousel Navigation */}
                <button
                  onClick={handlePrevSlide}
                  className="absolute left-4 top-1/2 -translate-y-1/2 bg-white hover:bg-brand hover:text-white p-3 rounded-full shadow-lg transition-colors z-10"
                >
                  <ChevronLeft className="w-6 h-6" />
                </button>
                <button
                  onClick={handleNextSlide}
                  className="absolute right-4 top-1/2 -translate-y-1/2 bg-white hover:bg-brand hover:text-white p-3 rounded-full shadow-lg transition-colors z-10"
                >
                  <ChevronRight className="w-6 h-6" />
                </button>

                {/* Dots Indicator */}
                <div className="absolute bottom-6 left-1/2 -translate-x-1/2 flex space-x-2">
                  {carouselSlides.map((_, index) => (
                    <button
                      key={index}
                      onClick={() => setCurrentSlide(index)}
                      className={`w-3 h-3 rounded-full transition-colors ${
                        index === currentSlide ? 'bg-brand' : 'bg-white bg-opacity-50'
                      }`}
                    />
                  ))}
                </div>
              </div>
            </div>

            {/* Right Side Ads */}
            <div className="lg:col-span-3">
              <div className="space-y-6">
                {sideAds.map((ad) => (
                  <div key={ad.id} className="banner-img style-4 mt-30 relative rounded-xl overflow-hidden">
                    <img
                      src={ad.image}
                      alt={ad.title}
                      className="w-full h-auto"
                      onError={(e) => {
                        (e.target as HTMLImageElement).src = '/assets/imgs/banner/banner-placeholder.jpg';
                      }}
                    />
                    <div className="banner-text absolute bottom-6 left-6 right-6">
                      <h4 className="text-white text-xl font-semibold mb-4 drop-shadow-lg">
                        {ad.title}
                      </h4>
                      <a
                        href={ad.url}
                        className="btn btn-xs bg-white hover:bg-brand text-gray-800 hover:text-white px-6 py-2 rounded-full inline-flex items-center transition-colors"
                      >
                        Shop Now <ArrowRight className="w-4 h-4 ml-2" />
                      </a>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Featured Categories Section */}
      <section className="popular-categories section-padding bg-gray-50 py-16">
        <div className="container mx-auto px-4">
          <div className="section-title mb-8">
            <div className="flex justify-between items-center">
              <div>
                <h3 className="text-3xl font-bold mb-4">Featured Categories</h3>
                <ul className="flex space-x-6">
                  {superCategories.slice(0, 5).map((category: any) => (
                    <li key={category.id}>
                      <a
                        href={`/category/${category.slug}`}
                        onClick={(e) => {
                          e.preventDefault();
                          handleCategoryClick(category.slug);
                        }}
                        className="text-gray-600 hover:text-brand font-medium"
                      >
                        {category.name}
                      </a>
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          </div>

          {/* Category Cards Carousel */}
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-10 gap-4">
            {mainCategories.slice(0, 10).map((category: any, index: number) => (
              <div
                key={category.id}
                className={`card-2 bg-${(index % 12) + 1} rounded-lg p-6 text-center hover:shadow-lg transition-shadow cursor-pointer`}
                onClick={() => handleCategoryClick(category.slug)}
              >
                <figure className="img-hover-scale overflow-hidden mb-4">
                  {category.categoryImage ? (
                    <img
                      src={category.categoryImage}
                      alt={category.name}
                      className="w-20 h-20 mx-auto object-cover rounded-lg hover:scale-110 transition-transform"
                      onError={(e) => {
                        (e.target as HTMLImageElement).src = '/assets/imgs/theme/category.png';
                      }}
                    />
                  ) : (
                    <div className="w-20 h-20 mx-auto bg-brand-light rounded-lg flex items-center justify-center">
                      <span className="text-3xl text-brand font-bold">{category.name.charAt(0)}</span>
                    </div>
                  )}
                </figure>
                <h6>
                  <a
                    href={`/category/${category.slug}`}
                    onClick={(e) => {
                      e.preventDefault();
                      handleCategoryClick(category.slug);
                    }}
                    className="text-gray-800 hover:text-brand font-medium text-sm"
                  >
                    {category.name}
                  </a>
                </h6>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Popular Products Section */}
      <section className="product-tabs section-padding py-16">
        <div className="container mx-auto px-4">
          <div className="section-title mb-8">
            <h3 className="text-3xl font-bold">Popular Products</h3>
            <p className="text-gray-600 mt-2">Don't miss our daily amazing deals. Shop now!</p>
          </div>

          {loading ? (
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
              {[...Array(10)].map((_, i) => (
                <div key={i} className="animate-pulse">
                  <div className="bg-gray-200 h-64 rounded-lg mb-4"></div>
                  <div className="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
                  <div className="h-4 bg-gray-200 rounded w-1/2"></div>
                </div>
              ))}
            </div>
          ) : (
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
              {products.slice(0, 10).map((product: any) => (
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
          )}

          {!loading && products.length === 0 && (
            <div className="text-center py-12">
              <p className="text-gray-500">No products available at the moment.</p>
            </div>
          )}
        </div>
      </section>

      {/* New Arrivals Section */}
      <section className="product-tabs section-padding bg-gray-50 py-16">
        <div className="container mx-auto px-4">
          <div className="section-title mb-8">
            <h3 className="text-3xl font-bold">New Arrivals</h3>
            <p className="text-gray-600 mt-2">Check out our latest products</p>
          </div>

          {!loading && (
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
              {products.slice(10, 20).map((product: any) => (
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
          )}
        </div>
      </section>

      {/* Vendor Showcase Section */}
      <section className="section-padding py-16">
        <div className="container mx-auto px-4">
          <div className="section-title mb-8">
            <h3 className="text-3xl font-bold">Featured Vendors</h3>
            <p className="text-gray-600 mt-2">Top rated sellers on our platform</p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {[1, 2, 3, 4].map((vendor) => (
              <div
                key={vendor}
                className="vendor-card bg-white rounded-lg shadow-sm hover:shadow-lg transition-shadow p-6 text-center"
              >
                <div className="vendor-logo mb-4">
                  <div className="w-20 h-20 mx-auto bg-brand-light rounded-full flex items-center justify-center">
                    <span className="text-2xl text-brand font-bold">V{vendor}</span>
                  </div>
                </div>
                <h5 className="font-semibold text-lg mb-2">Vendor Name {vendor}</h5>
                <p className="text-sm text-gray-600 mb-4">Premium quality products</p>
                <button className="text-brand hover:text-brand-dark font-medium text-sm">
                  Visit Store →
                </button>
              </div>
            ))}
          </div>
        </div>
      </section>
    </CustomerLayout>
  );
}
