# 🛍️ Full-Stack Django + GraphQL + React eCommerce Platform

A modern, scalable multi-vendor eCommerce marketplace built with **Django 5.1**, **GraphQL (Graphene-Django)**, and **React 18.3+**. All packages updated to the latest 2024/2025 versions with professional frontend dashboards for Admin, Vendor, and Customer roles.

## ✨ Features

### 🎯 Core Features
- **Multi-Vendor Marketplace** - Support for unlimited vendors with separate dashboards
- **GraphQL API** - Modern, efficient API powered by Graphene-Django
- **4-Level Category Hierarchy** - Super → Main → Sub → Mini categories
- **Product Management** - Complete product catalog with images, variants, and ratings
- **Order Processing** - Full order lifecycle management with vendor splitting
- **Payment Integration** - Stripe, Razorpay, PayPal, and MyFatoorah support
- **User Management** - Customer, Vendor, and Admin roles with Django authentication
- **Blog System** - Built-in blog with comments and categories
- **Newsletter** - Email subscription management
- **Referral System** - 2.5% commission for referrals
- **Vendor Payments** - Automated vendor payout system

### 🔒 Security Features
- Django's built-in authentication system
- Password hashing with PBKDF2
- CORS protection for React frontend
- CSRF protection
- Session-based authentication
- Admin panel with permission controls

## 🏗️ Technology Stack

### Backend
- **Framework**: Django 5.1.4 (Latest LTS)
- **API**: GraphQL with Graphene-Django 3.2.2
- **Database**: SQLite (Development) / PostgreSQL or MySQL (Production)
- **Authentication**: Django Auth System
- **ORM**: Django ORM
- **Payment**: Stripe 11.2+, Razorpay 1.4+
- **Email**: SendGrid integration
- **Python**: 3.8+

### Frontend
- **Framework**: React 18.3+ with TypeScript
- **Build Tool**: Vite 6.0+ (Latest & Fastest)
- **GraphQL Client**: Apollo Client 3.11+
- **UI Library**: TailwindCSS 3.4+ with Radix UI
- **State Management**: Zustand 5.0+
- **Forms**: React Hook Form 7.54+
- **Icons**: Lucide React

## 📁 Project Structure

```
Ecommerce_nest/
├── project/                      # Django project settings
│   ├── settings.py              # Main settings (GraphQL, CORS configured)
│   ├── urls.py                  # URL routing with GraphQL endpoint
│   └── wsgi.py                  # WSGI configuration
├── api/                         # GraphQL API app
│   ├── schema.py                # Complete GraphQL schema
│   └── __init__.py
├── accounts/                    # User profiles & authentication
├── products/                    # Product catalog
├── categories/                  # 4-level category system
├── orders/                      # Order management
├── payments/                    # Payment processing
├── suppliers/                   # Vendor management
├── blog/                        # Blog system
├── newsletters/                 # Newsletter subscriptions
├── contact/                     # Contact forms
├── settings/                    # Site settings
├── home/                        # Homepage & advertisements
├── client/                      # React frontend
│   ├── src/
│   │   ├── pages/              # React pages (Home, Admin, Vendor)
│   │   ├── lib/                # Apollo client config
│   │   ├── App.tsx             # Main app component
│   │   └── main.tsx            # Entry point
│   ├── vite.config.ts          # Vite configuration
│   └── package.json            # Frontend dependencies
├── requirements.txt             # Python dependencies (ALL LATEST!)
├── manage.py                    # Django management command
└── README.md                    # This file
```

## 🚀 Getting Started

### Prerequisites

- Python 3.8+ or 3.10+ (recommended)
- Node.js 18+ or 20+ (LTS)
- SQLite (included) or PostgreSQL/MySQL
- npm or yarn
- Git

### Installation

#### 1. **Clone the repository**
```bash
git clone <repository-url>
cd Ecommerce_nest
```

#### 2. **Backend Setup (Django)**

Create virtual environment:
```bash
python -m venv venv

# Activate virtual environment
# On Windows:
venv\Scripts\activate
# On macOS/Linux:
source venv/bin/activate
```

Install Python dependencies:
```bash
pip install -r requirements.txt
```

Run migrations:
```bash
python manage.py makemigrations
python manage.py migrate
```

Create superuser:
```bash
python manage.py createsuperuser
```

Start Django server:
```bash
python manage.py runserver
```

Django will run on: http://localhost:8000
GraphQL API: http://localhost:8000/graphql/

#### 3. **Frontend Setup (React)**

Install frontend dependencies:
```bash
cd client
npm install
```

Start React development server:
```bash
npm run dev
```

React will run on: http://localhost:3000

---

## 📚 GraphQL API

### Access GraphQL Playground

Visit http://localhost:8000/graphql/ to access the GraphiQL interface.

### Example Queries

**Get all products:**
```graphql
query {
  allProducts(limit: 10, offset: 0) {
    id
    productName
    productDescription
    prdPrice
    prdDiscountPrice
    productVendor {
      displayName
      user {
        username
      }
    }
  }
}
```

**Get product by slug:**
```graphql
query {
  productBySlug(slug: "product-slug-here") {
    id
    productName
    prdPrice
    productImage
    productVendor {
      displayName
    }
  }
}
```

**Get all categories:**
```graphql
query {
  allSuperCategories {
    id
    name
    slug
  }
  allMainCategories {
    id
    name
    slug
    superCategory {
      name
    }
  }
}
```

### Example Mutations

**Subscribe to newsletter:**
```graphql
mutation {
  subscribeNewsletter(email: "user@example.com") {
    newsletter {
      email
      subscribed
    }
  }
}
```

**Create contact message:**
```graphql
mutation {
  createContactMessage(
    name: "John Doe"
    email: "john@example.com"
    phone: "+1234567890"
    subject: "Inquiry"
    message: "Hello, I have a question..."
  ) {
    contact {
      id
      name
      email
    }
  }
}
```

---

## 🎨 Frontend Features

### Customer Frontend (Port 3000)
- Product browsing with search
- Shopping cart
- Checkout process
- Order tracking
- Product reviews

### Vendor Dashboard
- Product management (CRUD)
- Order fulfillment
- Sales analytics
- Earnings tracking

### Admin Panel
- User management
- Product moderation
- Order management
- Platform analytics
- Settings configuration

---

## 🔧 Development Commands

### Backend (Django)
```bash
# Run development server
python manage.py runserver

# Create migrations
python manage.py makemigrations

# Apply migrations
python manage.py migrate

# Create superuser
python manage.py createsuperuser

# Collect static files
python manage.py collectstatic

# Run tests
python manage.py test
```

### Frontend (React)
```bash
cd client

# Development server
npm run dev

# Production build
npm run build

# Preview production build
npm run preview

# Lint code
npm run lint
```

---

## 📦 Latest Package Versions

### Python/Django Packages
- Django==5.1.4
- graphene-django==3.2.2
- django-cors-headers==4.6.0
- djangorestframework==3.15.2
- Pillow==11.0.0
- stripe==11.2.0
- razorpay==1.4.2
- celery==5.4.0
- pytest-django==4.9.0

### React/TypeScript Packages
- react==18.3.1
- vite==6.0.3
- @apollo/client==3.11.11
- typescript==5.7.2
- tailwindcss==3.4.17

---

## 🌐 Environment Configuration

Create a `.env` file in the project root (optional, for production):

```env
# Django
SECRET_KEY=your-secret-key-here
DEBUG=False
ALLOWED_HOSTS=your-domain.com

# Database (optional - uses SQLite by default)
DATABASE_URL=postgresql://user:password@localhost:5432/dbname

# Email
SENDGRID_API_KEY=your-sendgrid-key

# Payment Gateways
STRIPE_SECRET_KEY=sk_live_your_key
RAZORPAY_KEY_ID=your_key_id
RAZORPAY_KEY_SECRET=your_secret

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:3000,https://yourdomain.com
```

---

## 🚢 Deployment

### Backend (Django)

**Recommended platforms:**
- Railway (easiest)
- Heroku
- AWS EC2
- DigitalOcean
- PythonAnywhere

**Steps:**
1. Set `DEBUG=False` in settings
2. Configure `ALLOWED_HOSTS`
3. Use PostgreSQL or MySQL for production
4. Collect static files: `python manage.py collectstatic`
5. Use Gunicorn or uWSGI as WSGI server
6. Configure web server (Nginx/Apache)

### Frontend (React)

**Recommended platforms:**
- Vercel
- Netlify
- Cloudflare Pages
- AWS Amplify

**Build command:** `npm run build`
**Output directory:** `dist`

---

## 🎯 Key Improvements from Original

✅ **Latest Django 5.1** - Upgraded from Django 3.2
✅ **GraphQL API** - Added Graphene-Django for modern API
✅ **All Packages Updated** - Every dependency upgraded to 2024/2025 versions
✅ **React Frontend** - Modern SPA with TypeScript
✅ **CORS Configured** - Full support for separate frontend
✅ **Better Security** - Updated security practices
✅ **Modern Tooling** - Vite, TypeScript, latest build tools

---

## 📝 Django Models Overview

The application includes comprehensive models for:
- **Users & Profiles** - Extended user profiles with vendor support
- **Products** - Complete product catalog with images, ratings, sizes
- **Categories** - 4-level hierarchy system
- **Orders** - Full order management with vendor splitting
- **Payments** - Multiple payment gateway integrations
- **Blog** - Posts, comments, and categories
- **Settings** - Site configuration and customization

---

## 🧪 Testing

```bash
# Run Django tests
python manage.py test

# Run with coverage
pip install coverage
coverage run --source='.' manage.py test
coverage report
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

---

## 📄 License

This project is licensed under the MIT License.

---

## 📞 Support

For support and questions:
- Email: support@yourstore.com
- Documentation: See this README
- Issues: GitHub Issues

---

## 🎉 Acknowledgments

- Django team for the excellent framework
- Graphene-Django for GraphQL support
- React team for the frontend library
- All open-source contributors

---

**Built with ❤️ using modern web technologies**

**Django 5.1 | Graphene-Django 3.2 | React 18.3+ | TypeScript 5.7+ | Vite 6.0+ | Apollo Client 3.11+**
