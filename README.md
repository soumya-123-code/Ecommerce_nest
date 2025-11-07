# 🛍️ Full-Stack NestJS + GraphQL + React eCommerce Platform

A modern, scalable multi-vendor eCommerce marketplace built with the latest technologies. Originally based on a Django application, now completely rewritten with NestJS, GraphQL, Prisma, and React.

## ✨ Features

### 🎯 Core Features
- **Multi-Vendor Marketplace** - Support for unlimited vendors with separate dashboards
- **GraphQL API** - Modern, efficient API with full type safety
- **4-Level Category Hierarchy** - Super → Main → Sub → Mini categories
- **Product Management** - Complete product catalog with images, variants, and ratings
- **Order Processing** - Full order lifecycle management with vendor splitting
- **Payment Integration** - Stripe, Razorpay, and PayPal support
- **User Management** - Customer, Vendor, and Admin roles with JWT authentication
- **Real-time Updates** - GraphQL subscriptions for live data
- **Blog System** - Built-in blog with comments and categories
- **Newsletter** - Email subscription management
- **Referral System** - 2.5% commission for referrals
- **Vendor Payments** - Automated vendor payout system

### 🔒 Security Features
- JWT-based authentication
- Role-based access control (RBAC)
- Password hashing with bcrypt
- Rate limiting
- CORS protection
- Helmet security headers
- Environment variable configuration

## 🏗️ Technology Stack

### Backend
- **Framework**: NestJS 10.4+ (Latest)
- **API**: GraphQL with Apollo Server 4.11+
- **Database**: PostgreSQL with Prisma ORM 6.1+
- **Authentication**: JWT with Passport.js
- **Validation**: class-validator & class-transformer
- **File Upload**: Multer with Sharp for image processing
- **Payment**: Stripe 17.4+, Razorpay 2.9+
- **Email**: Nodemailer with SendGrid
- **TypeScript**: 5.7+

### Frontend
- **Framework**: React 18.3+ with TypeScript
- **Build Tool**: Vite 6.0+ (Latest & Fastest)
- **GraphQL Client**: Apollo Client 3.11+
- **UI Library**: TailwindCSS 3.4+ with Radix UI
- **State Management**: Zustand 5.0+
- **Routing**: TanStack Router 1.96+
- **Forms**: React Hook Form 7.54+
- **Icons**: Lucide React

## 📁 Project Structure

```
Ecommerce_nest/
├── src/                          # Backend source code
│   ├── main.ts                   # Application entry point
│   ├── app.module.ts             # Root module
│   ├── common/                   # Shared utilities
│   │   ├── decorators/           # Custom decorators
│   │   └── guards/               # Authentication guards
│   ├── modules/                  # Feature modules
│   │   ├── auth/                 # Authentication & JWT
│   │   ├── users/                # User management
│   │   ├── products/             # Product catalog
│   │   ├── categories/           # Category hierarchy
│   │   ├── orders/               # Order processing
│   │   ├── payments/             # Payment processing
│   │   ├── vendors/              # Vendor management
│   │   ├── blog/                 # Blog system
│   │   ├── newsletter/           # Newsletter
│   │   ├── contact/              # Contact forms
│   │   └── settings/             # Site settings
│   └── prisma/                   # Database module
│       ├── prisma.module.ts
│       └── prisma.service.ts
├── prisma/                       # Prisma configuration
│   └── schema.prisma             # Database schema (60+ models)
├── client/                       # Frontend React application
│   ├── src/
│   │   ├── pages/                # Page components
│   │   │   ├── Home.tsx          # Landing page
│   │   │   ├── Login.tsx         # Login page
│   │   │   ├── AdminDashboard.tsx  # Admin panel
│   │   │   └── VendorDashboard.tsx # Vendor panel
│   │   ├── lib/                  # Utilities
│   │   │   └── apollo-client.ts  # GraphQL client config
│   │   ├── App.tsx               # Root component
│   │   ├── main.tsx              # Entry point
│   │   └── index.css             # Global styles
│   ├── vite.config.ts            # Vite configuration
│   └── package.json              # Frontend dependencies
├── .env                          # Environment variables
├── .env.example                  # Environment template
├── package.json                  # Backend dependencies
├── tsconfig.json                 # TypeScript config
├── nest-cli.json                 # NestJS CLI config
└── README.md                     # This file
```

## 🚀 Getting Started

### Prerequisites

- Node.js 18+ or 20+ (LTS)
- PostgreSQL 14+
- npm or yarn
- Git

### Installation

1. **Clone the repository**
```bash
git clone <repository-url>
cd Ecommerce_nest
```

2. **Install backend dependencies**
```bash
npm install
```

3. **Install frontend dependencies**
```bash
cd client
npm install
cd ..
```

4. **Set up environment variables**
```bash
cp .env.example .env
```

Edit `.env` and configure:
- `DATABASE_URL` - Your PostgreSQL connection string
- `JWT_SECRET` - Strong secret key for JWT
- `STRIPE_SECRET_KEY` - Your Stripe secret key
- `RAZORPAY_KEY_ID` and `RAZORPAY_KEY_SECRET` - Razorpay credentials
- `EMAIL_*` - Email configuration (SendGrid)

5. **Set up the database**
```bash
# Generate Prisma client
npx prisma generate

# Run migrations
npx prisma migrate dev --name init

# (Optional) Seed the database
npx prisma db seed
```

6. **Start the development servers**

**Backend** (Terminal 1):
```bash
npm run start:dev
```
The GraphQL API will be available at http://localhost:4000/graphql

**Frontend** (Terminal 2):
```bash
cd client
npm run dev
```
The React app will be available at http://localhost:3000

## 📚 Database Schema

The application uses Prisma ORM with PostgreSQL. The schema includes:

### Core Models
- **User** - User accounts with roles (Admin, Customer, Vendor)
- **Profile** - Extended user profiles with vendor/customer distinction
- **BankAccount** - Vendor banking information
- **SocialLink** - Vendor social media links

### Product Models
- **Product** - Products with 5 images, pricing, SKU, stock
- **ProductImage** - Additional product images
- **ProductSize** - Product size variations
- **ProductRating** - Customer reviews (1-5 stars)

### Category Models (4-Level Hierarchy)
- **SuperCategory** - Top-level categories
- **MainCategory** - Second-level categories
- **SubCategory** - Third-level categories
- **MiniCategory** - Fourth-level categories

### Order Models
- **Order** - Main orders with coupon support
- **OrderDetails** - Order line items
- **OrderSupplier** - Vendor-specific orders
- **OrderDetailsSupplier** - Vendor order items
- **Coupon** - Discount coupons
- **Payment** - Payment and shipping information
- **Country** - Country list for shipping

### Payment Models
- **VendorPayment** - Vendor payout requests

### Content Models
- **Post** - Blog posts with rich content
- **Comment** - Blog comments
- **Newsletter** - Email subscriptions
- **Contact** - Contact form submissions

### Settings Models
- **SiteSetting** - Global site configuration
- **ContactInfo** - Contact information
- **SupportNumber** - Support phone numbers
- **SiteSocialLinks** - Site social media links
- **HomePageTheme** - Homepage theme selection

### Advertisement Models
- **Carousel** - Homepage carousel/slider
- **HomeAdSidebar** - Sidebar advertisements
- **HomeAdMiddlebar** - Middle bar ads
- **HomeAdDealTime** - Deal time limited offers
- **HotDealAd** - Hot deal promotions
- **ShopAdSidebar** - Shop page sidebar ads

## 🔐 Authentication & Authorization

### JWT Authentication
The application uses JWT tokens for authentication:

**Register a new user:**
```graphql
mutation Register {
  register(registerInput: {
    email: "user@example.com"
    username: "johndoe"
    password: "securepass123"
    firstName: "John"
    lastName: "Doe"
    role: CUSTOMER
  }) {
    accessToken
    user {
      id
      email
      username
      role
    }
  }
}
```

**Login:**
```graphql
mutation Login {
  login(loginInput: {
    usernameOrEmail: "johndoe"
    password: "securepass123"
  }) {
    accessToken
    user {
      id
      email
      username
      role
    }
  }
}
```

**Get current user:**
```graphql
query Me {
  me {
    id
    email
    username
    role
    profile {
      displayName
      bio
    }
  }
}
```

### Role-Based Access Control

Three main roles:
- **ADMIN** - Full platform access
- **VENDOR** - Manage own products and orders
- **CUSTOMER** - Browse and purchase products

Guards protect routes:
- `@UseGuards(GqlAuthGuard)` - Require authentication
- `@Roles(UserRole.ADMIN)` - Require specific role
- `@Public()` - Public endpoints (no auth required)

## 📦 API Examples

### Products

**Get all products:**
```graphql
query Products {
  products(skip: 0, take: 20)
}
```

**Get single product:**
```graphql
query Product {
  product(id: "product-id")
}
```

### Users

**Get all users (Admin only):**
```graphql
query Users {
  users {
    id
    email
    username
    role
    profile {
      displayName
      status
    }
  }
}
```

## 🎨 Frontend Features

### Customer Frontend
- Product browsing with search and filters
- Shopping cart and checkout
- Order tracking
- Product reviews and ratings
- User profile management

### Vendor Dashboard
- Product management (CRUD)
- Order fulfillment
- Sales analytics
- Earnings and payouts
- Inventory management

### Admin Panel
- User management
- Product approval and moderation
- Order management
- Platform analytics
- Site settings and configuration
- Payment gateway management

## 🔧 Development Commands

### Backend
```bash
# Development mode (watch)
npm run start:dev

# Production build
npm run build
npm run start:prod

# Run tests
npm run test

# Linting
npm run lint

# Format code
npm run format

# Prisma commands
npx prisma studio          # Open Prisma Studio
npx prisma generate        # Generate Prisma Client
npx prisma migrate dev     # Create migration
npx prisma db push         # Push schema changes
```

### Frontend
```bash
cd client

# Development
npm run dev

# Production build
npm run build

# Preview production build
npm run preview

# Lint
npm run lint
```

## 🌐 Environment Variables

### Required Variables
```env
# Application
NODE_ENV=development
PORT=4000
API_URL=http://localhost:4000

# Database
DATABASE_URL="postgresql://user:password@localhost:5432/ecommerce_db"

# JWT
JWT_SECRET=your-super-secret-jwt-key
JWT_EXPIRES_IN=7d

# Frontend URLs
FRONTEND_URL=http://localhost:3000
ADMIN_URL=http://localhost:3001
VENDOR_URL=http://localhost:3002
```

### Optional Variables
```env
# Stripe
STRIPE_SECRET_KEY=sk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...

# Razorpay
RAZORPAY_KEY_ID=...
RAZORPAY_KEY_SECRET=...

# PayPal
PAYPAL_CLIENT_ID=...
PAYPAL_CLIENT_SECRET=...
PAYPAL_MODE=sandbox

# Email
EMAIL_HOST=smtp.sendgrid.net
EMAIL_PORT=587
EMAIL_USER=apikey
EMAIL_PASSWORD=...
EMAIL_FROM=noreply@yourstore.com

# AWS S3 (Optional)
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=...
AWS_SECRET_ACCESS_KEY=...
AWS_S3_BUCKET=...
```

## 🚢 Deployment

### Backend Deployment

**Recommended platforms:**
- Railway
- Heroku
- AWS (ECS, Elastic Beanstalk)
- Google Cloud Run
- DigitalOcean App Platform

**Environment:**
- Set `NODE_ENV=production`
- Configure production database URL
- Set strong `JWT_SECRET`
- Configure payment gateway credentials

### Frontend Deployment

**Recommended platforms:**
- Vercel
- Netlify
- Cloudflare Pages
- AWS Amplify

**Build command:** `npm run build`
**Output directory:** `dist`

### Database

**Production database options:**
- Railway PostgreSQL
- Supabase
- Heroku Postgres
- AWS RDS
- DigitalOcean Managed Databases

## 🧪 Testing

```bash
# Unit tests
npm run test

# E2E tests
npm run test:e2e

# Test coverage
npm run test:cov
```

## 📝 Migration from Django

This application was originally built with Django and has been completely rewritten using modern JavaScript/TypeScript technologies. Key improvements:

✅ **Modern Stack**: NestJS + GraphQL instead of Django REST Framework
✅ **Type Safety**: Full TypeScript implementation
✅ **Better Performance**: GraphQL reduces over-fetching
✅ **Improved DX**: Hot reload, better tooling, modern IDE support
✅ **Scalable**: Microservices-ready architecture
✅ **Latest Packages**: All dependencies updated to 2024/2025 versions

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License.

## 👥 Support

For support, email support@yourstore.com or join our Slack channel.

## 🎉 Acknowledgments

- Original Django codebase
- NestJS team for the amazing framework
- Prisma team for the excellent ORM
- All open-source contributors

---

**Built with ❤️ using the latest web technologies**

NestJS 10.4+ | GraphQL 16.9+ | Prisma 6.1+ | React 18.3+ | TypeScript 5.7+ | Vite 6.0+