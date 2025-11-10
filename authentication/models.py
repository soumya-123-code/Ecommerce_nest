from django.db import models
from django.contrib.auth.models import User
from django.utils import timezone
from datetime import timedelta
import random
import string


class UserProfile(models.Model):
    """Extended user profile for OTP authentication"""
    user = models.OneToOneField(User, on_delete=models.CASCADE, related_name='otp_profile')
    mobile = models.CharField(max_length=15, blank=True, null=True, unique=True)
    mobile_verified = models.BooleanField(default=False)
    email_verified = models.BooleanField(default=False)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self):
        return f"{self.user.email} - Profile"


class EmailOTP(models.Model):
    """Email OTP verification model"""
    email = models.EmailField()
    otp = models.CharField(max_length=6)
    temp_token = models.CharField(max_length=100, unique=True)
    is_verified = models.BooleanField(default=False)
    created_at = models.DateTimeField(auto_now_add=True)
    expires_at = models.DateTimeField()
    attempts = models.IntegerField(default=0)

    class Meta:
        ordering = ['-created_at']
        verbose_name = 'Email OTP'
        verbose_name_plural = 'Email OTPs'

    def __str__(self):
        return f"{self.email} - {self.otp}"

    def is_expired(self):
        return timezone.now() > self.expires_at

    def is_valid(self):
        return not self.is_verified and not self.is_expired() and self.attempts < 3

    @staticmethod
    def generate_otp():
        return ''.join(random.choices(string.digits, k=6))

    @staticmethod
    def generate_temp_token():
        return ''.join(random.choices(string.ascii_letters + string.digits, k=64))

    def save(self, *args, **kwargs):
        if not self.expires_at:
            # OTP expires in 10 minutes
            self.expires_at = timezone.now() + timedelta(minutes=10)
        if not self.temp_token:
            self.temp_token = self.generate_temp_token()
        super().save(*args, **kwargs)


class MobileOTP(models.Model):
    """Mobile OTP verification model"""
    mobile = models.CharField(max_length=15)
    otp = models.CharField(max_length=6)
    temp_token = models.CharField(max_length=100, unique=True)
    is_verified = models.BooleanField(default=False)
    created_at = models.DateTimeField(auto_now_add=True)
    expires_at = models.DateTimeField()
    attempts = models.IntegerField(default=0)
    user = models.ForeignKey(User, on_delete=models.CASCADE, null=True, blank=True, related_name='mobile_otps')

    class Meta:
        ordering = ['-created_at']
        verbose_name = 'Mobile OTP'
        verbose_name_plural = 'Mobile OTPs'

    def __str__(self):
        return f"{self.mobile} - {self.otp}"

    def is_expired(self):
        return timezone.now() > self.expires_at

    def is_valid(self):
        return not self.is_verified and not self.is_expired() and self.attempts < 3

    @staticmethod
    def generate_otp():
        return ''.join(random.choices(string.digits, k=6))

    @staticmethod
    def generate_temp_token():
        return ''.join(random.choices(string.ascii_letters + string.digits, k=64))

    def save(self, *args, **kwargs):
        if not self.expires_at:
            # OTP expires in 10 minutes
            self.expires_at = timezone.now() + timedelta(minutes=10)
        if not self.temp_token:
            self.temp_token = self.generate_temp_token()
        super().save(*args, **kwargs)
