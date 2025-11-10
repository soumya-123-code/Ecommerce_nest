# Django + GraphQL + React eCommerce Platform - Setup Guide

## 🚀 Complete Full-Stack Setup with OTP Authentication

This application has been transformed from Django server-side rendering to a modern **Django GraphQL API + React SPA** architecture.

---

## 📋 Architecture

- **Backend**: Django 5.1.4 + GraphQL (Graphene-Django) + REST Framework
- **Frontend**: React 18.3 + TypeScript + Apollo Client
- **Authentication**: OTP-based (Email + Mobile) with JWT tokens
- **Database**: SQLite (dev) / PostgreSQL/MySQL (production)
- **Static Assets**: Served by Django, referenced by React

---

## 🔧 Backend Setup (Django)

### 1. Install Python Dependencies

```bash
# Create virtual environment (recommended)
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install all dependencies
pip install -r requirements.txt
```

### 2. Database Setup

```bash
# Create migrations for authentication app
python manage.py makemigrations authentication

# Run all migrations
python manage.py migrate

# Create superuser for admin access
python manage.py createsuperuser
```

### 3. Configure Email & SMS (Optional)

The app uses SendGrid for email OTP and Twilio for mobile OTP.

**Email Configuration** (Already configured in settings.py):
- SendGrid is already set up in `project/settings.py`
- Update `SENDGRID_API_KEY` with your key if needed

**Mobile OTP Configuration** (Optional):
Edit `project/settings.py` and uncomment:
```python
TWILIO_ACCOUNT_SID = 'your_account_sid'
TWILIO_AUTH_TOKEN = 'your_auth_token'
TWILIO_PHONE_NUMBER = '+1234567890'
```

**Note**: If Twilio is not configured, mobile OTPs will be logged to console (dev mode).

### 4. Start Django Server

```bash
python manage.py runserver
```

Django will run on: **http://localhost:8000**

**Available Endpoints:**
- GraphQL API: `http://localhost:8000/graphql/` (GraphiQL interface)
- Admin Panel: `http://localhost:8000/admin/`
- REST Auth: `http://localhost:8000/auth/`

---

## 🎨 Frontend Setup (React)

### 1. Install Node Dependencies

```bash
cd client
npm install
```

### 2. Start React Development Server

```bash
npm run dev
```

React will run on: **http://localhost:3000**

---

## 🔐 Authentication API

### REST API Endpoints

#### Email OTP Flow

**1. Request Email OTP:**
```bash
POST http://localhost:8000/auth/request-email-otp/
Content-Type: application/json

{
  "email": "user@example.com"
}

Response:
{
  "status": "otp_sent",
  "message": "OTP has been sent to user@example.com",
  "temp_token": "abc123..."
}
```

**2. Verify Email OTP:**
```bash
POST http://localhost:8000/auth/verify-email-otp/
Content-Type: application/json

{
  "email": "user@example.com",
  "otp": "123456"
}

Response:
{
  "access": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "refresh": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "user": {
    "id": 1,
    "username": "user",
    "email": "user@example.com",
    "mobile": null,
    "email_verified": true,
    "mobile_verified": false
  }
}
```

#### Mobile OTP Flow

**1. Request Mobile OTP:**
```bash
POST http://localhost:8000/auth/request-mobile-otp/
Content-Type: application/json

{
  "mobile": "+911234567890"
}
```

**2. Verify Mobile OTP:**
```bash
POST http://localhost:8000/auth/verify-mobile-otp/
Content-Type: application/json

{
  "mobile": "+911234567890",
  "otp": "123456"
}
```

#### Link Mobile to Account

```bash
POST http://localhost:8000/auth/link-mobile/
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "mobile": "+911234567890",
  "otp": "123456"
}
```

#### Other Endpoints

```bash
# Get current user info
GET http://localhost:8000/auth/me/
Authorization: Bearer <access_token>

# Refresh access token
POST http://localhost:8000/auth/refresh/
Content-Type: application/json
{
  "refresh": "<refresh_token>"
}

# Logout (blacklist token)
POST http://localhost:8000/auth/logout/
Authorization: Bearer <access_token>
Content-Type: application/json
{
  "refresh": "<refresh_token>"
}
```

---

### GraphQL Mutations

```graphql
# Request Email OTP
mutation {
  requestEmailOtp(email: "user@example.com") {
    status
    message
    tempToken
  }
}

# Verify Email OTP
mutation {
  verifyEmailOtp(email: "user@example.com", otp: "123456") {
    access
    refresh
    user {
      id
      username
      email
      emailVerified
      mobileVerified
    }
  }
}

# Request Mobile OTP
mutation {
  requestMobileOtp(mobile: "+911234567890") {
    status
    message
    tempToken
  }
}

# Verify Mobile OTP
mutation {
  verifyMobileOtp(mobile: "+911234567890", otp: "123456") {
    access
    refresh
    user {
      id
      username
      mobile
      emailVerified
      mobileVerified
    }
  }
}

# Link Mobile (requires authentication)
mutation {
  linkMobile(mobile: "+911234567890", otp: "123456") {
    id
    username
    email
    mobile
    emailVerified
    mobileVerified
  }
}

# Get Current User
query {
  me {
    id
    username
    email
    mobile
    emailVerified
    mobileVerified
  }
}
```

---

## 📦 Project Structure

```
Ecommerce_nest/
├── authentication/          # OTP Authentication app
│   ├── models.py           # EmailOTP, MobileOTP, UserProfile
│   ├── views.py            # REST API views
│   ├── schema.py           # GraphQL mutations
│   ├── serializers.py      # DRF serializers
│   └── services.py         # Email & SMS services
├── api/                    # GraphQL schema (products, orders, etc.)
├── accounts/               # User accounts
├── products/               # Product models
├── orders/                 # Order management
├── categories/             # Product categories
├── payments/               # Payment processing
├── client/                 # React Frontend
│   ├── src/
│   │   ├── components/     # Reusable components
│   │   ├── pages/          # Page components
│   │   ├── lib/            # Apollo Client setup
│   │   └── App.tsx         # Main app
│   └── index.html
├── static/                 # Django static files (CSS, JS, images)
├── requirements.txt        # Python dependencies
└── manage.py
```

---

## 🎯 Key Features

### Authentication
- ✅ OTP-only authentication (no passwords)
- ✅ Email OTP via SendGrid
- ✅ Mobile OTP via Twilio
- ✅ JWT token-based authentication
- ✅ Token refresh & blacklisting
- ✅ Link mobile to email account
- ✅ Both REST and GraphQL support

### Frontend
- ✅ React SPA with TypeScript
- ✅ Apollo Client for GraphQL
- ✅ JWT token management
- ✅ Home page with carousel & products
- ✅ Product listing & details pages
- ✅ Shopping cart
- ✅ Responsive design

### Backend
- ✅ Django 5.1.4 (Latest LTS)
- ✅ Complete GraphQL API (Graphene-Django)
- ✅ Django REST Framework APIs
- ✅ 60+ GraphQL types covering all models
- ✅ Multi-vendor marketplace support
- ✅ Payment gateways (Stripe, Razorpay, PayPal)

---

## 🧪 Testing Authentication

### Using cURL

```bash
# 1. Request OTP
curl -X POST http://localhost:8000/auth/request-email-otp/ \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com"}'

# 2. Check console/email for OTP, then verify
curl -X POST http://localhost:8000/auth/verify-email-otp/ \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "otp": "123456"}'

# 3. Use the access token
curl -H "Authorization: Bearer <access_token>" \
  http://localhost:8000/auth/me/
```

### Using GraphiQL

Visit: `http://localhost:8000/graphql/`

Try the mutations above in the GraphiQL interface.

---

## 🔒 Security Notes

- OTPs expire after **10 minutes**
- Maximum **3 attempts** per OTP
- JWT access tokens valid for **1 hour**
- Refresh tokens valid for **7 days**
- Tokens are blacklisted on logout
- CORS configured for React frontend

---

## 📝 Next Steps

1. **Setup complete?** Check both servers are running:
   - Django: http://localhost:8000/graphql/
   - React: http://localhost:3000

2. **Test Authentication**: Use the REST or GraphQL endpoints above

3. **Customize**: Update branding, colors, and content in React components

4. **Production**:
   - Set `DEBUG = False` in settings
   - Configure PostgreSQL/MySQL database
   - Set up proper SMTP for emails
   - Configure Twilio for SMS
   - Use environment variables for secrets
   - Set up HTTPS
   - Configure static file serving (WhiteNoise or CDN)

---

## 🐛 Troubleshooting

**"Module not found" errors:**
```bash
pip install -r requirements.txt
```

**Database errors:**
```bash
python manage.py makemigrations
python manage.py migrate
```

**React build errors:**
```bash
cd client
rm -rf node_modules package-lock.json
npm install
```

**CORS errors:**
- Check `CORS_ALLOWED_ORIGINS` in `settings.py`
- Ensure React dev server is running on allowed port

---

## 📧 Support

For issues or questions, check the commit history or create an issue in the repository.

---

**Built with ❤️ using Django + GraphQL + React**
