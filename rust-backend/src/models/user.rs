use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::schema::{users, profiles, bank_accounts, countries};

// ==================== User Model ====================
// Django: auth.User → Rust: User struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub date_joined: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
}

// ==================== Profile Model ====================
// Django: accounts.Profile → Rust: Profile struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country_id: Option<i32>,
    pub postal_code: Option<String>,
    pub avatar: Option<String>,
    pub is_vendor: bool,
    pub vendor_admission: bool,
    pub wallet_balance: Decimal,
    pub referral_code: Option<String>,
    pub referred_by_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub user_id: i32,
    pub phone: Option<String>,
    pub referral_code: Option<String>,
    pub referred_by_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = profiles)]
pub struct UpdateProfile {
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country_id: Option<i32>,
    pub postal_code: Option<String>,
    pub avatar: Option<String>,
    pub is_vendor: Option<bool>,
    pub vendor_admission: Option<bool>,
    pub wallet_balance: Option<Decimal>,
}

// ==================== Bank Account Model ====================
// Django: accounts.BankAccount → Rust: BankAccount struct

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Profile))]
#[diesel(table_name = bank_accounts)]
pub struct BankAccount {
    pub id: i32,
    pub profile_id: i32,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = bank_accounts)]
pub struct NewBankAccount {
    pub profile_id: i32,
    pub bank_name: String,
    pub account_name: String,
    pub account_number: String,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
    pub is_default: bool,
}

// ==================== Country Model ====================
// Django: orders.Country → Rust: Country struct

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = countries)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub currency_code: String,
    pub currency_symbol: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = countries)]
pub struct NewCountry {
    pub name: String,
    pub code: String,
    pub currency_code: String,
    pub currency_symbol: String,
}
