import { useState } from 'react';
import { useMutation, gql } from '@apollo/client';

const REQUEST_EMAIL_OTP = gql`
  mutation RequestEmailOtp($email: String!) {
    requestEmailOtp(email: $email) {
      status
      message
      tempToken
    }
  }
`;

const VERIFY_EMAIL_OTP = gql`
  mutation VerifyEmailOtp($email: String!, $otp: String!) {
    verifyEmailOtp(email: $email, otp: $otp) {
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
`;

interface OTPLoginProps {
  onLogin?: (tokens: { access: string; refresh: string; user: any }) => void;
  onNavigate?: (path: string) => void;
}

export function OTPLogin({ onLogin, onNavigate }: OTPLoginProps) {
  const [email, setEmail] = useState('');
  const [otp, setOtp] = useState('');
  const [step, setStep] = useState<'email' | 'otp'>('email');
  const [tempToken, setTempToken] = useState('');

  const [requestOtp, { loading: requesting }] = useMutation(REQUEST_EMAIL_OTP);
  const [verifyOtp, { loading: verifying }] = useMutation(VERIFY_EMAIL_OTP);

  const handleRequestOtp = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      const { data } = await requestOtp({
        variables: { email }
      });

      if (data?.requestEmailOtp) {
        setTempToken(data.requestEmailOtp.tempToken);
        setStep('otp');
        alert(data.requestEmailOtp.message);
      }
    } catch (error: any) {
      alert('Error: ' + (error.message || 'Failed to send OTP'));
    }
  };

  const handleVerifyOtp = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      const { data } = await verifyOtp({
        variables: { email, otp }
      });

      if (data?.verifyEmailOtp) {
        // Store tokens
        localStorage.setItem('access_token', data.verifyEmailOtp.access);
        localStorage.setItem('refresh_token', data.verifyEmailOtp.refresh);
        localStorage.setItem('user', JSON.stringify(data.verifyEmailOtp.user));

        // Call onLogin callback
        if (onLogin) {
          onLogin(data.verifyEmailOtp);
        }

        alert('Login successful!');
        onNavigate?.('/');
      }
    } catch (error: any) {
      alert('Error: ' + (error.message || 'Invalid OTP'));
    }
  };

  return (
    <div className="page-content pt-150 pb-150">
      <div className="container">
        <div className="row">
          <div className="col-xl-6 col-lg-8 col-md-12 m-auto">
            <div className="row">
              <div className="col-lg-10 col-md-12 m-auto">
                <div className="login_wrap widget-taber-content background-white">
                  <div className="padding_eight_all bg-white">
                    <div className="heading_s1 text-center mb-30">
                      <h1 className="mb-5">Login with OTP</h1>
                      <p className="mb-30">
                        No password required! We'll send you a one-time code.
                      </p>
                    </div>

                    {step === 'email' ? (
                      <form onSubmit={handleRequestOtp}>
                        <div className="form-group">
                          <input
                            type="email"
                            required
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            placeholder="Enter your email"
                            className="form-control"
                          />
                        </div>
                        <div className="form-group mb-30">
                          <button
                            type="submit"
                            disabled={requesting}
                            className="btn btn-fill-out btn-block hover-up font-weight-bold"
                          >
                            {requesting ? 'Sending...' : 'Send OTP'}
                          </button>
                        </div>
                      </form>
                    ) : (
                      <form onSubmit={handleVerifyOtp}>
                        <div className="form-group">
                          <p className="mb-10">
                            We've sent a 6-digit code to <strong>{email}</strong>
                          </p>
                          <input
                            type="text"
                            required
                            value={otp}
                            onChange={(e) => setOtp(e.target.value)}
                            placeholder="Enter 6-digit OTP"
                            maxLength={6}
                            className="form-control"
                          />
                        </div>
                        <div className="form-group mb-30">
                          <button
                            type="submit"
                            disabled={verifying}
                            className="btn btn-fill-out btn-block hover-up font-weight-bold"
                          >
                            {verifying ? 'Verifying...' : 'Verify OTP'}
                          </button>
                        </div>
                        <div className="text-muted text-center">
                          <button
                            type="button"
                            onClick={() => setStep('email')}
                            className="btn btn-link"
                          >
                            Change email address
                          </button>
                        </div>
                      </form>
                    )}

                    <div className="divider-text-center mt-15 mb-15">
                      <span> or </span>
                    </div>
                    <p className="text-muted text-center">
                      Having trouble? Contact support
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
