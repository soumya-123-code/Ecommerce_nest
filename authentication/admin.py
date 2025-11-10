from django.contrib import admin
from .models import UserProfile, EmailOTP, MobileOTP


@admin.register(UserProfile)
class UserProfileAdmin(admin.ModelAdmin):
    list_display = ['user', 'mobile', 'email_verified', 'mobile_verified', 'created_at']
    list_filter = ['email_verified', 'mobile_verified']
    search_fields = ['user__email', 'user__username', 'mobile']
    readonly_fields = ['created_at', 'updated_at']


@admin.register(EmailOTP)
class EmailOTPAdmin(admin.ModelAdmin):
    list_display = ['email', 'otp', 'is_verified', 'created_at', 'expires_at', 'attempts']
    list_filter = ['is_verified', 'created_at']
    search_fields = ['email']
    readonly_fields = ['temp_token', 'created_at', 'expires_at']


@admin.register(MobileOTP)
class MobileOTPAdmin(admin.ModelAdmin):
    list_display = ['mobile', 'otp', 'is_verified', 'created_at', 'expires_at', 'attempts', 'user']
    list_filter = ['is_verified', 'created_at']
    search_fields = ['mobile', 'user__email']
    readonly_fields = ['temp_token', 'created_at', 'expires_at']
