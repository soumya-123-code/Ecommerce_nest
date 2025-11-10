# React Frontend Development Plan
## Converting Django Templates to React SPA

### Overview
This document outlines the complete conversion of Django server-side templates to a modern React SPA, using Django purely as a GraphQL API backend.

---

## 📋 Pages to Convert (Based on Django Templates)

### 1. **Customer Frontend** (Main eCommerce Site)

#### Home Pages
- `home/index-1.html` → `pages/customer/Home.tsx`
  - Hero slider/carousel with multiple slides
  - Category sidebar (collapsible, 10+ categories)
  - Featured products grid
  - Ads sections (sidebar, middle, daily deals, hot deals)
  - Newsletter subscription
  - Popular products, new arrivals
  - Vendor showcase

#### Product Pages
- `products/shop-product-vendor.tsx` → `pages/customer/ProductDetail.tsx`
  - Product image gallery (5 images)
  - Product information (price, discount, SKU, stock)
  - Size/variant selector
  - Add to cart functionality
  - Product ratings and reviews
  - Related products
  - Vendor information

- `products/product-search.html` → `pages/customer/ProductSearch.tsx`
  - Search results
  - Filters (category, price range, rating)
  - Sort options
  - Grid/List view toggle

#### Category Pages
- `categories/shop-super-category.html` → `pages/customer/CategoryPage.tsx`
  - 4-level category hierarchy navigation
  - Product grid with filters
  - Breadcrumbs
  - Sort and filter options

#### Cart & Checkout
- `orders/shop-cart.html` → `pages/customer/Cart.tsx`
  - Cart items list
  - Quantity update
  - Remove items
  - Coupon code input
  - Cart totals (subtotal, shipping, discount)

- `orders/shop-checkout.html` → `pages/customer/Checkout.tsx`
  - Billing/shipping information form
  - Payment method selection (Stripe, PayPal, Razorpay)
  - Order summary
  - Place order button

#### Account Pages
- `accounts/page-login.html` → `pages/customer/Login.tsx`
- `accounts/page-register.html` → `pages/customer/Register.tsx`
- `accounts/page-account.html` → `pages/customer/Account.tsx`
  - Profile information
  - Order history
  - Saved addresses
  - Wishlist

- `accounts/order-tracking.html` → `pages/customer/OrderTracking.tsx`
  - Track order by ID
  - Order status timeline
  - Shipping information

#### Blog
- `blog/blog-fullwidth.html` → `pages/customer/Blog.tsx`
- `blog/blog-post-fullwidth.html` → `pages/customer/BlogPost.tsx`

#### Other
- `contact/page-contact.html` → `pages/customer/Contact.tsx`
- `suppliers/vendors-grid.html` → `pages/customer/Vendors.tsx`
- `suppliers/vendor-details.html` → `pages/customer/VendorDetail.tsx`

---

### 2. **Vendor Panel** (Supplier Dashboard)

#### Dashboard
- `supplier-panel/index.html` → `pages/vendor/Dashboard.tsx`
  - Revenue stats card
  - Underway orders count
  - Products count
  - Total orders
  - Sales chart (Chart.js/Recharts)
  - Recent products list
  - Recent orders table

#### Products
- `supplier-panel/supplier-products-list.html` → `pages/vendor/ProductsList.tsx`
  - All products table
  - Edit/Delete actions
  - Stock status
  - Search and filter

- `supplier-panel/supplier-add-product.html` → `pages/vendor/AddProduct.tsx`
  - Product form (name, description, price, SKU)
  - Image uploads (5 images)
  - Category selection (4-level)
  - Size/variant management
  - Stock management

- `supplier-panel/supplier-edit-product.html` → `pages/vendor/EditProduct.tsx`
  - Same as add product but with existing data

#### Orders
- `supplier-panel/supplier-orders-list.html` → `pages/vendor/OrdersList.tsx`
  - Orders table
  - Status filters
  - Search by order ID
  - Date filters

- `supplier-panel/supplier-orders-detail.html` → `pages/vendor/OrderDetail.tsx`
  - Order information
  - Customer details
  - Products ordered
  - Update order status

#### Payments & Settings
- `supplier-panel/supplier-transactions.html` → `pages/vendor/Payments.tsx`
  - Payment history
  - Balance overview
  - Request payout

- `supplier-panel/page-bank-info.html` → `pages/vendor/BankInfo.tsx`
  - Bank account setup
  - PayPal email

- `supplier-panel/page-social-links.html` → `pages/vendor/SocialLinks.tsx`
  - Social media links

- `supplier-panel/supplier-reviews.html` → `pages/vendor/Reviews.tsx`
  - Product reviews list
  - Average ratings

#### Auth
- `supplier-panel/supplier-account-login.html` → `pages/vendor/VendorLogin.tsx`
- `supplier-panel/supplier-account-register.html` → `pages/vendor/VendorRegister.tsx`

---

### 3. **Admin Panel**

#### Dashboard
- `templates/admin/index.html` → `pages/admin/Dashboard.tsx`
  - Platform statistics
  - Recent orders
  - Revenue charts
  - User analytics

#### Management Pages
- Users management
- Products moderation
- Orders management
- Vendor approval
- Site settings
- Payment gateway configuration

---

## 🧩 Shared Components

### Layout Components
- `components/layout/Header.tsx`
  - Logo
  - Main navigation
  - Search bar
  - Cart icon with count
  - User menu (login/logout)
  - Category dropdown

- `components/layout/Footer.tsx`
  - Links sections
  - Newsletter signup
  - Social links
  - Copyright

- `components/layout/MobileMenu.tsx`
  - Mobile navigation
  - Category sidebar

### Common Components
- `components/ProductCard.tsx` - Product grid item
- `components/CategorySidebar.tsx` - 4-level category tree
- `components/Carousel.tsx` - Image slider
- `components/Breadcrumb.tsx` - Navigation breadcrumb
- `components/Pagination.tsx` - Page navigation
- `components/StarRating.tsx` - Star rating display/input
- `components/PriceDisplay.tsx` - Price with currency
- `components/CartButton.tsx` - Add to cart button
- `components/WishlistButton.tsx` - Add to wishlist
- `components/SearchBar.tsx` - Search input
- `components/FilterSidebar.tsx` - Product filters
- `components/OrderStatusBadge.tsx` - Status badge
- `components/LoadingSpinner.tsx` - Loading state
- `components/EmptyState.tsx` - Empty data state

### Vendor Panel Components
- `components/vendor/Sidebar.tsx` - Vendor sidebar navigation
- `components/vendor/StatCard.tsx` - Dashboard stat cards
- `components/vendor/SalesChart.tsx` - Chart component
- `components/vendor/ProductForm.tsx` - Product add/edit form
- `components/vendor/OrdersTable.tsx` - Orders data table
- `components/vendor/ImageUploader.tsx` - Multiple image upload

---

## 🔗 GraphQL Integration

### Queries
```typescript
// Products
GET_PRODUCTS - List products with pagination
GET_PRODUCT_BY_ID - Single product details
GET_PRODUCT_BY_SLUG - Product by slug
GET_PRODUCTS_BY_CATEGORY - Filtered by category
SEARCH_PRODUCTS - Search with filters

// Categories
GET_CATEGORIES - All category levels
GET_CATEGORY_TREE - Hierarchical structure

// Orders
GET_ORDERS - User orders
GET_ORDER_BY_ID - Order details
GET_VENDOR_ORDERS - Vendor's orders

// User
GET_CURRENT_USER - Logged in user
GET_USER_PROFILE - Profile details

// Cart
GET_CART - Current cart items

// Vendors
GET_VENDORS - All vendors
GET_VENDOR_BY_ID - Vendor details
GET_VENDOR_STATS - Dashboard stats
```

### Mutations
```typescript
// Auth
LOGIN - User login
REGISTER - User registration
LOGOUT - User logout

// Products
ADD_PRODUCT - Create product (vendor)
UPDATE_PRODUCT - Edit product (vendor)
DELETE_PRODUCT - Remove product (vendor)

// Cart
ADD_TO_CART - Add item
UPDATE_CART_ITEM - Change quantity
REMOVE_FROM_CART - Remove item
CLEAR_CART - Empty cart

// Orders
CREATE_ORDER - Place order
UPDATE_ORDER_STATUS - Change status (vendor)

// Reviews
ADD_REVIEW - Submit product review

// Newsletter
SUBSCRIBE_NEWSLETTER - Email subscription

// Contact
SEND_CONTACT_MESSAGE - Contact form
```

---

## 🎨 Styling & UI

### Tailwind CSS Configuration
- Match existing Django template design
- Color scheme from original CSS
- Responsive breakpoints
- Component utilities

### Assets
- Use existing `/static/assets/` folder structure
- Copy images, icons, fonts to React `/public` folder
- Maintain same visual design

---

## 🔄 State Management

### Context/Zustand Stores
- `AuthContext` - User authentication state
- `CartStore` - Shopping cart state
- `UIStore` - Modals, notifications, loading states
- `VendorStore` - Vendor panel state

---

## 🚀 Routing Structure

```
/ - Home
/products - Product listing
/products/:slug - Product detail
/category/:slug - Category products
/vendors - Vendors grid
/vendor/:slug - Vendor profile
/cart - Shopping cart
/checkout - Checkout page
/search - Search results
/blog - Blog listing
/blog/:slug - Blog post
/contact - Contact page

/account/login - Customer login
/account/register - Customer register
/account/dashboard - Customer account
/account/orders - Order history
/account/orders/:id - Order detail

/vendor/login - Vendor login
/vendor/register - Vendor register
/vendor/dashboard - Vendor dashboard
/vendor/products - Products management
/vendor/products/add - Add product
/vendor/products/:id/edit - Edit product
/vendor/orders - Orders list
/vendor/orders/:id - Order detail
/vendor/payments - Payment history
/vendor/settings - Vendor settings

/admin/login - Admin login
/admin/dashboard - Admin panel
/admin/users - User management
/admin/products - Product moderation
/admin/orders - Order management
```

---

## ✅ Implementation Priority

### Phase 1: Foundation (Current)
- ✅ Project setup
- ✅ GraphQL API configured
- ✅ Apollo Client setup
- ✅ TailwindCSS configuration
- ✅ Basic routing structure

### Phase 2: Core Customer Frontend
- [ ] Header & Footer components
- [ ] Home page with carousel & products
- [ ] Product listing page
- [ ] Product detail page
- [ ] Shopping cart
- [ ] Checkout flow

### Phase 3: Vendor Panel
- [ ] Vendor dashboard with stats
- [ ] Products management (list, add, edit)
- [ ] Orders management
- [ ] Payments page
- [ ] Settings pages

### Phase 4: Additional Features
- [ ] User authentication pages
- [ ] Account/profile pages
- [ ] Blog pages
- [ ] Search functionality
- [ ] Filters and sorting
- [ ] Reviews and ratings

### Phase 5: Admin Panel
- [ ] Admin dashboard
- [ ] User management
- [ ] Product moderation
- [ ] Site settings

---

## 📝 Notes

- Django templates use Bootstrap/custom CSS - maintain same look
- Preserve all functionality from Django version
- Use GraphQL API for all data
- Implement proper loading states
- Add error handling
- Mobile responsive
- SEO-friendly (use React Helmet)
- Performance optimization (lazy loading, code splitting)

---

## 🎯 Current Status

**✅ Completed:**
- Django backend with GraphQL API
- Basic React setup
- Apollo Client configured
- Initial vendor dashboard created

**🔄 In Progress:**
- Converting Django templates to React components
- Building complete UI based on templates

**📋 Next Steps:**
- Create shared components (Header, Footer)
- Build complete customer home page
- Build product pages
- Build vendor panel pages
