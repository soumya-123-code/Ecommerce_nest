"""
Django settings for project project.

Generated by 'django-admin startproject' using Django 3.2.12.

For more information on this file, see
https://docs.djangoproject.com/en/3.2/topics/settings/

For the full list of settings and their values, see
https://docs.djangoproject.com/en/3.2/ref/settings/
"""

from pathlib import Path
import os
from django.utils.translation import ugettext_lazy as _
# Build paths inside the project like this: BASE_DIR / 'subdir'.
BASE_DIR = Path(__file__).resolve().parent.parent


# Quick-start development settings - unsuitable for production
# See https://docs.djangoproject.com/en/3.2/howto/deployment/checklist/

# SECURITY WARNING: keep the secret key used in production secret!
SECRET_KEY = 'django-insecure-%eckd70i##mznm!a55p^yb&3unlw3!r)d3_g*=1zrx$_*_hy^5'

# SECURITY WARNING: don't run with debug turned on in production!
DEBUG = True

ALLOWED_HOSTS = ["*", "572d-156-209-45-224.ngrok.io",
                 "www.572d-156-209-45-224.ngrok.io", "127.0.0.1", "127.0.0.1:8000"]


# Application definition

INSTALLED_APPS = [
    # General use templates & template tags (should appear first)
    'adminlte3',
    # Optional: Django admin theme (must be before django.contrib.admin)
    'adminlte3_theme',
    'django.contrib.admin',
    'django.contrib.auth',
    'django.contrib.contenttypes',
    'django.contrib.sessions',
    'django.contrib.messages',
    'django.contrib.staticfiles',
    'whitenoise.runserver_nostatic',
    'captcha',
    'currencies',
    'home',
    'categories',
    'products',
    'accounts',
    'orders',
    'suppliers',
    'supplier_panel',
    'ckeditor',
    'crispy_forms',
    # 'FAQ',
    'newsletters',
    'blog',
    'reports',
    'settings',
    'contact',
    'pages',
    'payments',
]

MIDDLEWARE = [
    'django.middleware.security.SecurityMiddleware',
    'django.contrib.sessions.middleware.SessionMiddleware',
    'django.middleware.common.CommonMiddleware',
    'django.middleware.csrf.CsrfViewMiddleware',
    'django.contrib.auth.middleware.AuthenticationMiddleware',
    'django.contrib.messages.middleware.MessageMiddleware',
    'django.middleware.clickjacking.XFrameOptionsMiddleware',
    'whitenoise.middleware.WhiteNoiseMiddleware',
]

ROOT_URLCONF = 'project.urls'

TEMPLATES = [
    {
        'BACKEND': 'django.template.backends.django.DjangoTemplates',
        'DIRS': [os.path.join(BASE_DIR, 'templates')],
        'APP_DIRS': True,
        'OPTIONS': {
            'context_processors': [
                'django.template.context_processors.debug',
                'django.template.context_processors.request',
                'django.contrib.auth.context_processors.auth',
                'django.contrib.messages.context_processors.messages',
                'currencies.context_processors.currencies',
                'categories.context_processors.category_obj',
                'products.context_processors.new_products_obj',
                'orders.context_processors.orders_cart_obj',
                'home.context_processors.DealTime_obj',
                'home.context_processors.vendor_details_ad_image',
                'home.context_processors.shop_ad_sidebar',
                'home.context_processors.hot_deal_ad',
                'home.context_processors.head_text_ad',
                'settings.context_processors.socail_links_settings',
                'settings.context_processors.contact_info_settings',
                'settings.context_processors.support_number_settings',
                'settings.context_processors.site_settings',
                'pages.context_processors.pages_list_obj',

            ],
        },
    },
]

WSGI_APPLICATION = 'project.wsgi.application'


# Database
# https://docs.djangoproject.com/en/3.2/ref/settings/#databases

DATABASES = {
    'default': {
        'ENGINE': 'django.db.backends.sqlite3',
        'NAME': BASE_DIR / 'db.sqlite3',
    }
}

# DATABASES = {
#     'default': {
#         'ENGINE': 'django.db.backends.mysql',
#         'NAME': 'nestali',
#         'USER': 'nestali',
#         'PASSWORD': 'nestali123!@#',
#         'HOST': 'localhost',
#         'PORT':'',
#         'OPTIONS': {
#         'sql_mode': 'traditional',
#     }
#     }
# }

# Password validation
# https://docs.djangoproject.com/en/3.2/ref/settings/#auth-password-validators

AUTH_PASSWORD_VALIDATORS = [
    {
        'NAME': 'django.contrib.auth.password_validation.UserAttributeSimilarityValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.MinimumLengthValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.CommonPasswordValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.NumericPasswordValidator',
    },
]

###DEFAULT_CURRENCY##
DEFAULT_CURRENCY = 'USD'

# Internationalization
# https://docs.djangoproject.com/en/3.2/topics/i18n/

LANGUAGE_CODE = 'en-us'

TIME_ZONE = 'UTC'

USE_I18N = True

USE_L10N = True

USE_TZ = True

# Static files (CSS, JavaScript, Images)
# https://docs.djangoproject.com/en/3.2/howto/static-files/
STATIC_URL = '/static/'
STATIC_ROOT = os.path.join(BASE_DIR, 'static', 'site_static')
STATICFILES_DIRS = [
    os.path.join(BASE_DIR, "static"),
]

MEDIA_ROOT = os.path.join(BASE_DIR, 'media')
MEDIA_URL = '/media/'

# STATICFILES_STORAGE = 'whitenoise.storage.CompressedManifestStaticFilesStorage'
STATICFILES_STORAGE = 'whitenoise.storage.CompressedStaticFilesStorage'


# Default primary key field type
# https://docs.djangoproject.com/en/3.2/ref/settings/#default-auto-field

DEFAULT_AUTO_FIELD = 'django.db.models.BigAutoField'

CKEDITOR_CONFIGS = {
    'default': {
        'toolbar': 'Custom',
        'toolbar_Custom': [
            ['Bold', 'Italic', 'Underline'],
            ['NumberedList', 'BulletedList', '-', 'Outdent', 'Indent', '-',
                'JustifyLeft', 'JustifyCenter', 'JustifyRight', 'JustifyBlock'],
            ['Link', 'Unlink'],
            ['RemoveFormat', 'Source'],
            ['Form', 'Checkbox', 'Radio', 'TextField', 'Image',
                'Textarea', 'Select', 'Button', 'HiddenField']

        ]
    }
}

CRISPY_TEMPLATE_PACK = 'uni_form'

## STRIPE SETTINGS ##
STRIPE_PUBLIC_KEY = "pk_test_51JlJ7SDD5hTsFJUFhRIoCXwo6ubrbkKqzOnyPhM57tf8XeyHYlW5X275q7Ms8zHedBHTsACXHDwuCModpr0n5U05005vBVLepI"
STRIPE_SECRET_KEY = "sk_test_51JlJ7SDD5hTsFJUFx61JYCnJc8bqc3bW9MVaFdgg0g0tMrsHbsaQitwg5HHZ1e7VwyyXciTedv38GISa88VJDmu900Br72Jj3q"
STRIPE_WEBHOOK_SECRET = "whsec_1xR7X9MTv6Qjfbt7FQCZh5EMGkcFOuge"
# domain EX: example.com
YOUR_DOMAIN = "c8c5-156-209-74-170.ngrok-free.app"

# very important
# Set your Endpoint_URL in your stripe account WEBHOOK like this : https://YOUR_DOMAIN/orders/webhook/
# DEBUG_EMAIL_STRIPE
DEBUG_EMAIL = "info@mohamedselem.com"
## End Stripe Settings ##

# ClientInfo For Aramex
ARAMEX_USERNAME = ""
# ARAMEX_USERNAME = "Nabily@aramex.com"
ARAMEX_PASSWORD = "Aramex123$"
ARAMEX_VERSION = "v1.0"
ARAMEX_ACCOUNTNUMBER = "20016"
ARAMEX_ACCOUNTPIN = "543543"

ARAMEX_ACCOUNTENTITY = "AMM"
ARAMEX_ACCOUNTCOUNTRYCODE = "JO"
ARAMEX_SOURCE = "24"

ARAMEX_PRODUCTGROUP = "EXP"
ARAMEX_PRODUCTTYPE = "PPX"


# #Smtp Email for recovery password
EMAIL_BACKEND = 'sendgrid_backend.SendgridBackend'
SENDGRID_API_KEY = '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'
SENDGRID_SANDBOX_MODE_IN_DEBUG = False
EMAIL_SENDGRID = "selemhamed2016@gmail.com"


# razorpay account ###
RAZORPAY_KEY_ID = 'rzp_test_Fo7S7Rh7owOFJQ'
RAZORPAY_KEY_SECRET = 'pEUUmsqka5XJGIX5RdB7mlwC'


## paymob account ##
API_KEY = "ZXlKaGJHY2lPaUpJVXpVeE1pSXNJblI1Y0NJNklrcFhWQ0o5LmV5SmpiR0Z6Y3lJNklrMWxjbU5vWVc1MElpd2libUZ0WlNJNkltbHVhWFJwWVd3aUxDSndjbTltYVd4bFgzQnJJam95TkRFNU5qRjkuWGhwWnV0eU83U1BYUnR0Wk5sUkM5d2RpOEFReXhycmdqYmZQOUZ0M3BEcEp6MzVRTjNkYmFDN1VCaGFrQjdVZS1Xcms0aTZmRnNqN3NqdGpHRnIxUnc="
PAYMENT_INTEGRATIONS_ID = 2405584


## paypal account ##
PAYPAL_CLIENT_ID = "AdvH9io4569_jQt-c4wBxJD85SM4ujhSS1uvkrSgzIdhiO69SlTAZmcVIBlJXUzYQ3apJJ28WBFMCBmq"
PAYPAL_SECRET = "ENwDJF-kn34uO8yg_V-MxQFr4gbKCYgrd_iCY6fmzR_KKnooB2JlDTIBwrZ4bYsoh7Nyh0-GTEgbbHAq"
PAYPAL_ACCESS_TOKEN = "A21AAKwFzvBhx4uG4CnQyn34iea3bVSLAzda4-4WMwKLhQw1hq9JlMqhX1VqydwjeFZK_aXZrMUFdSM8OJtxruL0eyLcVZZnw"
PAYPAL_CURRENCY = "USD"
# how you can get your PAYPAL_ACCESS_TOKEN
# https://developer.paypal.com/api/rest/authentication/


## Fatoorah account ##
FATOORAH_API_KEY = 'rLtt6JWvbUHDDhsZnfpAhpYk4dxYDQkbcPTyGaKp2TYqQgG7FGZ5Th_WD53Oq8Ebz6A53njUoo1w3pjU1D4vs_ZMqFiz_j0urb_BH9Oq9VZoKFoJEDAbRZepGcQanImyYrry7Kt6MnMdgfG5jn4HngWoRdKduNNyP4kzcp3mRv7x00ahkm9LAK7ZRieg7k1PDAnBIOG3EyVSJ5kK4WLMvYr7sCwHbHcu4A5WwelxYK0GMJy37bNAarSJDFQsJ2ZvJjvMDmfWwDVFEVe_5tOomfVNt6bOg9mexbGjMrnHBnKnZR1vQbBtQieDlQepzTZMuQrSuKn-t5XZM7V6fCW7oP-uXGX-sMOajeX65JOf6XVpk29DP6ro8WTAflCDANC193yof8-f5_EYY-3hXhJj7RBXmizDpneEQDSaSz5sFk0sV5qPcARJ9zGG73vuGFyenjPPmtDtXtpx35A-BVcOSBYVIWe9kndG3nclfefjKEuZ3m4jL9Gg1h2JBvmXSMYiZtp9MR5I6pvbvylU_PP5xJFSjVTIz7IQSjcVGO41npnwIxRXNRxFOdIUHn0tjQ-7LwvEcTXyPsHXcMD8WtgBh-wxR8aKX7WPSsT1O8d8reb2aR7K3rkV3K82K_0OgawImEpwSvp9MNKynEAJQS6ZHe_J_l77652xwPNxMRTMASk1ZsJL'

'''
Live API
For the live API environment, you must use the https://api.myfatoorah.com/v2/ URL and the Live API key.

Live API URL for Saudi Arabia: https://api-sa.myfatoorah.com/v2/
'''
FATOORAHBASURL = "https://apitest.myfatoorah.com/v2"

FATOORAHBACKURL = f'https://{YOUR_DOMAIN}/en/api/callbacks-myfatoorah/'
FATOORAHERRORURL = f'https://{YOUR_DOMAIN}/order/cancel/'
FATOORAH_CURREENCY ='usd'