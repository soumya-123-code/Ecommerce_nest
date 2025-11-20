use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub stripe: StripeConfig,
    pub razorpay: RazorpayConfig,
    pub email: EmailConfig,
    pub referral_commission_percent: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripeConfig {
    pub secret_key: String,
    pub webhook_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RazorpayConfig {
    pub key_id: String,
    pub key_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            server: ServerConfig {
                host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")?,
                expiration: env::var("JWT_EXPIRATION")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .unwrap_or(86400),
            },
            stripe: StripeConfig {
                secret_key: env::var("STRIPE_SECRET_KEY").unwrap_or_default(),
                webhook_secret: env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),
            },
            razorpay: RazorpayConfig {
                key_id: env::var("RAZORPAY_KEY_ID").unwrap_or_default(),
                key_secret: env::var("RAZORPAY_KEY_SECRET").unwrap_or_default(),
            },
            email: EmailConfig {
                smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
                smtp_port: env::var("SMTP_PORT")
                    .unwrap_or_else(|_| "587".to_string())
                    .parse()
                    .unwrap_or(587),
                smtp_user: env::var("SMTP_USER").unwrap_or_default(),
                smtp_password: env::var("SMTP_PASSWORD").unwrap_or_default(),
            },
            referral_commission_percent: env::var("REFERRAL_COMMISSION_PERCENT")
                .unwrap_or_else(|_| "2.5".to_string())
                .parse()
                .unwrap_or(2.5),
        })
    }
}
