import { Heart, ShoppingCart, Eye, Star } from 'lucide-react';

interface ProductCardProps {
  id: string;
  name: string;
  slug: string;
  price: number;
  discountPrice?: number;
  image?: string;
  rating?: number;
  vendor?: {
    displayName: string;
  };
  onAddToCart?: (id: string) => void;
  onAddToWishlist?: (id: string) => void;
  onQuickView?: (id: string) => void;
  onClick?: (slug: string) => void;
}

export function ProductCard({
  id,
  name,
  slug,
  price,
  discountPrice,
  image,
  rating = 0,
  vendor,
  onAddToCart,
  onAddToWishlist,
  onQuickView,
  onClick
}: ProductCardProps) {
  const discountPercentage = discountPrice
    ? Math.round(((price - discountPrice) / price) * 100)
    : 0;

  const handleProductClick = (e: React.MouseEvent) => {
    e.preventDefault();
    if (onClick) {
      onClick(slug);
    }
  };

  return (
    <div className="product-cart-wrap bg-white rounded-lg shadow-sm hover:shadow-lg transition-shadow overflow-hidden group">
      {/* Product Image */}
      <div className="product-img-action-wrap relative">
        <div className="product-img product-img-zoom h-64 overflow-hidden bg-gray-100">
          <a href={`/product/${slug}`} onClick={handleProductClick}>
            {image ? (
              <img
                src={image}
                alt={name}
                className="w-full h-full object-cover group-hover:scale-110 transition-transform duration-300"
                onError={(e) => {
                  (e.target as HTMLImageElement).src = '/assets/imgs/shop/product-placeholder.jpg';
                }}
              />
            ) : (
              <div className="w-full h-full flex items-center justify-center bg-gray-200">
                <span className="text-gray-400 text-4xl">{name.charAt(0)}</span>
              </div>
            )}
          </a>
        </div>

        {/* Discount Badge */}
        {discountPercentage > 0 && (
          <div className="product-badges absolute top-3 left-3">
            <span className="bg-red-500 text-white px-3 py-1 rounded-full text-xs font-semibold">
              -{discountPercentage}%
            </span>
          </div>
        )}

        {/* Action Buttons */}
        <div className="product-action-1 absolute top-3 right-3 opacity-0 group-hover:opacity-100 transition-opacity">
          <button
            onClick={() => onAddToWishlist?.(id)}
            className="bg-white hover:bg-brand hover:text-white text-gray-700 p-2 rounded-full shadow-md mb-2 transition-colors"
            title="Add to Wishlist"
          >
            <Heart className="w-5 h-5" />
          </button>
          <button
            onClick={() => onQuickView?.(id)}
            className="bg-white hover:bg-brand hover:text-white text-gray-700 p-2 rounded-full shadow-md transition-colors"
            title="Quick View"
          >
            <Eye className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Product Info */}
      <div className="product-content-wrap p-4">
        {/* Vendor */}
        {vendor && (
          <div className="product-category text-xs text-gray-500 mb-2">
            <span>{vendor.displayName}</span>
          </div>
        )}

        {/* Product Name */}
        <h2 className="product-title mb-2">
          <a
            href={`/product/${slug}`}
            onClick={handleProductClick}
            className="text-gray-800 hover:text-brand font-medium line-clamp-2"
          >
            {name}
          </a>
        </h2>

        {/* Rating */}
        <div className="product-rate-cover mb-3 flex items-center">
          <div className="product-rate flex items-center">
            {[...Array(5)].map((_, i) => (
              <Star
                key={i}
                className={`w-4 h-4 ${
                  i < rating ? 'text-yellow-400 fill-yellow-400' : 'text-gray-300'
                }`}
              />
            ))}
          </div>
          <span className="font-small ml-2 text-gray-500 text-sm">({rating}.0)</span>
        </div>

        {/* Price and Add to Cart */}
        <div className="product-card-bottom flex items-center justify-between">
          <div className="product-price">
            {discountPrice ? (
              <>
                <span className="text-brand font-bold text-lg">${discountPrice}</span>
                <span className="text-gray-400 line-through text-sm ml-2">${price}</span>
              </>
            ) : (
              <span className="text-brand font-bold text-lg">${price}</span>
            )}
          </div>
          <button
            onClick={() => onAddToCart?.(id)}
            className="bg-brand-light hover:bg-brand text-brand hover:text-white p-2 rounded-lg transition-colors"
            title="Add to Cart"
          >
            <ShoppingCart className="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  );
}
