import { useState } from 'react';
import { gql, useQuery } from '@apollo/client';
import { CustomerLayout } from '../../components/layout/CustomerLayout';
import { ProductCard } from '../../components/ProductCard';
import { Star, ShoppingCart, Heart, Share2, Minus, Plus, Package, Truck, RotateCcw } from 'lucide-react';

const GET_PRODUCT_DETAILS = gql`
  query GetProductDetails($slug: String!) {
    productBySlug(slug: $slug) {
      id
      productName
      productSlug
      productDescription
      prdPrice
      prdDiscountPrice
      productImage
      additionalImage1
      additionalImage2
      additionalImage3
      additionalImage4
      prdSku
      prdInStock
      productSuperCategory {
        name
        slug
      }
      productMainCategory {
        name
        slug
      }
      productVendor {
        displayName
        user {
          username
        }
      }
    }
    allProducts(limit: 4, offset: 0) {
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

interface ProductDetailsProps {
  slug: string;
  onNavigate?: (path: string) => void;
}

export function ProductDetails({ slug, onNavigate }: ProductDetailsProps) {
  const [selectedImage, setSelectedImage] = useState(0);
  const [quantity, setQuantity] = useState(1);
  const [activeTab, setActiveTab] = useState<'description' | 'additional' | 'reviews'>('description');

  const { data, loading } = useQuery(GET_PRODUCT_DETAILS, {
    variables: { slug }
  });

  const product = data?.productBySlug;
  const relatedProducts = data?.allProducts || [];

  if (loading) {
    return (
      <CustomerLayout onNavigate={onNavigate}>
        <div className="container mx-auto px-4 py-20">
          <div className="animate-pulse">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
              <div className="bg-gray-200 h-96 rounded-lg"></div>
              <div className="space-y-4">
                <div className="h-8 bg-gray-200 rounded w-3/4"></div>
                <div className="h-6 bg-gray-200 rounded w-1/2"></div>
                <div className="h-20 bg-gray-200 rounded"></div>
              </div>
            </div>
          </div>
        </div>
      </CustomerLayout>
    );
  }

  if (!product) {
    return (
      <CustomerLayout onNavigate={onNavigate}>
        <div className="container mx-auto px-4 py-20 text-center">
          <h2 className="text-2xl font-bold mb-4">Product Not Found</h2>
          <p className="text-gray-600 mb-8">Sorry, the product you're looking for doesn't exist.</p>
          <button
            onClick={() => onNavigate?.('/')}
            className="bg-brand hover:bg-brand-dark text-white px-8 py-3 rounded-lg"
          >
            Back to Home
          </button>
        </div>
      </CustomerLayout>
    );
  }

  const images = [
    product.productImage,
    product.additionalImage1,
    product.additionalImage2,
    product.additionalImage3,
    product.additionalImage4
  ].filter(Boolean);

  const discountPercentage = product.prdDiscountPrice
    ? Math.round(((parseFloat(product.prdPrice) - parseFloat(product.prdDiscountPrice)) / parseFloat(product.prdPrice)) * 100)
    : 0;

  const handleAddToCart = () => {
    console.log('Add to cart:', product.id, 'quantity:', quantity);
    // TODO: Implement add to cart mutation
  };

  const handleAddToWishlist = () => {
    console.log('Add to wishlist:', product.id);
    // TODO: Implement add to wishlist
  };

  return (
    <CustomerLayout onNavigate={onNavigate}>
      <main className="main">
        {/* Breadcrumb */}
        <div className="page-header breadcrumb-wrap bg-gray-100 py-6">
          <div className="container mx-auto px-4">
            <div className="breadcrumb flex items-center space-x-2 text-sm flex-wrap">
              <a href="/" onClick={(e) => { e.preventDefault(); onNavigate?.('/'); }} className="text-brand hover:underline flex items-center">
                Home
              </a>
              <span>/</span>
              {product.productSuperCategory && (
                <>
                  <a
                    href={`/category/${product.productSuperCategory.slug}`}
                    onClick={(e) => { e.preventDefault(); onNavigate?.(`/category/${product.productSuperCategory.slug}`); }}
                    className="text-gray-600 hover:text-brand"
                  >
                    {product.productSuperCategory.name}
                  </a>
                  <span>/</span>
                </>
              )}
              <span className="text-gray-600">{product.productName}</span>
            </div>
          </div>
        </div>

        {/* Product Details */}
        <div className="container mx-auto px-4 mb-30 mt-30">
          <div className="row">
            <div className="col-xl-11 col-lg-12 m-auto max-w-7xl mx-auto">
              <div className="product-detail accordion-detail">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-12 mb-12">
                  {/* Product Image Gallery */}
                  <div className="detail-gallery">
                    {/* Main Image */}
                    <div className="product-image-slider mb-4">
                      <figure className="border-radius-10 overflow-hidden rounded-xl bg-gray-100">
                        <img
                          src={images[selectedImage] || '/assets/imgs/shop/product-placeholder.jpg'}
                          alt={product.productName}
                          className="w-full h-auto object-cover"
                          style={{ maxHeight: '600px' }}
                        />
                      </figure>
                    </div>

                    {/* Thumbnails */}
                    {images.length > 1 && (
                      <div className="slider-nav-thumbnails flex space-x-3">
                        {images.map((image, index) => (
                          <div
                            key={index}
                            onClick={() => setSelectedImage(index)}
                            className={`cursor-pointer border-2 rounded-lg overflow-hidden ${
                              selectedImage === index ? 'border-brand' : 'border-gray-200'
                            }`}
                          >
                            <img
                              src={image}
                              alt={`Thumbnail ${index + 1}`}
                              className="w-20 h-20 object-cover"
                            />
                          </div>
                        ))}
                      </div>
                    )}
                  </div>

                  {/* Product Info */}
                  <div className="detail-info">
                    {/* Stock Badge */}
                    {product.prdInStock && (
                      <span className="stock-status in-stock bg-green-100 text-green-600 px-4 py-1 rounded-full text-sm font-semibold inline-block mb-4">
                        In Stock
                      </span>
                    )}

                    {discountPercentage > 0 && (
                      <span className="stock-status sale bg-red-500 text-white px-4 py-1 rounded-full text-sm font-semibold inline-block mb-4 ml-2">
                        -{discountPercentage}% Off
                      </span>
                    )}

                    {/* Product Name */}
                    <h2 className="title-detail text-4xl font-bold mb-4">{product.productName}</h2>

                    {/* Rating */}
                    <div className="product-detail-rating mb-4 flex items-center">
                      <div className="flex items-center">
                        {[...Array(5)].map((_, i) => (
                          <Star key={i} className={`w-5 h-5 ${i < 4 ? 'text-yellow-400 fill-yellow-400' : 'text-gray-300'}`} />
                        ))}
                      </div>
                      <span className="font-small ml-3 text-gray-600">(32 reviews)</span>
                    </div>

                    {/* Price */}
                    <div className="clearfix product-price-cover mb-6">
                      {product.prdDiscountPrice ? (
                        <div className="product-price">
                          <span className="current-price text-brand text-5xl font-bold">${product.prdDiscountPrice}</span>
                          <span className="save-price font-md color3 ml-4 bg-red-100 text-red-600 px-3 py-1 rounded">
                            {discountPercentage}% Off
                          </span>
                          <span className="old-price font-md ml-4 text-gray-400 line-through text-2xl">${product.prdPrice}</span>
                        </div>
                      ) : (
                        <div className="product-price">
                          <span className="current-price text-brand text-5xl font-bold">${product.prdPrice}</span>
                        </div>
                      )}
                    </div>

                    {/* Short Description */}
                    <div className="short-desc mb-6">
                      <p className="text-gray-600 text-lg leading-relaxed">{product.productDescription?.substring(0, 200)}...</p>
                    </div>

                    {/* Product Meta */}
                    <div className="font-xs mb-6 space-y-2">
                      <div className="flex items-center">
                        <span className="text-gray-600 w-24">SKU:</span>
                        <span className="text-gray-800 font-semibold">{product.prdSku || 'N/A'}</span>
                      </div>
                      <div className="flex items-center">
                        <span className="text-gray-600 w-24">Vendor:</span>
                        <span className="text-brand font-semibold">{product.productVendor?.displayName}</span>
                      </div>
                      {product.productMainCategory && (
                        <div className="flex items-center">
                          <span className="text-gray-600 w-24">Category:</span>
                          <span className="text-gray-800">{product.productMainCategory.name}</span>
                        </div>
                      )}
                    </div>

                    {/* Quantity and Add to Cart */}
                    <div className="detail-extralink mb-6">
                      <div className="flex items-center space-x-4">
                        {/* Quantity Selector */}
                        <div className="detail-qty border border-gray-300 rounded-lg flex items-center">
                          <button
                            onClick={() => setQuantity(Math.max(1, quantity - 1))}
                            className="qty-down p-3 hover:bg-gray-100 transition-colors"
                          >
                            <Minus className="w-4 h-4" />
                          </button>
                          <span className="qty-val px-6 font-semibold text-lg">{quantity}</span>
                          <button
                            onClick={() => setQuantity(quantity + 1)}
                            className="qty-up p-3 hover:bg-gray-100 transition-colors"
                          >
                            <Plus className="w-4 h-4" />
                          </button>
                        </div>

                        {/* Add to Cart Button */}
                        <button
                          onClick={handleAddToCart}
                          className="button button-add-to-cart bg-brand hover:bg-brand-dark text-white px-8 py-4 rounded-lg font-semibold flex-1 flex items-center justify-center transition-colors"
                        >
                          <ShoppingCart className="w-5 h-5 mr-2" />
                          Add to Cart
                        </button>

                        {/* Wishlist Button */}
                        <button
                          onClick={handleAddToWishlist}
                          className="bg-gray-100 hover:bg-brand hover:text-white p-4 rounded-lg transition-colors"
                        >
                          <Heart className="w-6 h-6" />
                        </button>
                      </div>
                    </div>

                    {/* Features */}
                    <div className="product-extra-info bg-gray-50 rounded-lg p-6 space-y-3">
                      <div className="flex items-center">
                        <Package className="w-5 h-5 text-brand mr-3" />
                        <span className="text-gray-700">Free shipping on orders over $50</span>
                      </div>
                      <div className="flex items-center">
                        <Truck className="w-5 h-5 text-brand mr-3" />
                        <span className="text-gray-700">Fast delivery within 2-3 business days</span>
                      </div>
                      <div className="flex items-center">
                        <RotateCcw className="w-5 h-5 text-brand mr-3" />
                        <span className="text-gray-700">30-day easy return policy</span>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Product Tabs */}
                <div className="product-info mt-12">
                  <div className="tab-style3 border-b mb-6">
                    <ul className="nav nav-tabs flex space-x-8">
                      <li>
                        <button
                          onClick={() => setActiveTab('description')}
                          className={`nav-link pb-4 font-semibold ${activeTab === 'description' ? 'text-brand border-b-2 border-brand' : 'text-gray-600'}`}
                        >
                          Description
                        </button>
                      </li>
                      <li>
                        <button
                          onClick={() => setActiveTab('additional')}
                          className={`nav-link pb-4 font-semibold ${activeTab === 'additional' ? 'text-brand border-b-2 border-brand' : 'text-gray-600'}`}
                        >
                          Additional Info
                        </button>
                      </li>
                      <li>
                        <button
                          onClick={() => setActiveTab('reviews')}
                          className={`nav-link pb-4 font-semibold ${activeTab === 'reviews' ? 'text-brand border-b-2 border-brand' : 'text-gray-600'}`}
                        >
                          Reviews (32)
                        </button>
                      </li>
                    </ul>
                  </div>

                  <div className="tab-content">
                    {activeTab === 'description' && (
                      <div className="prose max-w-none">
                        <p className="text-gray-700 leading-relaxed text-lg">{product.productDescription}</p>
                      </div>
                    )}

                    {activeTab === 'additional' && (
                      <table className="w-full">
                        <tbody>
                          <tr className="border-b">
                            <th className="text-left py-3 text-gray-600">SKU</th>
                            <td className="py-3">{product.prdSku || 'N/A'}</td>
                          </tr>
                          <tr className="border-b">
                            <th className="text-left py-3 text-gray-600">Vendor</th>
                            <td className="py-3">{product.productVendor?.displayName}</td>
                          </tr>
                          <tr className="border-b">
                            <th className="text-left py-3 text-gray-600">Stock Status</th>
                            <td className="py-3">
                              <span className={`${product.prdInStock ? 'text-green-600' : 'text-red-600'}`}>
                                {product.prdInStock ? 'In Stock' : 'Out of Stock'}
                              </span>
                            </td>
                          </tr>
                        </tbody>
                      </table>
                    )}

                    {activeTab === 'reviews' && (
                      <div className="space-y-6">
                        <p className="text-gray-600">No reviews yet. Be the first to review this product!</p>
                        <button className="bg-brand hover:bg-brand-dark text-white px-6 py-3 rounded-lg">
                          Write a Review
                        </button>
                      </div>
                    )}
                  </div>
                </div>

                {/* Related Products */}
                <div className="related-products mt-20">
                  <h3 className="text-3xl font-bold mb-8">Related Products</h3>
                  <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-6">
                    {relatedProducts.map((relatedProduct: any) => (
                      <ProductCard
                        key={relatedProduct.id}
                        id={relatedProduct.id}
                        name={relatedProduct.productName}
                        slug={relatedProduct.productSlug}
                        price={parseFloat(relatedProduct.prdPrice)}
                        discountPrice={relatedProduct.prdDiscountPrice ? parseFloat(relatedProduct.prdDiscountPrice) : undefined}
                        image={relatedProduct.productImage}
                        vendor={relatedProduct.productVendor}
                        rating={4}
                        onClick={(slug) => onNavigate?.(`/product/${slug}`)}
                      />
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </CustomerLayout>
  );
}
