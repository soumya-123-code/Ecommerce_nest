# Django E-Commerce Codebase Comprehensive Analysis
## Project Structure and Architecture Overview

---

## 1. DATABASE MODELS (Core Data Structures)

### 1.1 ACCOUNTS & USER MANAGEMENT

#### **Profile Model** (`accounts/models.py`)
- **Purpose**: Extended user profile with vendor/customer distinction
- **Fields**:
  - `image`: Profile picture (ImageField)
  - `user`: OneToOne relationship with Django User
  - `display_name`: CharField(100)
  - `bio`: TextField
  - `mobile_number`, `address`, `city`, `post_code`, `country`, `state`: CharField(100)
  - `is_verified`: BooleanField (email/mobile verification)
  - `status`: CharField with choices ('customer', 'vendor')
  - `admission`: BooleanField (vendor approval status)
  - `code`: CharField(250) - referral code
  - `recommended_by`: ForeignKey(User) - referral tracking
  - `referrals`: IntegerField (count of referrals)
  - `blance` (misspelled balance): FloatField - vendor wallet balance
  - `requested`: FloatField - payment request amount
  - `slug`: SlugField (unique) - profile URL slug
  - `date`, `date_update`: DateTime fields
- **Business Logic**:
  - Auto-creates on User creation via signal
  - Auto-generates unique slug from username
  - Tracks referral relationships
  - Manages vendor wallet system

#### **BankAccount Model** (`accounts/models.py`)
- **Purpose**: Vendor payment method storage
- **Fields**:
  - `vendor_profile`: OneToOne to Profile
  - `bank_name`, `account_number`, `swift_code`, `account_name`: CharField
  - `country`: CharField
  - `paypal_email`: CharField
  - `description`: TextField
  - `date`, `date_update`: DateTime

#### **SocialLink Model** (`accounts/models.py`)
- **Purpose**: Vendor social media links
- **Fields**:
  - `vendor_profile`: OneToOne to Profile
  - `facebook`, `twitter`, `instagram`, `pinterest`: CharField

---

### 1.2 PRODUCTS & CATEGORIES

#### **SuperCategory → MainCategory → SubCategory → MiniCategory** Hierarchy
- **SuperCategory Model**:
  - `name`: CharField(50)
  - `category_image`: ImageField (120x120px recommended)
  - `slug`: SlugField (unique)
  - `date`, `date_update`: DateTime

- **MainCategory Model**:
  - FK to SuperCategory
  - Same fields as SuperCategory

- **SubCategory Model**:
  - FK to MainCategory
  - Same fields structure

- **MiniCategory Model**:
  - FK to SubCategory
  - Same fields structure

#### **Product Model** (`products/models.py`)
- **Core Product Information**:
  - `product_vendor`: FK to Profile (seller)
  - `product_name`: CharField(150)
  - `product_description`: TextField (short)
  - `content`: RichTextField (full description with CKEditor)
  - `product_image`: ImageField (main image, compressed to WebP)
  - `additional_image_1/2/3/4`: ImageField (gallery images, compressed)
  
- **Categorization**:
  - `product_minicategor`: FK to MiniCategory
  - `product_subcategory`: FK to SubCategory
  - `product_maincategory`: FK to MainCategory
  - `product_supercategory`: FK to SuperCategory

- **Pricing & Inventory**:
  - `PRDPrice`: FloatField (regular price)
  - `PRDDiscountPrice`: FloatField (sale price)
  - `available`: PositiveIntegerField (stock count)
  - `PRDSKU`: CharField(100) (product SKU)
  - `pieces`: PositiveIntegerField (items per set)

- **Dimensions & Weight**:
  - `width`, `height`: FloatField
  - `PRDWeight`: DecimalField (kg)

- **Status & Flags**:
  - `PRDISactive`: BooleanField (active/inactive)
  - `PRDISDeleted`: BooleanField (soft delete)
  - `promotional`: CharField with choices ('New', 'Hot')
  - `PRDISSale`: BooleanField (on sale flag)

- **Ratings & Reviews**:
  - `feedbak_average`: PositiveIntegerField (0-100 scale)
  - `feedbak_number`: PositiveIntegerField (review count)

- **Metadata**:
  - `PRDtags`: CharField(100) - search tags
  - `PRDSlug`: SlugField (unique, auto-generated)
  - `digital_file`: FileField (for digital products, .zip/.rar only)
  - `date`, `date_update`: DateTime

- **Business Logic**:
  - Image compression on save (converts to WebP, max 1100x1100px)
  - Auto-slug generation with uniqueness handling
  - Tracks digital product files

#### **ProductImage Model**
- `PRDIProduct`: FK to Product
- `PRDIImage`: ImageField (compressed on save)

#### **ProductRating Model**
- `PRDIProduct`: FK to Product
- `vendor`: FK to Profile
- `rate`: PositiveIntegerField (1-5 validation)
- `client_name`: FK to Profile (reviewer)
- `client_comment`: CharField(100)
- `active`: BooleanField
- `rating_date`, `rating_update`: DateTime

#### **ProductSize Model**
- `PRDIProduct`: FK to Product
- `name_variation`: CharField (XXS, XS, S, M, L, XL, XXL)

---

### 1.3 ORDERS & PAYMENTS

#### **Order Model** (`orders/models.py`)
- **Core Order Data**:
  - `user`: FK to User (buyer, nullable)
  - `email_client`: EmailField (guest checkout support)
  - `details`: ManyToMany to Product through OrderDetails
  
- **Financial Data**:
  - `sub_total`: CharField (calculated total)
  - `discount`: CharField (coupon/discount amount)
  - `shipping`: CharField (shipping cost)
  - `amount`: CharField (final total)
  - `coupon`: FK to Coupon (nullable)

- **Status & Tracking**:
  - `status`: CharField with choices ('PENDING', 'Underway', 'COMPLETE', 'Refunded')
  - `is_finished`: BooleanField
  - `tracking_no`: CharField
  - `rpt_cache`: URLField

- **Metadata**:
  - `order_date`, `date_update`: DateTime
  - `weight`: DecimalField (total order weight)
  - `merchant_order_id`, `order_id_paymob`: CharField (payment provider IDs)
  - `auth_token_order`, `trnx_id`: TextField/CharField

- **Business Logic in save()**:
  - When status = "PENDING": updates all OrderSupplier with same status
  - When status != "PENDING": marks order as finished, calculates 2.5% referral bonus
  - Referral system: adds bonus to recommended_by user's balance

#### **OrderDetails Model**
- `supplier`: FK to User (vendor)
- `product`: FK to Product
- `order`: FK to Order
- `price`: DecimalField (unit price at time of order)
- `quantity`: IntegerField
- `size`: CharField (selected size variation)
- `weight`: DecimalField (calculated from product)

#### **OrderSupplier Model** (Vendor-specific order view)
- `user`: FK to User (buyer)
- `email_client`: EmailField
- `vendor`: FK to Profile (seller)
- `order`: FK to Order (parent order)
- `coupon`: FK to Coupon
- `sub_total`, `discount`, `shipping`, `amount`: CharField
- `weight`: DecimalField
- `status`: CharField ('PENDING', 'Underway', 'COMPLETE', 'Refunded')
- `is_finished`: BooleanField
- `order_date`, `date_update`: DateTime

- **Business Logic in save()**:
  - When status = "Underway": calculates 2.5% referral bonus
  - Updates vendor's balance with referral bonuses

#### **OrderDetailsSupplier Model**
- Similar to OrderDetails but vendor-specific
- Links to OrderSupplier and OrderDetails
- Tracks supplier-level item details

#### **Coupon Model**
- `code`: CharField(50, unique)
- `valid_form`, `valid_to`: DateTimeField
- `discount`: PositiveIntegerField (0-100 percentage)
- `active`: BooleanField

#### **Payment Model**
- `order`: FK to Order
- `first_name`, `last_name`, `street_address`, `City`: CharField
- `country`, `country_code`, `state`: CharField
- `post_code`: CharField(10)
- `Email_Address`: EmailField
- `phone`: CharField(20)
- `payment_method`: CharField

#### **Country Model**
- `name_country`: CharField(40)
- `country_code`: CharField(40)
- `countries`: CountryField

---

### 1.4 PAYMENTS & VENDOR TRANSACTIONS

#### **VendorPayments Model** (`payments/models.py`)
- `vendor_profile`: FK to User
- `request_amount`: FloatField (amount requested)
- `fee`: FloatField (platform fee)
- `description`: TextField
- `status`: CharField with choices ('Paid', 'Pending', 'Progressing', 'Refunded')
- `method`: CharField with choices ('Bank', 'Paypal')
- `comment`: TextField
- `date`, `date_update`: DateTime

---

### 1.5 BLOG & CONTENT

#### **Post Model** (`blog/models.py`)
- `title`: CharField(500)
- `content`: RichTextField (with CKEditor)
- `post_date`, `post_update`: DateTime
- `post_supercategory`: FK to SuperCategory
- `author`: FK to Profile
- `post_image`: ImageField
- `views`: IntegerField (view count)
- `posttags`: CharField(500)
- `post_Slug`: SlugField (unique, auto-generated)

#### **Comment Model** (`blog/models.py`)
- `name`, `email`, `body`: CharField/TextField
- `comment_date`, `active`: DateTime/Boolean
- `post`: FK to Post
- `profile`: FK to Profile (nullable)

---

### 1.6 CONTACT & NEWSLETTERS

#### **Newsletter Model** (`newsletters/models.py`)
- `email`: EmailField (unique)
- `subscribed`: BooleanField
- `created_At`: DateTime

#### **MessagesList Model** (`contact/models.py`)
- `name`, `email`, `phone`, `subject`: CharField/EmailField
- `message`: TextField
- `date`, `date_update`: DateTime

---

### 1.7 SETTINGS & CONFIGURATION

#### **SiteSetting Model** (`settings/models.py`)
- `site_name`: CharField(150)
- `site_title`: CharField(500)
- `description`: TextField(500)
- `site_url`: URLField
- `site_logo`, `favicon`, `login_image`, `footer_image`: ImageField
- `shipping`: FloatField (shipping fee)
- `date`, `date_update`: DateTime

#### **ContactInfo Model**
- `description`, `full_address`, `phone`, `email`: CharField/EmailField
- `Work_time`: CharField(150)
- `contact_date`: DateTime
- `map_link`: URLField
- `active`: BooleanField

#### **SocailLinks Model**
- `facebook`, `twitter`, `youtube`, `pinterest`, `instagram`: URLField

#### **SupportNumber Model**
- `number`, `Work_time`: CharField
- `date`, `date_update`: DateTime

#### **HomePageTheme Model**
- `page_name`: CharField(150)
- `active`: BooleanField
- `date`, `date_update`: DateTime

---

### 1.8 HOME PAGE ADS & MARKETING

#### **Carousel Model** (`home/models.py`)
- `CARImage`: ImageField (1372x830px recommended)
- `CARtitle`, `CARURL`: CharField/URLField

#### **HomeAdSidebar Model**
- `ad_mage`: ImageField (760x596px, 250KB max)
- `ad_title`: CharField
- `ad_URL`: URLField
- `image_position`: CharField (Left/Right)

#### **HomeAdMiddlebar, HomeAdSupplier, HomeAdDaily, HomeAdDealTime** Models
- Similar ad structures with different dimensions
- HomeAdDealTime includes: `supplier` FK, `PRDPrice`, `PRDDiscountPrice`, `PRDdealtime`

#### **VendorDetailsAdImage, ShopAdSidebar, HotDealAd, HeadTextAd** Models
- Various ad placement options across the site

---

### 1.9 REPORTS & ANALYTICS

#### **PostView Model** (`reports/models.py`)
- `post`: FK to Post
- `ip`, `session`, `referral`: CharField
- `user_agent`, `user_agent_browser`, `user_agent_os`, `user_agent_device`: CharField
- `is_mobile`, `is_tablet`, `is_touch_capable`, `is_pc`, `is_bot`: BooleanField
- `created`, `date`, `date_update`: DateTime

#### **PostReport Model**
- `impressions`: IntegerField
- `created`, `date`, `date_update`: DateTime
- `publisher`: FK to User
- `post`: FK to Post

---

## 2. VIEWS & ENDPOINTS

### 2.1 PRODUCTS APP

#### **product_details(request, slug)** - GET
- Fetches product by slug
- Gets product variations (sizes)
- Gets product images
- Calculates related products by mini-category
- Computes average rating and star distribution (1-5 breakdown)
- **Template**: `products/shop-product-vendor.html`

#### **product_search(request)** - GET/POST
- Filters products by name and category
- Implements pagination (12 items per page)
- Session-based search state
- **Template**: `products/product-search.html`

#### **product_rating(request)** - POST (AJAX)
- Creates/updates product ratings (1-5 stars)
- Validates authenticated user
- Updates product rating metrics
- Returns JSON response

---

### 2.2 ACCOUNTS APP

#### **register(request)** - GET/POST
- User registration with password hashing
- Form validation via UserCreationForm
- Redirects to login on success
- **Template**: `accounts/page-register.html`

#### **login_user(request)** - GET/POST
- Authenticates by username or email
- Session management
- **Template**: `accounts/page-login.html`

#### **logout_user(request)** - GET
- Clears session

#### **dashboard_customer(request)** - GET/POST
- Authenticated users only
- Profile update form handling
- Image upload processing
- **Template**: `accounts/page-account.html`

#### **dashboard_account_details(request)** - GET/POST
- Extended profile update
- Image validation
- **Template**: `accounts/account-details.html`

#### **change_password(request)** - POST
- Password reset with session hash update
- **Template**: `accounts/change-password.html`

#### **MyOrdersJsonListView** - AJAX GET
- Returns paginated user orders (10 per request)
- Parameters: `num_products` (offset)

#### **order(request, order_id)** - GET
- Displays completed order details
- Shows line items and totals
- **Template**: `accounts/order-archive.html`

#### **download_list(request)** - GET
- Lists digital products purchased by user
- **Template**: `accounts/download-page.html`

#### **download_file(request, order_id, filename)** - GET
- Serves digital product files
- Validates order ownership
- Returns .zip/.rar files

---

### 2.3 ORDERS APP (Very Complex - Checkout & Payment Flow)

#### **add_to_cart(request)** - POST
- Validates product availability
- Creates/updates Order and OrderDetails
- Handles OrderSupplier creation
- Calculates totals and weights
- Manages cart for authenticated and anonymous users
- Session-based cart_id for guests

#### **cart(request)** - GET/POST
- Displays current cart
- Handles quantity updates
- Shows order breakdown

#### **StatesJsonListView** - AJAX GET
- Returns states for selected country
- Parameters: `country` code

#### **remove_item(request, productdeatails_id)** - POST
- Removes product from cart
- Recalculates totals

#### **payment(request)** - GET/POST
- Displays payment form
- Collects shipping address
- Handles form submission

#### **payment_blance(request)** - POST
- Payment from vendor wallet balance
- Deducts from vendor's balance field

#### **payment_cash(request)** - POST
- Cash on delivery payment method

#### **create_checkout_session(request)** - POST
- Creates Stripe checkout session
- Returns session ID for client-side redirect

#### **verify_payment_razorpay(request)** - POST
- Verifies Razorpay payment signature
- Creates Payment record
- Updates order status

#### **verify_payment_paypal(request)** - POST
- Verifies PayPal payment
- Updates order status

#### **checkout_payment_paymob(request, id)** - POST
- Paymob payment integration
- Returns transaction data

#### **my_webhook_view(request)** - POST (CSRF exempt)
- Stripe webhook handler
- Updates order status on successful payment

#### **my_webhook_view_paymob(request)** - POST (CSRF exempt)
- Paymob webhook callback

#### **send_payment_fatoorah(request, id)** - POST
- MyFatoorah payment gateway integration

#### **callback_url_fatoorah(request)** - POST
- MyFatoorah payment callback

#### **success(request)** - GET
- Order completion page
- **Template**: Order success confirmation

#### **CancelView** - GET
- Order cancellation

---

### 2.4 SUPPLIERS APP

#### **supplier_list(request)** - GET
- **Template**: `suppliers/vendors-grid.html`

#### **VendorsJsonListView** - AJAX GET
- Paginated vendor list (12 per request)
- Filters by status='vendor'
- Parameters: `num_vendors`

#### **vendor_details(request, slug)** - GET
- Shows vendor profile and social links
- **Template**: `suppliers/vendor-details.html`

#### **VendorDetailsJsonListView** - AJAX GET
- Lists vendor's products (10 per request)
- Parameters: `num_products`, `order_by`, `vendor_slug`

---

### 2.5 SUPPLIER PANEL (Vendor Dashboard)

#### **supplier_dashboard(request)** - GET
- Vendor-only (decorator: @vendor_only)
- Shows vendor orders, products, and underway orders
- **Template**: `supplier-panel/index.html`

#### **supplier_login(request)** - GET/POST
- Vendor login by username or email
- Checks vendor status and admission flag

#### **supplier_register(request)** - GET/POST
- New vendor registration

#### **supplier_add_product(request)** - GET/POST
- Creates new product listing
- Handles image uploads

#### **CategoriesJsonListView** - AJAX GET
- Returns category hierarchy for product creation

#### **supplier_products_list(request)** - GET
- Lists vendor's products
- **Template**: Product management page

#### **SupplierProductsJsonListView** - AJAX GET
- Paginated product list for vendor

#### **remove_product(request, id)** - POST
- Soft-deletes vendor's product

#### **supplier_edit_product(request, id)** - GET/POST
- Updates product details

#### **supplier_orders_list(request)** - GET
- Lists vendor's orders
- **Template**: Order management page

#### **SupplierOrdersJsonListView** - AJAX GET
- Paginated orders for vendor

#### **supplier_orders_detail(request, id)** - GET
- Shows order details for vendor
- **Template**: Order detail page

#### **supplier_reviews(request)** - GET
- Lists product reviews for vendor
- **Template**: Reviews management

#### **bank_info(request)** - GET/POST
- Manages vendor bank account details

#### **social_links(request)** - GET/POST
- Manages vendor social media links

#### **payments(request)** - GET
- Lists vendor payment history

#### **request_payment(request)** - POST
- Vendor requests payment/withdrawal

#### **chartJsonListView** - AJAX GET
- Returns monthly product and order counts for charts

#### **chartJsonListViewAdmin** - AJAX GET
- Admin chart data for all platform

---

### 2.6 CATEGORIES APP

#### **shop(request)** - GET
- Main shop page
- **Template**: `categories/shop-grid-left.html`

#### **super_category(request, slug)** - GET
- Lists main categories under selected super-category
- **Template**: `categories/shop-super-category.html`

#### **main_category(request, slug)** - GET
- Lists sub-categories
- **Template**: `categories/shop-main-category.html`

#### **sub_category(request, slug)** - GET
- Lists mini-categories
- **Template**: `categories/shop-sub-category.html`

#### **category_list(request)** - GET
- Displays all categories
- **Template**: `categories/category-list.html`

#### **CategoryJsonListView** - AJAX GET
- Paginated products by category type (super/main/sub/mini)
- Parameters: `num_products`, `order_by`, `CAT_id`, `cat_type`

---

### 2.7 HOME APP

#### **home_page(request)** - GET
- Displays home page with:
  - Super categories (random)
  - Carousels
  - Home ads (sidebar left/right, middlebar)
  - Main categories
  - Products (random, active only)
  - Deal time ads
  - Theme-based rendering
- **Template**: Dynamic (index-1.html to index-4.html)

#### **set_currency(request)** - POST
- Sets session currency
- Stores in `request.session['currency']`

---

### 2.8 BLOG APP

#### **home(request)** - GET (Blog home)
- Lists all posts paginated (16 per page)
- **Template**: `blog/blog-fullwidth.html`

#### **super_category(request, slug)** - GET
- Lists posts in category
- **Template**: `blog/blog-category-fullwidth.html`

#### **post_detail(request, slug)** - GET/POST
- Displays full post with comments
- Tracks post views (IP-based, one per session per day)
- Handles comment submission
- Records analytics: browser, OS, device, mobile flag, bot detection
- **Template**: `blog/blog-post-fullwidth.html`

---

### 2.9 CONTACT APP

#### **contact(request)** - GET/POST
- Contact form submission
- Creates MessagesList record
- **Template**: `contact/page-contact.html`

---

### 2.10 NEWSLETTERS APP

#### **letter(request)** - POST (AJAX)
- Newsletter subscription
- Duplicate email handling
- Returns JSON with success/error

---

### 2.11 PAGES APP

#### Views for static pages (not detailed in views.py)

---

## 3. URL PATTERNS & ROUTING

```
/                                  → home:index (home_page)
/set_currency/                     → home:set-currency

/product-details/<slug>/           → products:product-details
/product-search/                   → products:product-search
/rating/                           → products:product_rating (AJAX)

/register/                         → accounts:register
/login/                            → accounts:login
/logout/                           → accounts:logout
/dashboard/                        → accounts:dashboard_customer
/account_details/                  → accounts:account_details
/change-password/                  → accounts:change_password
/orders-ajax/                      → accounts:orders-ajax (AJAX)
/dashboard/order/<id>/             → accounts:order
/download-list/                    → accounts:download-list
/download_file/<id>/<filename>/    → accounts:download-file
/password-reset/                   → auth:PasswordResetView
/password-reset/done/              → auth:PasswordResetDoneView
/password-confirm/<uidb64>/<token>/→ auth:PasswordResetConfirmView
/password-complete/                → auth:PasswordResetCompleteView

/add_to_cart/                      → orders:add-to-cart
/cart/                             → orders:cart
/cart/<country>/                   → orders:get-states (AJAX)
/order/remeve-product/<id>/        → orders:remove-item
/payment/                          → orders:payment
/payment_blance/                   → orders:payment-blance
/payment_cash/                     → orders:payment-cash
/order/cancel/                     → orders:cancel
/order/success/                    → orders:success
/create_checkout_session/          → orders:create_checkout_session
/orders/webhook/                   → orders:my-webhook (Stripe)
/verify-payment/                   → orders:verify-payment (Razorpay)
/verify-payment-paypal/            → orders:verify-payment-paypal
/checkout-paymob/<id>/             → orders:checkout-paymob
/api/callbacks/                    → orders:webhook-view-paymob
/checkout-fatoorah/<id>/           → orders:checkout-fatoorah
/api/callbacks-myfatoorah/         → orders:callbacks-myfatoorah

/category-list/                    → categories:category-list
/shop/                             → categories:shop
/shop/super/<slug>/                → categories:super-category
/shop/main/<slug>/                 → categories:main-category
/shop/sub/<slug>/                  → categories:sub-category
/shop-ajax/                        → categories:shop-ajax (AJAX)

/supplier-list/                    → suppliers:supplier-list
/vendors-ajax/                     → suppliers:orders-ajax (AJAX)
/vendor-details/<slug>/            → suppliers:vendor-details
/vendor-details-ajax/              → suppliers:orders-ajax (AJAX)

/supplier-panel/                   → supplier_dashboard:supplier-panel
/chart-ajax/                       → supplier_dashboard:chart-ajax
/chart-ajax-admin/                 → supplier_dashboard:chart-ajax-admin
/supplier-login/                   → supplier_dashboard:supplier-login
/supplier-register/                → supplier_dashboard:supplier-register
/supplier-add-product/             → supplier_dashboard:supplier-add-product
/supplier-categories-ajax/         → supplier_dashboard:get-categories
/supplier-products-list/           → supplier_dashboard:supplier-products-list
/supplier-products-list-ajax/      → supplier_dashboard:supplier-products-list-ajax
/supplier-products/remeve-product/<id>/→ supplier_dashboard:remove-item
/supplier-edit-product/<id>/       → supplier_dashboard:supplier-edit-product
/supplier-orders-list/             → supplier_dashboard:supplier-orders-list
/supplier-reviews/                 → supplier_dashboard:supplier-reviews
/settings/bank-info/               → supplier_dashboard:bank-info
/settings/social-links/            → supplier_dashboard:social-links
/order-details/<id>/               → supplier_dashboard:order-details
/payments/                         → supplier_dashboard:payments
/request_payment/                  → supplier_dashboard:request-payment

/blog/                             → blog:home-blog
/category/<slug>/                  → blog:category
/detail/<slug>/                    → blog:postdetail

/contact/                          → contact:contact

/admin/                            → Django Admin
/captcha/                          → Captcha URLs
/graphql/                          → GraphQL API
/currencies/                        → Currency conversion
```

---

## 4. BUSINESS LOGIC & SERVICES

### 4.1 CART & ORDER MANAGEMENT
- **Shopping Cart**: Session-based for guests, database for authenticated users
- **Order Flow**: 
  1. Product added to cart (creates Order if not exists)
  2. Creates OrderDetails for each product
  3. Creates OrderSupplier for each vendor
  4. Creates OrderDetailsSupplier for supplier-specific items
  
- **Total Calculation**: Sums (price × quantity) for all items
- **Weight Calculation**: Sums (product weight × quantity)

### 4.2 REFERRAL SYSTEM
- **Mechanism**: 
  - When order status changes from PENDING to other states
  - Calculates 2.5% of order amount as bonus
  - Adds bonus to referred user's `blance` field
  
- **Tracking**: Profile.recommended_by field tracks referrer

### 4.3 PAYMENT GATEWAY INTEGRATIONS
- **Stripe**: Create checkout session, handle webhook
- **Razorpay**: Generate payment link, verify signature
- **PayPal**: Integration endpoints
- **Paymob**: Egyptian payment gateway
- **MyFatoorah**: Middle East payment solution
- **Wallet Payment**: Balance deduction from vendor account

### 4.4 VENDOR MANAGEMENT
- **Vendor-only Decorator**: `@vendor_only` ensures vendor status and admission
- **Vendor Dashboard**:
  - View orders received from customers
  - Manage product listings
  - Track payments and withdrawals
  - View product reviews
  - Update bank/social info

### 4.5 PRODUCT RATING & REVIEWS
- **Rating Calculation**: Average of all 1-5 star ratings
- **Star Distribution**: Percentage breakdown (1-star %, 2-star %, etc.)
- **Update Logic**: Recalculates on each new rating or rating update
- **Conversion**: Average rating stored as 0-100 scale (multiply by 20)

### 4.6 CONTENT MANAGEMENT
- **Blog Tracking**: IP-based view counting (one per IP per day)
- **Analytics**:
  - User agent parsing (browser, OS, device)
  - Device type detection (mobile, tablet, PC, bot)
  - Daily impressions tracking
  
- **Comments**: User-specific commenting with profile tracking

### 4.7 IMAGE PROCESSING
- **Automatic Compression**:
  - Products: Convert RGBA/P to RGB, max 1100×1100px, WebP format, quality 20
  - Categories: Store as-is
  
- **File Handling**: Compressed on upload and update

### 4.8 SLUG GENERATION
- **Auto-slug** from product name or username
- **Collision Handling**: Appends random 5-char code if slug exists
- **Recursive**: Checks for collision and retries if needed

### 4.9 AUTHENTICATION
- **Methods**:
  - Username authentication
  - Email authentication (lookup then authenticate by username)
  - Password reset via email
  - Session-based
  
- **Profile Auto-creation**: Signal creates Profile on User creation

### 4.10 CURRENCY SUPPORT
- **Session-based**: Stores in `request.session['currency']`
- **Default**: USD defined in settings
- **Integration**: Django-currencies app for conversion

---

## 5. MIDDLEWARE & CONTEXT PROCESSORS

### 5.1 Middleware Stack (project/settings.py)
1. `SecurityMiddleware`
2. `SessionMiddleware`
3. `CorsMiddleware` (for React frontend)
4. `CommonMiddleware`
5. `CsrfViewMiddleware`
6. `AuthenticationMiddleware`
7. `MessageMiddleware`
8. `XFrameOptionsMiddleware`
9. `WhiteNoiseMiddleware` (static files)

### 5.2 Context Processors
- **currencies**: Currency context
- **category_obj**: All categories
- **new_products_obj**: New products
- **orders_cart_obj**: Shopping cart
- **DealTime_obj**, **vendor_details_ad_image**, **shop_ad_sidebar**, **hot_deal_ad**, **head_text_ad**: Home page ads
- **socail_links_settings**, **contact_info_settings**, **support_number_settings**, **site_settings**: Site configuration
- **pages_list_obj**: Static pages

---

## 6. CONFIGURATION & SETTINGS

### 6.1 Database
- **Default**: SQLite (db.sqlite3)
- **Commented MySQL config** available (Nestali database)
- **ORM**: Django ORM with no raw SQL visible

### 6.2 Payment Gateway Keys
- **Stripe**:
  - Public: `pk_test_51JlJ7SDD5hTsFJUF...`
  - Secret: `sk_test_51JlJ7SDD5hTsFJUF...`
  - Webhook: `whsec_1xR7X9MTv6Qjfbt7...`
  
- **Razorpay**: Integrated (no keys in settings)
- **PayPal**: Integrated (no keys in settings)
- **Paymob**: Integrated
- **MyFatoorah**: Integrated

### 6.3 Email & Communication
- **SendGrid** integration for email
- **Debug Email**: info@mohamedselem.com
- **SMTP**: Django email backend

### 6.4 Static & Media
- **Static Root**: `/project/static/site_static/`
- **Media Root**: `/project/media/`
- **Storage**: WhiteNoise CompressedStaticFilesStorage

### 6.5 Installed Apps
```
Django core apps
+ AdminLTE3 theme
+ GraphQL (graphene-django)
+ REST Framework
+ CORS headers
+ CKEditor (rich text)
+ Captcha
+ Currencies
+ Custom apps: accounts, orders, products, etc.
```

---

## 7. GRAPHQL API (api/schema.py)

### 7.1 Object Types (All Models Mapped)
- UserType, ProfileType, BankAccountType, SocialLinkType
- CategoryTypes (Super, Main, Sub, Mini)
- ProductTypes (Product, ProductImage, ProductRating, ProductSize)
- OrderTypes (Order, OrderDetails, OrderSupplier, OrderDetailsSupplier, Coupon, Payment, Country)
- VendorPaymentType
- BlogTypes (Post, Comment)
- NewsletterType, ContactType
- SettingsTypes (SocialLinks, ContactInfo, SupportNumber, SiteSetting, HomePageTheme)
- HomeAdTypes (Carousel, HomeAdSidebar, HomeAdMiddlebar, etc.)

### 7.2 Query Root
- User queries: all_users, user, me (authenticated)
- Profile queries: all_profiles, profile, profile_by_slug
- Category queries: all categories by type
- Product queries: all_products (with limit/offset), product, product_by_slug, products_by_vendor, products_by_category
- Order queries: all_orders, order, orders_by_user
- Blog queries: all_posts, post, post_by_slug
- Newsletter & Contact queries
- Settings queries: site_settings, contact_info
- Ad queries: all_carousels, hot_deals

### 7.3 Mutations
- **CreateProduct**: product_name, description, price, sku → creates Product
- **UpdateProduct**: id, product_name, description, price → updates Product
- **DeleteProduct**: id → soft deletes Product
- **CreateOrder**: email_client, amount → creates Order
- **SubscribeNewsletter**: email → newsletter subscription
- **CreateContactMessage**: name, email, phone, subject, message → contact form

### 7.4 GraphQL Endpoint
- URL: `/graphql/`
- GraphiQL IDE enabled for development
- CSRF exempt

---

## 8. KEY UTILITIES & HELPERS

### 8.1 Slug Generation (accounts/utils.py, products/utils.py, categories/utils.py, blog/utils.py)
```python
def code_generator(size=5, chars=string.ascii_lowercase + string.digits):
    # Generates random 5-char code
    
def create_shortcode(instance):
    # Creates slug as: slugify(name)-random_code
    # Checks for duplicates recursively
```

### 8.2 Vendor Permission Check (supplier_panel/utils.py)
```python
@vendor_only
def function(request, *args, **kwargs):
    # Decorator checks:
    # - User is authenticated
    # - Profile exists
    # - status == "vendor"
    # - admission == True
```

### 8.3 IP Address Extraction (blog/views.py)
```python
def get_ip(request):
    # Returns X_FORWARDED_FOR or REMOTE_ADDR
```

---

## 9. KEY THIRD-PARTY LIBRARIES

- **Django 5.1.4**: Web framework
- **graphene-django 3.2.2**: GraphQL support
- **djangorestframework 3.15.2**: REST API
- **django-cors-headers**: CORS for React frontend
- **Pillow 11.0.0**: Image processing
- **stripe 11.2.0**: Stripe payments
- **razorpay 1.4.2**: Razorpay payments
- **beautifulsoup4 4.12.3**: Web scraping
- **django-ckeditor 6.7.1**: Rich text editor
- **celery 5.4.0**: Async tasks
- **redis 5.2.1**: Caching/task queue
- **whitenoise 6.8.2**: Static file serving
- **django-user-agents 0.4.0**: User agent parsing

---

## 10. IMPORTANT BUSINESS RULES

1. **Referral Commission**: 2.5% of order amount when status changes from PENDING
2. **Vendor Admission**: Only admitted vendors can access supplier panel
3. **Soft Delete**: Products marked deleted but not removed from database
4. **Order Status Flow**: PENDING → Underway → COMPLETE or Refunded
5. **Multi-Vendor**: Orders split into OrderSupplier per vendor
6. **Price Storage**: Prices stored as FloatField/DecimalField as strings in Order
7. **Stock Management**: Product.available decrements with OrderDetails.quantity
8. **Image Compression**: All product images auto-compressed to WebP on upload
9. **Cart Persistence**: Guests use session, authenticated users use database
10. **Payment Methods**: Multiple gateways (Stripe, Razorpay, PayPal, Paymob, MyFatoorah, Wallet)

---

## 11. SECURITY CONSIDERATIONS

1. **CSRF Protection**: Enabled except for webhook endpoints (csrf_exempt)
2. **Authentication**: Django's built-in system with session-based auth
3. **Authorization**: 
   - Vendor-only decorator for supplier panel
   - User checks for order ownership
   - Admin-only for admin endpoints
4. **File Uploads**: File extension validation (zip/rar only for digital products)
5. **Image Validation**: PIL validation on upload
6. **Payment Signatures**: Razorpay signature verification

---

## 12. DATABASE RELATIONSHIPS (ERD Summary)

```
User (Django)
├── Profile (1:1) - Extended user data, vendor status
├── BankAccount (via Profile 1:1)
├── SocialLink (via Profile 1:1)
├── Product (vendor_id) - Vendor has many products
├── Order (user_id) - User has many orders
└── OrderSupplier - Vendor's view of orders

Product
├── SuperCategory, MainCategory, SubCategory, MiniCategory
├── ProductImage (1:many) - Multiple images per product
├── ProductRating (1:many) - Multiple ratings per product
└── ProductSize (1:many) - Multiple sizes per product

Order (1:many OrderDetails)
├── OrderDetails (many:many through Product)
├── OrderSupplier (1:many) - Split by vendor
│   └── OrderDetailsSupplier (1:many)
├── Coupon (0:1)
└── Payment (1:1)

Post
├── SuperCategory (many:1)
├── Profile/Author (many:1)
└── Comment (1:many)
```

---

## 13. IMPORTANT NOTES FOR RUST MIGRATION

1. **No ORM Signals**: Use application-level logic for auto-profile creation
2. **Large Views**: Orders/payment views are complex; break into smaller services
3. **AJAX Heavy**: Many AJAX endpoints return JSON with pagination
4. **Multi-Currency**: Implement currency service
5. **File Processing**: Image compression needs library (imageproc, image crate)
6. **Slug Generation**: Implement collision detection for slugs
7. **Webhook Handlers**: Implement idempotency for payment webhooks
8. **Email Sending**: Integrate with SendGrid API
9. **Session Management**: Either use Redis or implement JWT
10. **GraphQL**: Use async-graphql or juniper crates
11. **Payment Gateways**: Multiple SDK integrations needed
12. **Admin Panel**: May need separate admin service or use existing one
13. **Template Engine**: No server-side rendering in REST API (React frontend handles it)

