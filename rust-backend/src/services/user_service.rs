use diesel::prelude::*;

use crate::db::DbConnection;
use crate::models::*;
use crate::schema::*;
use crate::utils::errors::{AppError, AppResult};

pub struct UserService;

impl UserService {
    /// Update user profile
    /// Django: accounts/views.py - profile_view
    pub fn update_profile(
        conn: &mut DbConnection,
        user_id: i32,
        user_updates: UpdateUser,
        profile_updates: UpdateProfile,
    ) -> AppResult<(User, Profile)> {
        // Update user
        diesel::update(users::table.find(user_id))
            .set(&user_updates)
            .execute(conn)?;

        // Update profile
        diesel::update(profiles::table.filter(profiles::user_id.eq(user_id)))
            .set(&profile_updates)
            .execute(conn)?;

        let user = users::table.find(user_id).first::<User>(conn)?;
        let profile = profiles::table
            .filter(profiles::user_id.eq(user_id))
            .first::<Profile>(conn)?;

        Ok((user, profile))
    }

    /// Get vendor products
    /// Django: supplier_panel/views.py - supplier_products
    pub fn get_vendor_products(conn: &mut DbConnection, vendor_id: i32) -> AppResult<Vec<Product>> {
        products::table
            .filter(products::vendor_id.eq(vendor_id))
            .order(products::created_at.desc())
            .load::<Product>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Apply for vendor status
    /// Django: accounts/views.py - apply_vendor_view
    pub fn apply_for_vendor(conn: &mut DbConnection, user_id: i32) -> AppResult<Profile> {
        diesel::update(profiles::table.filter(profiles::user_id.eq(user_id)))
            .set(profiles::is_vendor.eq(true))
            .execute(conn)?;

        profiles::table
            .filter(profiles::user_id.eq(user_id))
            .first::<Profile>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get bank accounts
    pub fn get_bank_accounts(conn: &mut DbConnection, profile_id: i32) -> AppResult<Vec<BankAccount>> {
        bank_accounts::table
            .filter(bank_accounts::profile_id.eq(profile_id))
            .load::<BankAccount>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Add bank account
    pub fn add_bank_account(
        conn: &mut DbConnection,
        profile_id: i32,
        bank_name: &str,
        account_name: &str,
        account_number: &str,
        routing_number: Option<String>,
        swift_code: Option<String>,
        is_default: bool,
    ) -> AppResult<BankAccount> {
        // If this is default, unset other defaults
        if is_default {
            diesel::update(bank_accounts::table.filter(bank_accounts::profile_id.eq(profile_id)))
                .set(bank_accounts::is_default.eq(false))
                .execute(conn)?;
        }

        let new_account = NewBankAccount {
            profile_id,
            bank_name: bank_name.to_string(),
            account_name: account_name.to_string(),
            account_number: account_number.to_string(),
            routing_number,
            swift_code,
            is_default,
        };

        diesel::insert_into(bank_accounts::table)
            .values(&new_account)
            .execute(conn)?;

        bank_accounts::table
            .order(bank_accounts::id.desc())
            .first::<BankAccount>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get vendor payment history
    /// Django: supplier_panel/views.py - supplier_payments
    pub fn get_vendor_payments(
        conn: &mut DbConnection,
        vendor_id: i32,
    ) -> AppResult<Vec<VendorPayment>> {
        vendor_payments::table
            .filter(vendor_payments::vendor_id.eq(vendor_id))
            .order(vendor_payments::created_at.desc())
            .load::<VendorPayment>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Request vendor payout
    /// Django: supplier_panel/views.py - supplier_request_payment
    pub fn request_payout(
        conn: &mut DbConnection,
        vendor_id: i32,
        amount: rust_decimal::Decimal,
        bank_account_id: i32,
        notes: Option<String>,
    ) -> AppResult<VendorPayment> {
        // Verify vendor has enough balance
        let profile = profiles::table
            .find(vendor_id)
            .first::<Profile>(conn)?;

        if profile.wallet_balance < amount {
            return Err(AppError::Validation("Insufficient wallet balance".to_string()));
        }

        // Verify bank account belongs to vendor
        let bank_account = bank_accounts::table
            .find(bank_account_id)
            .first::<BankAccount>(conn)?;

        if bank_account.profile_id != vendor_id {
            return Err(AppError::Forbidden("Invalid bank account".to_string()));
        }

        // Deduct from wallet
        diesel::update(profiles::table.find(vendor_id))
            .set(profiles::wallet_balance.eq(profiles::wallet_balance - amount))
            .execute(conn)?;

        // Create payment request
        let new_payment = NewVendorPayment {
            vendor_id,
            order_supplier_id: None,
            amount,
            payment_type: "payout".to_string(),
            status: "pending".to_string(),
            reference_number: None,
            notes,
        };

        diesel::insert_into(vendor_payments::table)
            .values(&new_payment)
            .execute(conn)?;

        vendor_payments::table
            .order(vendor_payments::id.desc())
            .first::<VendorPayment>(conn)
            .map_err(|e| AppError::Database(e))
    }

    /// Get all countries
    pub fn get_countries(conn: &mut DbConnection) -> AppResult<Vec<Country>> {
        countries::table
            .load::<Country>(conn)
            .map_err(|e| AppError::Database(e))
    }
}
