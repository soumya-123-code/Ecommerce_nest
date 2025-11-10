from rest_framework import serializers
from django.contrib.auth.models import User
from .models import EmailOTP, MobileOTP, UserProfile


class RequestEmailOTPSerializer(serializers.Serializer):
    email = serializers.EmailField(required=True)


class VerifyEmailOTPSerializer(serializers.Serializer):
    email = serializers.EmailField(required=True)
    otp = serializers.CharField(max_length=6, required=True)


class RequestMobileOTPSerializer(serializers.Serializer):
    mobile = serializers.CharField(max_length=15, required=True)


class VerifyMobileOTPSerializer(serializers.Serializer):
    mobile = serializers.CharField(max_length=15, required=True)
    otp = serializers.CharField(max_length=6, required=True)


class LinkMobileSerializer(serializers.Serializer):
    mobile = serializers.CharField(max_length=15, required=True)
    otp = serializers.CharField(max_length=6, required=True)


class UserSerializer(serializers.ModelSerializer):
    mobile = serializers.SerializerMethodField()
    email_verified = serializers.SerializerMethodField()
    mobile_verified = serializers.SerializerMethodField()

    class Meta:
        model = User
        fields = ['id', 'username', 'email', 'first_name', 'last_name', 'mobile', 'email_verified', 'mobile_verified']

    def get_mobile(self, obj):
        try:
            return obj.otp_profile.mobile
        except:
            return None

    def get_email_verified(self, obj):
        try:
            return obj.otp_profile.email_verified
        except:
            return False

    def get_mobile_verified(self, obj):
        try:
            return obj.otp_profile.mobile_verified
        except:
            return False


class AuthResponseSerializer(serializers.Serializer):
    """Response after successful authentication"""
    access = serializers.CharField()
    refresh = serializers.CharField()
    user = UserSerializer()


class OTPResponseSerializer(serializers.Serializer):
    """Response after OTP request"""
    status = serializers.CharField()
    message = serializers.CharField()
    temp_token = serializers.CharField(required=False)
