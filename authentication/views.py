from rest_framework import status
from rest_framework.decorators import api_view, permission_classes
from rest_framework.response import Response
from rest_framework.permissions import AllowAny, IsAuthenticated
from rest_framework_simplejwt.tokens import RefreshToken
from django.contrib.auth.models import User
from django.utils import timezone
from .models import EmailOTP, MobileOTP, UserProfile
from .serializers import (
    RequestEmailOTPSerializer,
    VerifyEmailOTPSerializer,
    RequestMobileOTPSerializer,
    VerifyMobileOTPSerializer,
    LinkMobileSerializer,
    UserSerializer,
    AuthResponseSerializer,
    OTPResponseSerializer
)
from .services import OTPService
import logging

logger = logging.getLogger(__name__)


def get_tokens_for_user(user):
    """Generate JWT tokens for user"""
    refresh = RefreshToken.for_user(user)
    return {
        'refresh': str(refresh),
        'access': str(refresh.access_token),
    }


@api_view(['POST'])
@permission_classes([AllowAny])
def request_email_otp(request):
    """
    Request OTP to be sent to email
    POST /auth/request-email-otp
    Body: { "email": "user@example.com" }
    """
    serializer = RequestEmailOTPSerializer(data=request.data)
    if not serializer.is_valid():
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

    email = serializer.validated_data['email']

    # Invalidate previous OTPs for this email
    EmailOTP.objects.filter(email=email, is_verified=False).update(is_verified=True)

    # Generate new OTP
    otp_code = EmailOTP.generate_otp()
    temp_token = EmailOTP.generate_temp_token()

    # Create OTP record
    email_otp = EmailOTP.objects.create(
        email=email,
        otp=otp_code,
        temp_token=temp_token
    )

    # Send OTP via email
    otp_sent = OTPService.send_email_otp(email, otp_code)

    if otp_sent:
        response_data = {
            'status': 'otp_sent',
            'message': f'OTP has been sent to {email}',
            'temp_token': temp_token
        }
        return Response(response_data, status=status.HTTP_200_OK)
    else:
        return Response(
            {'error': 'Failed to send OTP. Please try again.'},
            status=status.HTTP_500_INTERNAL_SERVER_ERROR
        )


@api_view(['POST'])
@permission_classes([AllowAny])
def verify_email_otp(request):
    """
    Verify email OTP and return JWT tokens
    POST /auth/verify-email-otp
    Body: { "email": "user@example.com", "otp": "123456" }
    """
    serializer = VerifyEmailOTPSerializer(data=request.data)
    if not serializer.is_valid():
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

    email = serializer.validated_data['email']
    otp_code = serializer.validated_data['otp']

    try:
        # Get the most recent OTP for this email
        email_otp = EmailOTP.objects.filter(email=email, is_verified=False).latest('created_at')

        # Check if OTP is valid
        if not email_otp.is_valid():
            return Response(
                {'error': 'OTP has expired or maximum attempts exceeded. Please request a new OTP.'},
                status=status.HTTP_400_BAD_REQUEST
            )

        # Increment attempts
        email_otp.attempts += 1
        email_otp.save()

        # Verify OTP
        if email_otp.otp != otp_code:
            return Response(
                {'error': f'Invalid OTP. {3 - email_otp.attempts} attempts remaining.'},
                status=status.HTTP_400_BAD_REQUEST
            )

        # Mark OTP as verified
        email_otp.is_verified = True
        email_otp.save()

        # Get or create user
        user, created = User.objects.get_or_create(
            email=email,
            defaults={
                'username': email.split('@')[0] + str(User.objects.count()),
                'is_active': True
            }
        )

        # Get or create user profile
        profile, _ = UserProfile.objects.get_or_create(
            user=user,
            defaults={'email_verified': True}
        )
        if not profile.email_verified:
            profile.email_verified = True
            profile.save()

        # Generate JWT tokens
        tokens = get_tokens_for_user(user)

        response_data = {
            'access': tokens['access'],
            'refresh': tokens['refresh'],
            'user': UserSerializer(user).data
        }

        logger.info(f"User {user.email} logged in successfully via email OTP")
        return Response(response_data, status=status.HTTP_200_OK)

    except EmailOTP.DoesNotExist:
        return Response(
            {'error': 'No OTP found for this email. Please request a new OTP.'},
            status=status.HTTP_404_NOT_FOUND
        )


@api_view(['POST'])
@permission_classes([AllowAny])
def request_mobile_otp(request):
    """
    Request OTP to be sent to mobile
    POST /auth/request-mobile-otp
    Body: { "mobile": "+911234567890" }
    """
    serializer = RequestMobileOTPSerializer(data=request.data)
    if not serializer.is_valid():
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

    mobile = serializer.validated_data['mobile']

    # Invalidate previous OTPs for this mobile
    MobileOTP.objects.filter(mobile=mobile, is_verified=False).update(is_verified=True)

    # Generate new OTP
    otp_code = MobileOTP.generate_otp()
    temp_token = MobileOTP.generate_temp_token()

    # Create OTP record
    mobile_otp = MobileOTP.objects.create(
        mobile=mobile,
        otp=otp_code,
        temp_token=temp_token
    )

    # Send OTP via SMS
    otp_sent = OTPService.send_mobile_otp(mobile, otp_code)

    if otp_sent:
        response_data = {
            'status': 'otp_sent',
            'message': f'OTP has been sent to {mobile}',
            'temp_token': temp_token
        }
        return Response(response_data, status=status.HTTP_200_OK)
    else:
        return Response(
            {'error': 'Failed to send OTP. Please try again.'},
            status=status.HTTP_500_INTERNAL_SERVER_ERROR
        )


@api_view(['POST'])
@permission_classes([AllowAny])
def verify_mobile_otp(request):
    """
    Verify mobile OTP and return JWT tokens
    POST /auth/verify-mobile-otp
    Body: { "mobile": "+911234567890", "otp": "123456" }
    """
    serializer = VerifyMobileOTPSerializer(data=request.data)
    if not serializer.is_valid():
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

    mobile = serializer.validated_data['mobile']
    otp_code = serializer.validated_data['otp']

    try:
        # Get the most recent OTP for this mobile
        mobile_otp = MobileOTP.objects.filter(mobile=mobile, is_verified=False).latest('created_at')

        # Check if OTP is valid
        if not mobile_otp.is_valid():
            return Response(
                {'error': 'OTP has expired or maximum attempts exceeded. Please request a new OTP.'},
                status=status.HTTP_400_BAD_REQUEST
            )

        # Increment attempts
        mobile_otp.attempts += 1
        mobile_otp.save()

        # Verify OTP
        if mobile_otp.otp != otp_code:
            return Response(
                {'error': f'Invalid OTP. {3 - mobile_otp.attempts} attempts remaining.'},
                status=status.HTTP_400_BAD_REQUEST
            )

        # Mark OTP as verified
        mobile_otp.is_verified = True
        mobile_otp.save()

        # Check if user already exists with this mobile
        try:
            profile = UserProfile.objects.get(mobile=mobile)
            user = profile.user
            if not profile.mobile_verified:
                profile.mobile_verified = True
                profile.save()
        except UserProfile.DoesNotExist:
            # Create new user with mobile
            username = f"user_{mobile[-10:]}"
            user = User.objects.create(
                username=username,
                is_active=True
            )
            profile = UserProfile.objects.create(
                user=user,
                mobile=mobile,
                mobile_verified=True
            )

        # Generate JWT tokens
        tokens = get_tokens_for_user(user)

        response_data = {
            'access': tokens['access'],
            'refresh': tokens['refresh'],
            'user': UserSerializer(user).data
        }

        logger.info(f"User {user.username} logged in successfully via mobile OTP")
        return Response(response_data, status=status.HTTP_200_OK)

    except MobileOTP.DoesNotExist:
        return Response(
            {'error': 'No OTP found for this mobile. Please request a new OTP.'},
            status=status.HTTP_404_NOT_FOUND
        )


@api_view(['POST'])
@permission_classes([IsAuthenticated])
def link_mobile(request):
    """
    Link mobile number to existing email account
    POST /auth/link-mobile
    Headers: Authorization: Bearer <token>
    Body: { "mobile": "+911234567890", "otp": "123456" }
    """
    serializer = LinkMobileSerializer(data=request.data)
    if not serializer.is_valid():
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

    mobile = serializer.validated_data['mobile']
    otp_code = serializer.validated_data['otp']

    try:
        # Get the most recent OTP for this mobile
        mobile_otp = MobileOTP.objects.filter(mobile=mobile, is_verified=False).latest('created_at')

        # Check if OTP is valid
        if not mobile_otp.is_valid():
            return Response(
                {'error': 'OTP has expired or maximum attempts exceeded. Please request a new OTP.'},
                status=status.HTTP_400_BAD_REQUEST
            )

        # Increment attempts
        mobile_otp.attempts += 1
        mobile_otp.save()

        # Verify OTP
        if mobile_otp.otp != otp_code:
            return Response(
                {'error': f'Invalid OTP. {3 - mobile_otp.attempts} attempts remaining.'},
                status=status.HTTP_400_BAD_REQUEST
            )

        # Mark OTP as verified
        mobile_otp.is_verified = True
        mobile_otp.save()

        # Check if mobile is already linked to another account
        if UserProfile.objects.filter(mobile=mobile).exclude(user=request.user).exists():
            return Response(
                {'error': 'This mobile number is already linked to another account.'},
                status=status.HTTP_400_BAD_REQUEST
            )

        # Link mobile to current user
        profile, _ = UserProfile.objects.get_or_create(user=request.user)
        profile.mobile = mobile
        profile.mobile_verified = True
        profile.save()

        logger.info(f"Mobile {mobile} linked to user {request.user.email}")
        return Response({
            'status': 'success',
            'message': 'Mobile number linked successfully',
            'user': UserSerializer(request.user).data
        }, status=status.HTTP_200_OK)

    except MobileOTP.DoesNotExist:
        return Response(
            {'error': 'No OTP found for this mobile. Please request a new OTP.'},
            status=status.HTTP_404_NOT_FOUND
        )


@api_view(['POST'])
@permission_classes([IsAuthenticated])
def logout_view(request):
    """
    Logout user by blacklisting refresh token
    POST /auth/logout
    Headers: Authorization: Bearer <token>
    Body: { "refresh": "<refresh_token>" }
    """
    try:
        refresh_token = request.data.get('refresh')
        if refresh_token:
            token = RefreshToken(refresh_token)
            token.blacklist()
        return Response({'status': 'success', 'message': 'Logged out successfully'}, status=status.HTTP_200_OK)
    except Exception as e:
        return Response({'error': str(e)}, status=status.HTTP_400_BAD_REQUEST)


@api_view(['GET'])
@permission_classes([IsAuthenticated])
def user_info(request):
    """
    Get current user information
    GET /auth/me
    Headers: Authorization: Bearer <token>
    """
    return Response(UserSerializer(request.user).data, status=status.HTTP_200_OK)
