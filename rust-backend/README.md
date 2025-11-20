# E-Commerce Backend - Rust Migration

This is a complete migration of the Django e-commerce backend to Rust using Axum and Diesel ORM with MySQL.

## Technology Stack

- **Web Framework**: Axum 0.7
- **ORM**: Diesel 2.1 (MySQL)
- **Connection Pool**: r2d2
- **Authentication**: JWT (jsonwebtoken)
- **Password Hashing**: bcrypt
- **Serialization**: Serde

## Project Structure

```
src/
├── config/          # Configuration loading
├── controllers/     # Axum handlers (Django views)
├── db/              # Database connection pool
├── dto/             # Request/Response types (Django serializers)
├── middlewares/     # Authentication middleware
├── models/          # Diesel models (Django models)
├── routes/          # Route definitions (Django urls.py)
├── schema/          # Diesel schema
├── services/        # Business logic layer
├── utils/           # Utilities and error handling
└── main.rs          # Application entry point
```

## Migration Mapping

| Django | Rust |
|--------|------|
| models.py | src/models/*.rs |
| views.py | src/controllers/*.rs |
| urls.py | src/routes/mod.rs |
| serializers.py | src/dto/*.rs |
| middleware.py | src/middlewares/*.rs |

## Getting Started

### Prerequisites

- Rust 1.70+
- MySQL 8.0+
- Diesel CLI

### Installation

1. Install Diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features mysql
```

2. Copy environment file:
```bash
cp .env.example .env
```

3. Edit `.env` with your database credentials.

4. Run migrations:
```bash
diesel migration run
```

5. Build and run:
```bash
cargo run
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login
- `GET /api/auth/profile` - Get profile
- `PUT /api/auth/profile` - Update profile
- `POST /api/auth/change-password` - Change password

### Products
- `GET /api/products` - List products (with filters)
- `GET /api/products/:slug` - Get product details
- `POST /api/products/:id/rate` - Rate product
- `GET /api/categories` - Get category tree

### Orders
- `POST /api/orders` - Create order
- `GET /api/orders` - List user orders
- `GET /api/orders/:id` - Get order details

### Vendor Panel
- `GET /api/vendor/products` - List vendor products
- `POST /api/vendor/products` - Create product
- `PUT /api/vendor/products/:id` - Update product
- `GET /api/vendor/orders` - List vendor orders
- `PUT /api/vendor/orders/:id` - Update order status
- `GET /api/vendor/wallet` - Get wallet balance

## Features Migrated

- User authentication with JWT
- Multi-vendor marketplace
- 4-level category hierarchy
- Product management with images and sizes
- Order processing with vendor splitting
- Coupon system
- Referral commission (2.5%)
- Blog with analytics
- Newsletter subscriptions

## Environment Variables

```env
DATABASE_URL=mysql://user:password@localhost:3306/ecommerce_db
HOST=127.0.0.1
PORT=8080
JWT_SECRET=your-secret-key
JWT_EXPIRATION=86400
```

## Running Tests

```bash
cargo test
```

## Building for Production

```bash
cargo build --release
```

## License

MIT
