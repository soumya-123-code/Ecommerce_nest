import graphene
from graphene_django import DjangoObjectType
from django.contrib.auth.models import User
from .models import EmailOTP, MobileOTP, UserProfile
from .services import OTPService
from rest_framework_simplejwt.tokens import RefreshToken
import logging

logger = logging.getLogger(__name__)


class UserType(DjangoObjectType):
    mobile = graphene.String()
    email_verified = graphene.Boolean()
    mobile_verified = graphene.Boolean()

    class Meta:
        model = User
        fields = ('id', 'username', 'email', 'first_name', 'last_name')

    def resolve_mobile(self, info):
        try:
            return self.otp_profile.mobile
        except:
            return None

    def resolve_email_verified(self, info):
        try:
            return self.otp_profile.email_verified
        except:
            return False

    def resolve_mobile_verified(self, info):
        try:
            return self.otp_profile.mobile_verified
        except:
            return False


class OTPResponseType(graphene.ObjectType):
    status = graphene.String()
    message = graphene.String()
    temp_token = graphene.String()


class AuthPayloadType(graphene.ObjectType):
    access = graphene.String()
    refresh = graphene.String()
    user = graphene.Field(UserType)


class RequestEmailOtp(graphene.Mutation):
    class Arguments:
        email = graphene.String(required=True)

    Output = OTPResponseType

    def mutate(self, info, email):
        # Invalidate previous OTPs
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

        # Send OTP
        otp_sent = OTPService.send_email_otp(email, otp_code)

        if otp_sent:
            return OTPResponseType(
                status='otp_sent',
                message=f'OTP has been sent to {email}',
                temp_token=temp_token
            )
        else:
            raise Exception('Failed to send OTP. Please try again.')


class VerifyEmailOtp(graphene.Mutation):
    class Arguments:
        email = graphene.String(required=True)
        otp = graphene.String(required=True)

    Output = AuthPayloadType

    def mutate(self, info, email, otp):
        try:
            # Get the most recent OTP
            email_otp = EmailOTP.objects.filter(email=email, is_verified=False).latest('created_at')

            # Check if valid
            if not email_otp.is_valid():
                raise Exception('OTP has expired or maximum attempts exceeded')

            # Increment attempts
            email_otp.attempts += 1
            email_otp.save()

            # Verify OTP
            if email_otp.otp != otp:
                raise Exception(f'Invalid OTP. {3 - email_otp.attempts} attempts remaining')

            # Mark as verified
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

            # Get or create profile
            profile, _ = UserProfile.objects.get_or_create(
                user=user,
                defaults={'email_verified': True}
            )
            if not profile.email_verified:
                profile.email_verified = True
                profile.save()

            # Generate tokens
            refresh = RefreshToken.for_user(user)

            logger.info(f"User {user.email} logged in via GraphQL email OTP")

            return AuthPayloadType(
                access=str(refresh.access_token),
                refresh=str(refresh),
                user=user
            )

        except EmailOTP.DoesNotExist:
            raise Exception('No OTP found for this email')


class RequestMobileOtp(graphene.Mutation):
    class Arguments:
        mobile = graphene.String(required=True)

    Output = OTPResponseType

    def mutate(self, info, mobile):
        # Invalidate previous OTPs
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

        # Send OTP
        otp_sent = OTPService.send_mobile_otp(mobile, otp_code)

        if otp_sent:
            return OTPResponseType(
                status='otp_sent',
                message=f'OTP has been sent to {mobile}',
                temp_token=temp_token
            )
        else:
            raise Exception('Failed to send OTP. Please try again.')


class VerifyMobileOtp(graphene.Mutation):
    class Arguments:
        mobile = graphene.String(required=True)
        otp = graphene.String(required=True)

    Output = AuthPayloadType

    def mutate(self, info, mobile, otp):
        try:
            # Get the most recent OTP
            mobile_otp = MobileOTP.objects.filter(mobile=mobile, is_verified=False).latest('created_at')

            # Check if valid
            if not mobile_otp.is_valid():
                raise Exception('OTP has expired or maximum attempts exceeded')

            # Increment attempts
            mobile_otp.attempts += 1
            mobile_otp.save()

            # Verify OTP
            if mobile_otp.otp != otp:
                raise Exception(f'Invalid OTP. {3 - mobile_otp.attempts} attempts remaining')

            # Mark as verified
            mobile_otp.is_verified = True
            mobile_otp.save()

            # Check if user exists
            try:
                profile = UserProfile.objects.get(mobile=mobile)
                user = profile.user
                if not profile.mobile_verified:
                    profile.mobile_verified = True
                    profile.save()
            except UserProfile.DoesNotExist:
                # Create new user
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

            # Generate tokens
            refresh = RefreshToken.for_user(user)

            logger.info(f"User {user.username} logged in via GraphQL mobile OTP")

            return AuthPayloadType(
                access=str(refresh.access_token),
                refresh=str(refresh),
                user=user
            )

        except MobileOTP.DoesNotExist:
            raise Exception('No OTP found for this mobile')


class LinkMobile(graphene.Mutation):
    class Arguments:
        mobile = graphene.String(required=True)
        otp = graphene.String(required=True)

    Output = graphene.Field(UserType)

    def mutate(self, info, mobile, otp):
        user = info.context.user
        if not user.is_authenticated:
            raise Exception('Authentication required')

        try:
            # Get the most recent OTP
            mobile_otp = MobileOTP.objects.filter(mobile=mobile, is_verified=False).latest('created_at')

            # Check if valid
            if not mobile_otp.is_valid():
                raise Exception('OTP has expired or maximum attempts exceeded')

            # Increment attempts
            mobile_otp.attempts += 1
            mobile_otp.save()

            # Verify OTP
            if mobile_otp.otp != otp:
                raise Exception(f'Invalid OTP. {3 - mobile_otp.attempts} attempts remaining')

            # Mark as verified
            mobile_otp.is_verified = True
            mobile_otp.save()

            # Check if mobile is already linked
            if UserProfile.objects.filter(mobile=mobile).exclude(user=user).exists():
                raise Exception('This mobile number is already linked to another account')

            # Link mobile
            profile, _ = UserProfile.objects.get_or_create(user=user)
            profile.mobile = mobile
            profile.mobile_verified = True
            profile.save()

            logger.info(f"Mobile {mobile} linked to user {user.email} via GraphQL")

            return user

        except MobileOTP.DoesNotExist:
            raise Exception('No OTP found for this mobile')


class AuthMutation(graphene.ObjectType):
    request_email_otp = RequestEmailOtp.Field()
    verify_email_otp = VerifyEmailOtp.Field()
    request_mobile_otp = RequestMobileOtp.Field()
    verify_mobile_otp = VerifyMobileOtp.Field()
    link_mobile = LinkMobile.Field()


class AuthQuery(graphene.ObjectType):
    me = graphene.Field(UserType)

    def resolve_me(self, info):
        user = info.context.user
        if user.is_authenticated:
            return user
        return None
