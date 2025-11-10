from django.urls import path
from rest_framework_simplejwt.views import TokenRefreshView
from . import views

app_name = 'authentication'

urlpatterns = [
    # Email OTP endpoints
    path('request-email-otp/', views.request_email_otp, name='request_email_otp'),
    path('verify-email-otp/', views.verify_email_otp, name='verify_email_otp'),

    # Mobile OTP endpoints
    path('request-mobile-otp/', views.request_mobile_otp, name='request_mobile_otp'),
    path('verify-mobile-otp/', views.verify_mobile_otp, name='verify_mobile_otp'),

    # Link mobile to account
    path('link-mobile/', views.link_mobile, name='link_mobile'),

    # Token management
    path('refresh/', TokenRefreshView.as_view(), name='token_refresh'),
    path('logout/', views.logout_view, name='logout'),

    # User info
    path('me/', views.user_info, name='user_info'),
]
