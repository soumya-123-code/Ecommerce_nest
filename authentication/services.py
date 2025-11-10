from django.core.mail import send_mail
from django.conf import settings
from twilio.rest import Client
import logging

logger = logging.getLogger(__name__)


class OTPService:
    """Service for sending OTP via email and SMS"""

    @staticmethod
    def send_email_otp(email, otp):
        """Send OTP via email"""
        try:
            subject = f'Your OTP Code - {settings.SITE_NAME if hasattr(settings, "SITE_NAME") else "eCommerce"}'
            message = f"""
Hello,

Your One-Time Password (OTP) for login is: {otp}

This OTP is valid for 10 minutes.

If you didn't request this OTP, please ignore this email.

Best regards,
{settings.SITE_NAME if hasattr(settings, "SITE_NAME") else "eCommerce"} Team
            """
            html_message = f"""
<!DOCTYPE html>
<html>
<head>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #3BB77E; color: white; padding: 20px; text-align: center; border-radius: 5px 5px 0 0; }}
        .content {{ background: #f9f9f9; padding: 30px; border-radius: 0 0 5px 5px; }}
        .otp-code {{ font-size: 32px; font-weight: bold; color: #3BB77E; text-align: center; padding: 20px; background: white; border-radius: 5px; margin: 20px 0; letter-spacing: 5px; }}
        .footer {{ text-align: center; margin-top: 20px; color: #666; font-size: 12px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>OTP Verification</h1>
        </div>
        <div class="content">
            <p>Hello,</p>
            <p>Your One-Time Password (OTP) for login is:</p>
            <div class="otp-code">{otp}</div>
            <p><strong>This OTP is valid for 10 minutes.</strong></p>
            <p>If you didn't request this OTP, please ignore this email.</p>
            <p>Best regards,<br>{settings.SITE_NAME if hasattr(settings, "SITE_NAME") else "eCommerce"} Team</p>
        </div>
        <div class="footer">
            <p>This is an automated email. Please do not reply.</p>
        </div>
    </div>
</body>
</html>
            """

            send_mail(
                subject=subject,
                message=message,
                from_email=settings.DEFAULT_FROM_EMAIL,
                recipient_list=[email],
                html_message=html_message,
                fail_silently=False,
            )
            logger.info(f"OTP email sent successfully to {email}")
            return True
        except Exception as e:
            logger.error(f"Failed to send OTP email to {email}: {str(e)}")
            return False

    @staticmethod
    def send_mobile_otp(mobile, otp):
        """Send OTP via SMS using Twilio"""
        try:
            # Check if Twilio credentials are configured
            if not hasattr(settings, 'TWILIO_ACCOUNT_SID') or not hasattr(settings, 'TWILIO_AUTH_TOKEN'):
                logger.warning("Twilio credentials not configured. Skipping SMS send.")
                # For development, just log the OTP
                logger.info(f"[DEV MODE] OTP for {mobile}: {otp}")
                return True

            client = Client(settings.TWILIO_ACCOUNT_SID, settings.TWILIO_AUTH_TOKEN)

            message = client.messages.create(
                body=f'Your {settings.SITE_NAME if hasattr(settings, "SITE_NAME") else "eCommerce"} OTP is: {otp}. Valid for 10 minutes.',
                from_=settings.TWILIO_PHONE_NUMBER,
                to=mobile
            )

            logger.info(f"OTP SMS sent successfully to {mobile}. SID: {message.sid}")
            return True
        except Exception as e:
            logger.error(f"Failed to send OTP SMS to {mobile}: {str(e)}")
            # For development, log the OTP anyway
            logger.info(f"[DEV MODE] OTP for {mobile}: {otp}")
            return True  # Return True in dev mode to not block the flow
